use sps_embryo::event::Event;
use sps_embryo::journal::DeliveryJournal;
use sps_embryo::reducer::{core_reducer, ReducerRegistry};
use sps_embryo::state::AppState;

#[test]
fn test_deterministic_reducer() {
    let mut registry = ReducerRegistry::new();
    registry.register("core", core_reducer);
    let event = Event {
        id: 1,
        tick: 1,
        vector_clock: String::new(),
        event_type: "Test".to_string(),
        payload: "test_payload".to_string(),
        parent_id: None,
    };
    let state1 = core_reducer(AppState::default(), &event);
    let state2 = core_reducer(AppState::default(), &event);
    assert_eq!(state1.counter, state2.counter);
}

#[test]
fn test_idempotent_reducer() {
    let event = Event {
        id: 1,
        tick: 1,
        vector_clock: String::new(),
        event_type: "Test".to_string(),
        payload: "test".to_string(),
        parent_id: None,
    };
    let mut journal = DeliveryJournal::new();
    let state1 = sps_embryo::reducer::idempotent_reduce(AppState::default(), &event, &mut journal);
    let state2 = sps_embryo::reducer::idempotent_reduce(state1.clone(), &event, &mut journal);
    assert_eq!(state1.counter, state2.counter);
}
