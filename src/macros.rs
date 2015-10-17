macro_rules! try_return(
  ($e:expr) => {{
    match $e {
      Ok(_)  => { return; },
      Err(e) => { println!("Error: {}", e); return; }
    }
  }}
);

macro_rules! use_header_if_present(
  ($given_headers:expr, $headers:expr, $cookie:expr) => {{
    let _tmp = $given_headers.get_raw($cookie);
    if _tmp.is_some() {
      $headers.set_raw($cookie, _tmp.unwrap().to_vec());
    }
  }}
);
