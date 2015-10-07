#![feature(slice_patterns, convert, plugin)]

extern crate regex;
extern crate rustc_serialize;
extern crate hyper;
extern crate cookie;
extern crate time;

#[macro_use]
pub mod macros;

pub mod config;
pub mod status;
pub mod camo;

pub use config::Config;
pub use status::Status;
pub use camo::Camo;
