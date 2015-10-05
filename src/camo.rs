use ::Config;
use ::Status;
use ::Server;

use hyper::{Get};
use hyper::header::Headers;
use hyper::server::{Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

pub struct Camo {
  config: Config,
  status: Status
}

macro_rules! try_return(
  ($e:expr) => {{
    match $e {
      Ok(_)  => { return; },
      Err(e) => { println!("Error: {}", e); return; }
    }
  }}
);

impl Camo {
  pub fn new(config: Config) -> Camo {
    let status = Status {
      current_connections: 0,
      total_connections:   0,
      started_at:          0
    };

    return Camo {
      config: config,
      status: status
    };
  }

  pub fn start(&self) {
    let server = Server::new(self.config.clone());
    server.start(Camo::camo);
  }

  pub fn default_security_headers(res: &mut Response) {
    let headers : &mut Headers = res.headers_mut();
    headers.set_raw("X-Frame-Options", vec![b"deny".to_vec()]);
    headers.set_raw("X-XSS-Protection", vec![b"1; mode=block".to_vec()]);
    headers.set_raw("X-Content-Type-Options", vec![b"nosniff".to_vec()]);
    headers.set_raw("Content-Security-Policy", vec![b"default-src 'none'; img-src data:; style-src 'unsafe-inline'".to_vec()]);
    headers.set_raw("Strict-Transport-Security", vec![b"max-age=31536000; includeSubDomains".to_vec()]);
  }

  pub fn camo(camo: Camo, req: Request, mut res: Response) {
    /*Camo::default_security_headers(&mut res);

    match req.uri {
      AbsolutePath(ref path) => match (&req.method, &path[..]) {
        (&Get, "/")            => { try_return!(res.send(b"hwhat"));   },
        (&Get, "/favicon.ico") => { try_return!(res.send(b"ok"));      },
        (&Get, "/status")      => { try_return!(res.send(camo.status.to_string())) },
        _ => return
      },
      _ => { try_return!(res.send(b"hwhat")); }
    };*/
  }
}
