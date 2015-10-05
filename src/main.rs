extern crate camo;

use camo::{Config, Camo};

fn main() {
  let config = Config { port: 3333 };
  Camo::new(config).start();
}
