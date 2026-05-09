use crate::event_log::{append_event, read_all_events, clear_log, Event};
use crate::reducer::{idempotent_reduce, ReducerRegistry, core_reducer};
use crate::hash::hash_state;
use crate::snapshot::{save_snapshot, load_snapshot};
use crate::state::AppState;
use crate::journal::DeliveryJournal;
use std::fs;

pub fn run_stress_test(event_count: u64, seed: u64) -> bool {
    clean_env();
    let mut registry = ReducerRegistry::new();
    registry.register("core", core_reducer);

    let events = generate_deterministic_events(event_count, seed);

    // ----- Live Mode -----
    let mut live_state = AppState::default();
    let mut journal = DeliveryJournal::new();
    for event in &events {
        append_event(event).expect("stress: append failed");
        live_state = idempotent_reduce(live_state, event, &mut journal);
    }
    let live_hash = hash_state(&live_state);

    let last_tick = events.last().map(|e| e.tick).unwrap_or(0);
    save_snapshot(&live_state, last_tick, events.len()).expect("stress: snapshot failed");

    // ----- Replay Mode -----
    let all_events = read_all_events().expect("stress: read failed");
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

    live_hash == replay_hash
}

fn clean_env() {
    let _ = clear_log();
    let _ = fs::remove_file("snapshot.json");
}

/// يولّد أحداثًا حتمية تضمن Register قبل أي Transition.
fn generate_deterministic_events(count: u64, seed: u64) -> Vec<Event> {
    let mut events = Vec::new();
    let mut wf_counter = 1u64;

    for i in 0..count {
        let id = seed * 1000 + i;
        let tick = i + 1;
        let payload;

        // كل 200 حدث: سجّل Workflow جديدًا
        if i % 200 == 0 {
            let wf_id = wf_counter;
            wf_counter += 1;
            payload = format!(
                wf_id, wf_id
            );
        }
        // كل 50 حدث (بعد أول 200): أرسل Transition على آخر Workflow مُسجّل
        else if i >= 200 && i % 50 == 0 {
            let wf_id = wf_counter - 1; // آخر مُسجّل
            let new_state = if (i / 50) % 2 == 0 { "Ready" } else { "Running" };
            payload = format!(
                wf_id, new_state
            );
        }
        // كل 13 حدث: أرسل أمرًا جانبيًا
        else if i % 13 == 0 {
            payload = "EXEC:spawn_subprocess".to_string();
        }
        else {
            payload = format!("stress_event_{}", i);
        }
    }
    events
}
