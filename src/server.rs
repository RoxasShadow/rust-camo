extern crate hyper;

use hyper::server::Handler as HTTPHandler;
use std::net::{SocketAddrV4, Ipv4Addr};
use hyper::server::{Request, Response};
use hyper::server::Server as HTTPServer;
use hyper::net::Fresh;
use ::Config;
use ::Camo;

pub struct Server {
  config: Config
}

pub trait Handler: HTTPHandler {
  fn handle<'a, 'k>(&'a self, Camo, Request<'a, 'k>, Response<'a, Fresh>);
}

impl<F> Handler for F where F: Fn(Camo, Request, Response<Fresh>), F: Handler {
  fn handle<'a, 'k>(&'a self, camo: Camo, req: Request<'a, 'k>, res: Response<'a, Fresh>) {
    self(camo, req, res)
  }
}

impl Server {
  pub fn new(config: Config) -> Server {
    return Server { config: config };
  }

  pub fn start<H: Handler + 'static>(&self, h: H) {
    let ip   = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(ip, self.config.port);

    let server = HTTPServer::http(addr).unwrap();
    let _guard = server.handle(h);
    println!("Listening on {}", addr);
  }
}
