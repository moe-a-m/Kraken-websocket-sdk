use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    requests: Vec<Instant>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: Vec::new(),
        }
    }

    pub async fn acquire(&mut self) {
        let now = Instant::now();

        // Remove old requests outside the window
        self.requests
            .retain(|&time| now.duration_since(time) < self.window);

        // If at limit, wait
        if self.requests.len() >= self.max_requests as usize {
            if let Some(&oldest) = self.requests.first() {
                let wait_time = self.window - now.duration_since(oldest);
                sleep(wait_time).await;
            }
        }

        self.requests.push(now);
    }
}
