use crate::event_log;
use crate::reducer::reduce;
use crate::state::AppState;
use std::time::Instant;

pub struct ReplayBenchmark;

impl ReplayBenchmark {
    /// يُعيد بناء الحالة من السجل ويقيس الوقت المستغرق
    pub fn run() -> Result<(usize, f64), Box<dyn std::error::Error>> {
        let events = event_log::read_all_events()?;
        let count = events.len();
        if count == 0 {
            return Ok((0, 0.0));
        }

        let start = Instant::now();
        let mut state = AppState::default();
        for event in &events {
            state = reduce(state, event);
        }
        let elapsed = start.elapsed().as_secs_f64();

        Ok((count, elapsed))
    }

    /// طباعة تقرير
    pub fn report() -> String {
        match Self::run() {
            Ok((count, secs)) => {
                if count > 0 {
                    let eps = count as f64 / secs.max(0.001);
                    format!(
                        "Replay benchmark: {} events in {:.3}s ({:.0} events/s)",
                        count, secs, eps
                    )
                } else {
                    "No events to replay.".to_string()
                }
            }
            Err(e) => format!("Benchmark failed: {}", e),
        }
    }
}
