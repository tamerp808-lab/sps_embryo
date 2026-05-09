use std::time::Instant;

pub struct LatencyMetrics {
    start: Instant,
}

impl LatencyMetrics {
    pub fn begin() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    pub fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
}
