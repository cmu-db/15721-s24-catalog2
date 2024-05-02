use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
  let current_time = SystemTime::now();
  current_time
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs()
}
