extern crate camo;

use camo::{Config, Camo};

fn main() {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  let config = Config {
    version:    VERSION,
    port:       3333,
    user_agent: format!("rust-camo Asset Proxy #{}", VERSION),
    shared_key: String::from("0x24FEEDFACEDEADBEEFCAFE")
  };

  Camo::serve(config).unwrap();
}
