use std::sync::RwLock;
use crate::state::AppState;
use crate::event::Event;
use crate::event_log;

pub struct StateStore {
    state: RwLock<AppState>,
}

impl StateStore {
    pub fn new() -> Self {
        Self { state: RwLock::new(AppState::default()) }
    }

    pub fn read_state(&self) -> AppState {
        self.state.read().unwrap().clone()
    }

    pub fn apply_event(&self, event: &Event) {
        let mut state = self.state.write().unwrap();
        state.apply_event(event);
        let _ = event_log::append_event(event);
    }

    pub fn rebuild_from_log(&self) {
        if let Ok(events) = event_log::read_all_events() {
            let mut new_state = AppState::default();
            for event in &events {
                new_state.apply_event(event);
            }
            let mut state = self.state.write().unwrap();
            *state = new_state;
        }
    }
}
