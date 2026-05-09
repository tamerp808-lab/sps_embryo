use std::collections::BTreeMap;
use crate::event::Event;
use crate::state::AppState;
use crate::journal::DeliveryJournal;

pub type ReducerFn = fn(AppState, &Event) -> AppState;

#[derive(Clone)]
pub struct ReducerRegistry {
    reducers: BTreeMap<&'static str, ReducerFn>,
}

impl ReducerRegistry {
    pub fn new() -> Self {
        Self { reducers: BTreeMap::new() }
    }

    pub fn register(&mut self, name: &'static str, f: ReducerFn) {
        self.reducers.insert(name, f);
    }

    pub fn dispatch(&self, name: &str, state: AppState, event: &Event) -> AppState {
        let reducer = self.reducers.get(name)
            .unwrap_or_else(|| panic!("Reducer '{}' not registered", name));
        reducer(state, event)
    }
}

pub fn core_reducer(state: AppState, event: &Event) -> AppState {
    let mut s = state;
    s.apply_event(event);
    s
}

// استعادة الدوال المفقودة
pub fn reduce(state: AppState, event: &Event) -> AppState {
    core_reducer(state, event)
}

pub fn idempotent_reduce(
    state: AppState,
    event: &Event,
    journal: &mut DeliveryJournal,
) -> AppState {
    if journal.already_processed(event.id) {
        return state;
    }
    journal.record(event.id);
    reduce(state, event)
}
