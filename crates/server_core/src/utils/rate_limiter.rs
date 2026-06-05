use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<HashMap<i32, VecDeque<Instant>>>>,
    max_requests: u64,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u64, window_seconds: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_seconds),
        }
    }

    /// Returns `true` if the request is allowed, `false` if rate limited.
    pub async fn check(&self, user_id: i32) -> bool {
        let mut map = self.inner.lock().await;
        let now = Instant::now();
        let deque = map.entry(user_id).or_default();

        // Remove expired entries outside the window
        while let Some(&t) = deque.front() {
            if now.duration_since(t) > self.window {
                deque.pop_front();
            } else {
                break;
            }
        }

        if deque.len() as u64 >= self.max_requests {
            return false;
        }

        deque.push_back(now);
        true
    }
}
