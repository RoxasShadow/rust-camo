#[derive(Debug, Clone)]
pub struct Status {
  pub current_connections: u16,
  pub total_connections:   u16,
  pub started_at:          u16
}

impl Status {
  pub fn to_string(&self) -> &[u8] {
    return b"lolwut";
    // "ok #{current_connections}/#{total_connections} since #{started_at.toString()}"
  }
}
