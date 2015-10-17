use std::io::Cursor;

use libc;
use libc::pid_t;
use regex::Regex;
use time;
use rustc_serialize::hex::FromHex;
use hyper::header::{Headers, Cookie, SetCookie};
use cookie::Cookie as CookiePair;
use std::io::{self, Read};
use hyper::client::Response as ClientResponse;
use byteorder::{BigEndian, ReadBytesExt};

pub struct Utils;

impl Utils {
  pub fn pid() -> pid_t {
    unsafe { libc::getpid() }
  }

  pub fn hexdec(s: &str) -> Option<String> {
    let length = s.len();

    if length > 0 && length % 2 == 0 {
      if !Regex::new(r"[^0-9a-f]").unwrap().is_match(s) {
        return match s.from_hex() {
          Ok(val) => String::from_utf8(val).ok(),
          Err(_)  => None,
        };
      }
    }

    return None;
  }

  pub fn clear_cookies(headers: &mut Headers, cookies: Option<&Cookie>) {
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

  pub fn read_to_string(mut r: ClientResponse) -> io::Result<Vec<u8>> {
    let mut v = Vec::new();
    try!(r.read_to_end(&mut v));
    return Ok(v);
  }

  pub fn bytes_to_int(bytes: &[u8]) -> u32 {
    let mut buf = Cursor::new(&bytes[..]);
    return buf.read_u32::<BigEndian>().unwrap();
  }
}

#[cfg(test)]
mod tests {
  use Utils;

  #[test]
  fn test_hexdec() {
    assert_eq!(Utils::hexdec("687474703a2f2f6578616d706c652e636f6d2f6f63746f6361742e6a7067"), Some(String::from("http://example.com/octocat.jpg")));
    assert_eq!(Utils::hexdec("lolwut"), None);
    assert_eq!(Utils::hexdec(""), None);
  }
}
