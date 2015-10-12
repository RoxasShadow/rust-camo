use libc;
use libc::pid_t;
use regex::Regex;
use time;
use rustc_serialize::hex::FromHex;
use hyper::header::{Headers, Cookie, SetCookie};
use cookie::Cookie as CookiePair;

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
}
