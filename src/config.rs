#[derive(Debug)]
pub struct Config {
  pub version:    &'static str,
  pub port:       u16,
  pub user_agent: String,
  pub shared_key: String
}
