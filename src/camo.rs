use ::Config;
use ::Status;
use ::Utils;

use std::net::{SocketAddrV4, Ipv4Addr};
use std::cell::RefCell;
use std::sync::Mutex;

use time;
use rustc_serialize::hex::ToHex;
use hyper::{Url, Get, Client};
use hyper::client::Response as ClientResponse;
use hyper::header::{Headers, Cookie, Connection};
use hyper::error::Error;
use hyper::server::{Listening, Handler, Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::status::StatusCode;
use crypto::digest::Digest;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use crypto::mac::Mac;

pub struct Camo {
  config: Config,
  status: Status
}

impl Handler for Camo {
  fn handle(&self, mut req: Request, mut res: Response) {
    Camo::set_default_security_headers(&mut res);

    match req.uri.clone() {
      AbsolutePath(path) => match (&req.method, &path[..]) {
        (&Get, "/")            => { try_return!(res.send(b"hwhat")); },
        (&Get, "/favicon.ico") => { try_return!(res.send(b"ok"));    },
        (&Get, "/status")      => { try_return!(res.send(format!("ok {}", self.status.as_string()).as_bytes())) },
        (&Get, _)              => { self.camo(&mut req, res) },
        _ => { try_return!(res.send(b"hwhat")); }
      },
      _ => { try_return!(res.send(b"hwhat")); }
    };
  }
}

impl Camo {
  pub fn serve(config: Config) -> Result<Listening, Error> {
    let ip   = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(ip, config.port);

    let status = Status {
      current_connections: Mutex::new(RefCell::new(0)),
      total_connections:   Mutex::new(RefCell::new(0)),
      started_at:          time::now_utc()
    };

    println!("SSL-Proxy running on {} with pid:{} version:{}",
      config.port, Utils::pid(), config.version);

    let camo = Camo {
      config: config,
      status: status
    };

    return Server::http(addr).unwrap().handle(camo);
  }

  fn from_query_string(pairs: Vec<(String, String)>) -> Option<(String)> {
    return Some(pairs[0].clone().1);
  }

  fn from_encoded_url<'a>(path: &'a [String]) -> Option<String> {
    return if path.len() >= 3 {
      let encoded_url = &*path[2];
      Utils::hexdec(encoded_url)
    }
    else {
      None
    };
  }

  fn process_url(&self, dest_url: String, mut res: &mut Response) -> Option<Vec<u8>> {
    let _url = Url::parse(&*dest_url);
    if _url.is_err() {
      return None;
    }

    let url = _url.unwrap();

    // TODO: newHeaders -> statusCode

    let client_res: ClientResponse = Client::new()
        .get(url)
        .header(Connection::close())
        .send().unwrap();

    return Utils::read_to_string(client_res).ok();
  }

  fn camo(&self, req: &Request, mut res: Response) {
    {
      let headers: &mut Headers    = res.headers_mut();
      let cookies: Option<&Cookie> = req.headers.get();
      Utils::clear_cookies(headers, cookies);
    }

    {
      let url  = Url::parse(&*format!("http://127.0.0.1:{}/{}", self.config.port, req.uri)).unwrap();
      let path = url.path().unwrap();

      let query_digest = path[1].clone();
      let dest_url     = match url.query_pairs() {
        Some(pairs) => Camo::from_query_string(pairs),
        None        => Camo::from_encoded_url(path),
      };

      // TODO: check headers here

      match dest_url {
        Some(url) => {
          let mut hmac = Hmac::new(Sha1::new(), self.config.shared_key.as_bytes());
          hmac.input(url.as_bytes());

          let hmac_digest = hmac.result().code().to_hex();

          if hmac_digest == query_digest {
            self.set_transfer_headers(req.headers.clone(), &mut res);
            match self.process_url(url, &mut res) {
              Some(s) => try_return!(res.send(&*s)),
              None    => {
                *res.status_mut() = StatusCode::NotFound;
                try_return!(res.send(b"No host found"));
              }
            }
          }
          else {
            let s = format!("checksum mismatch {}:{}", hmac_digest, query_digest);
            *res.status_mut() = StatusCode::NotFound;
            try_return!(res.send(s.as_bytes()));
          }
        }
        None => {
          *res.status_mut() = StatusCode::NotFound;
          try_return!(res.send(b"No pathname provided on the server"));
        }
      }
    }

    self.status.new_visitor();
  }

  fn set_transfer_headers(&self, given_headers: Headers, res: &mut Response) {
    let headers: &mut Headers = res.headers_mut();
    headers.set_raw("Via", vec![self.config.user_agent.as_bytes().to_vec()]);
    headers.set_raw("User-Agent", vec![self.config.user_agent.as_bytes().to_vec()]);

    let accept = given_headers.get_raw("Accept");
    headers.set_raw("Accept", accept.unwrap_or(&[vec![b"image/*"[0]]]).to_vec());

    let accept_encoding = given_headers.get_raw("Accept-Encoding");
    headers.set_raw("Accept-Encoding", accept_encoding.unwrap_or(&[vec![b"*"[0]]]).to_vec());
  }

  fn set_default_security_headers(res: &mut Response) {
    let headers: &mut Headers = res.headers_mut();
    headers.set_raw("X-Frame-Options", vec![b"deny".to_vec()]);
    headers.set_raw("X-XSS-Protection", vec![b"1; mode=block".to_vec()]);
    headers.set_raw("X-Content-Type-Options", vec![b"nosniff".to_vec()]);
    headers.set_raw("Content-Security-Policy", vec![b"default-src 'none'; img-src data:; style-src 'unsafe-inline'".to_vec()]);
    headers.set_raw("Strict-Transport-Security", vec![b"max-age=31536000; includeSubDomains".to_vec()]);
  }
}

#[cfg(test)]
mod tests {
  use Camo;

  macro_rules! s(
    ($e:expr) => {{ String::from($e) }}
  );

  #[test]
  fn test_from_query_string() {
    let pairs: Vec<(String, String)> = vec![ (s!("url"), s!("http://example.com/octocat.jpg")) ];
    assert_eq!(Camo::from_query_string(pairs), Some(s!("http://example.com/octocat.jpg")));
  }

  #[test]
  fn test_from_encoded_url() {
    let path = [s!(""), s!("b9f45c9f94e3b15fecae2bf9a8b497fc7280fd29"), s!("687474703a2f2f6578616d706c652e636f6d2f6f63746f6361742e6a7067")];
    assert_eq!(Camo::from_encoded_url(&path), Some(s!("http://example.com/octocat.jpg")));
  }
}
