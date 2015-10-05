macro_rules! try_return(
  ($e:expr) => {{
    match $e {
      Ok(_)  => { return; },
      Err(e) => { println!("Error: {}", e); return; }
    }
  }}
);
