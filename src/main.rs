extern crate camo;

use camo::{Config, Camo};

fn main() {
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  let config = Config {
    port:       3333,
    user_agent: format!("Camoscio Asset Proxy #{}", VERSION)
  };

  Camo::serve(config);
}
