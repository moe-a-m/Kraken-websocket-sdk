use instant::Instant;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub msg_count: Arc<AtomicU64>,
    pub start_time: Instant,
    pub last_msg_time: Arc<AtomicU64>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            msg_count: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
            last_msg_time: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl Metrics {
    pub fn record_message(&self) {
        self.msg_count.fetch_add(1, Ordering::Relaxed);
        let now = self.start_time.elapsed().as_millis() as u64;
        self.last_msg_time.store(now, Ordering::Relaxed);
    }

    pub fn msg_per_sec(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.msg_count.load(Ordering::Relaxed) as f64 / elapsed
        } else {
            0.0
        }
    }

    pub fn latency_ms(&self) -> u64 {
        let now = self.start_time.elapsed().as_millis() as u64;
        let last = self.last_msg_time.load(Ordering::Relaxed);
        now.saturating_sub(last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_default() {
        let metrics = Metrics::default();
        assert_eq!(metrics.msg_count.load(Ordering::Relaxed), 0);
        assert_eq!(metrics.msg_per_sec(), 0.0);
    }

    #[test]
    fn test_record_message() {
        let metrics = Metrics::default();
        metrics.record_message();
        metrics.record_message();
        assert_eq!(metrics.msg_count.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_msg_per_sec() {
        let metrics = Metrics::default();
        metrics.record_message();
        thread::sleep(Duration::from_millis(100));
        let rate = metrics.msg_per_sec();
        assert!(rate > 0.0);
    }
}
