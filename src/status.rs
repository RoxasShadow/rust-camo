use std::cell::RefCell;
use std::sync::Mutex;

use time::Tm;

#[derive(Debug)]
pub struct Status {
  pub current_connections: Mutex<RefCell<u16>>,
  pub total_connections:   Mutex<RefCell<u16>>,
  pub started_at:          Tm
}

impl Status {
  pub fn as_string(&self) -> String {
    let current_connections = self.current_connections.lock().unwrap();
    let total_connections   = self.total_connections.lock().unwrap();

    return format!("{}/{} since {}",
      *current_connections.borrow(),
      *total_connections.borrow(),
      self.started_at.asctime()
    );
  }

  pub fn new_visitor(&self) {
    let current_connections = self.current_connections.lock().unwrap();
    *current_connections.borrow_mut() += 1;

    let total_connections = self.total_connections.lock().unwrap();
    *total_connections.borrow_mut() += 1;
  }
}
