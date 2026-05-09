use crate::event::Event;
use crate::event_log::read_all_events;
use crate::reducer::reduce;
use crate::state::AppState;

/// محرك الإعادة الرسمي
pub struct ReplayEngine {
    events: Vec<Event>,
}

impl ReplayEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let events = read_all_events()?;
        Ok(Self { events })
    }

    /// يعيد بناء الحالة كاملة من الأحداث
    pub fn replay_full(&self) -> (AppState, usize) {
        let mut state = AppState::default();
        let mut processed = 0;
        for event in &self.events {
            state = reduce(state, event);
            processed += 1;
        }
        (state, processed)
    }

    /// التحقق من صحة التطابق (يُستخدم في مرحلة الإقلاع)
    pub fn validate_determinism(&self, live_hash: &str) -> bool {
        let (replayed_state, _) = self.replay_full();
        let replayed_hash = crate::hash::hash_state(&replayed_state);
        live_hash == &replayed_hash
    }

    pub fn stream_count(&self) -> usize {
        self.events.len()
    }
}
