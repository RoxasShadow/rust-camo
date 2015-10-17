#[derive(Debug)]
pub struct Config {
  pub version:              &'static str,
  pub port:                 u16,
  pub user_agent:           String,
  pub shared_key:           String,
  pub hostname:             String,
  pub timing_allow_origin:  Option<String>,
  pub content_length_limit: u32,
}
