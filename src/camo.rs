use ::Config;
use ::Status;

use std::net::{SocketAddrV4, Ipv4Addr};
use std::cell::RefCell;
use std::sync::Mutex;

use time;
use hyper::Get;
use hyper::header::{Headers, Cookie, SetCookie};
use cookie::Cookie as CookiePair;
use hyper::server::Handler;
use hyper::server::{Request, Response};
use hyper::server::Server;
use hyper::uri::RequestUri::AbsolutePath;

pub struct Camo {
  config: Config,
  status: Status
}

impl Handler for Camo {
  fn handle(&self, mut req: Request, mut res: Response) {
    self.default_security_headers(&mut res);

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
  pub fn serve(config: Config) {
    let ip   = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(ip, config.port);

    let status = Status {
      current_connections: Mutex::new(RefCell::new(0)),
      total_connections:   Mutex::new(RefCell::new(0)),
      started_at:          time::now_utc()
    };

    let camo = Camo {
      config: config,
      status: status
    };

    println!("Listening on {}", addr);
    Server::http(addr).unwrap().handle(camo).unwrap();
  }

  fn clear_cookies(&self, headers: &mut Headers, cookies: Option<&Cookie>) {
    match cookies {
      Some(cookies) => {
        for cookie in &mut cookies.iter() {
          let mut cookie = CookiePair::new(cookie.name.clone(), "".to_owned());
          cookie.expires = Some(time::empty_tm());
          headers.set(SetCookie(vec![cookie]));
        }
      },

      None => {}
    }
  }

  fn camo(&self, req: &Request, mut res: Response) {
    {
      let headers: &mut Headers    = res.headers_mut();
      let cookies: Option<&Cookie> = req.headers.get();
      self.clear_cookies(headers, cookies);
    }

    self.status.new_visitor();

    try_return!(res.send(b"ok"));
  }

  fn default_security_headers(&self, res: &mut Response) {
    let headers : &mut Headers = res.headers_mut();
    headers.set_raw("X-Frame-Options", vec![b"deny".to_vec()]);
    headers.set_raw("X-XSS-Protection", vec![b"1; mode=block".to_vec()]);
    headers.set_raw("X-Content-Type-Options", vec![b"nosniff".to_vec()]);
    headers.set_raw("Content-Security-Policy", vec![b"default-src 'none'; img-src data:; style-src 'unsafe-inline'".to_vec()]);
    headers.set_raw("Strict-Transport-Security", vec![b"max-age=31536000; includeSubDomains".to_vec()]);
  }
}
