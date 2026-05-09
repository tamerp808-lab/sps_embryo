use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    requests: Mutex<HashMap<String, Vec<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Mutex::new(HashMap::new()),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    /// يُرجع true إذا كان الطلب مسموحًا
    pub fn allow(&self, ip: &str) -> bool {
        let mut map = self.requests.lock().unwrap();
        let now = Instant::now();
        let entries = map.entry(ip.to_string()).or_default();
        // إزالة الطلبات القديمة
        entries.retain(|t| now.duration_since(*t) < self.window);
        if entries.len() >= self.max_requests {
            return false;
        }
        entries.push(now);
        true
    }
}
