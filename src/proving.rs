use crate::event_log::{append_event, read_all_events, clear_log, Event};
use crate::reducer::{idempotent_reduce, ReducerRegistry, core_reducer};
use crate::hash::hash_state;
use crate::snapshot::{save_snapshot, load_snapshot};
use crate::state::AppState;
use crate::journal::DeliveryJournal;
use crate::execution::ExecutionRuntime;
use crate::outbox::EffectOutbox;
use std::fs;

/// نتيجة إثبات واحد
pub struct ProofResult {
    pub scenario: String,
    pub live_hash: String,
    pub replay_hash: String,
    pub passed: bool,
}

/// ينظف البيئة ويعيدها جديدة
fn clean_env() {
    let _ = clear_log();
    let _ = fs::remove_file("snapshot.json");
}

/// يُجري إثباتًا: Live → Replay ويُقارن التجزئات
pub fn run_proof(scenario: &str, events: Vec<Event>) -> ProofResult {
    clean_env();
    let mut registry = ReducerRegistry::new();
    registry.register("core", core_reducer);
    let mut live_state = AppState::default();
    let mut journal = DeliveryJournal::new();
    let mut outbox = EffectOutbox::new();

    // Live Mode
    for event in &events {
        append_event(event).expect("Proof: append failed");
        live_state = idempotent_reduce(live_state, event, &mut journal);
        ExecutionRuntime::process_live_event(event.id, event.tick, &event.payload, &mut outbox);
    }
    // تنفيذ التأثيرات (لا يؤثر على state هنا)
    ExecutionRuntime::run_effects(&mut outbox);
    let live_hash = hash_state(&live_state);

    // Snapshot
    let last_tick = events.last().map(|e| e.tick).unwrap_or(0);
    save_snapshot(&live_state, last_tick, events.len()).expect("Proof: snapshot failed");

    // Replay Mode
    let all_events = read_all_events().expect("Proof: read failed");
    let snapshot = load_snapshot().ok().flatten();
    let (mut replay_state, start_idx) = if let Some(snap) = snapshot {
        (snap.state, snap.event_count)
    } else {
        (AppState::default(), 0)
    };
    let mut replay_journal = DeliveryJournal::new();
    for event in all_events.iter().skip(start_idx) {
        replay_state = idempotent_reduce(replay_state, event, &mut replay_journal);
    }
    let replay_hash = hash_state(&replay_state);

    ProofResult {
        scenario: scenario.to_string(),
        live_hash: live_hash.clone(),
        replay_hash,
        passed: live_hash == hash_state(&replay_state),
    }
}
