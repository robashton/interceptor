use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

/// MockTime is a helper to replace SystemTime::now() for testing purposes.
pub struct MockTime {
    cur_now: Mutex<SystemTime>,
}

impl Default for MockTime {
    fn default() -> Self {
        MockTime {
            cur_now: Mutex::new(SystemTime::UNIX_EPOCH),
        }
    }
}

impl MockTime {
    /// set_now sets the current time.
    pub async fn set_now(&self, now: SystemTime) {
        let mut cur_now = self.cur_now.lock().await;
        *cur_now = now;
    }

    /// now returns the current time.
    pub async fn now(&self) -> SystemTime {
        let cur_now = self.cur_now.lock().await;
        *cur_now
    }

    /// advance advances duration d
    pub async fn advance(&mut self, d: Duration) {
        let mut cur_now = self.cur_now.lock().await;
        *cur_now = cur_now.checked_add(d).unwrap_or(*cur_now);
    }
}
