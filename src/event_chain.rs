use crate::event_log::read_all_events;
use crate::hash::hash_state;
use crate::journal::DeliveryJournal;
use crate::reducer::idempotent_reduce;
use crate::state::AppState;

pub fn compute_event_chain_hash() -> String {
    let events = read_all_events().unwrap_or_default();
    if events.is_empty() {
        return String::new();
    }
    let mut state = AppState::default();
    let mut journal = DeliveryJournal::new();
    for event in &events {
        state = idempotent_reduce(state, event, &mut journal);
    }
    hash_state(&state)
}
