use crate::event_log::{append_event, read_all_events, clear_log, Event};
use crate::reducer::reduce;
use crate::hash::hash_state;
use crate::snapshot::{save_snapshot, load_snapshot};
use crate::state::AppState;
use std::fs;

// =====================================================
// نتيجة اختبار واحد
// =====================================================

pub struct SimulationResult {
    pub test_type: String,
    pub passed: bool,
    pub live_hash: String,
    pub replay_hash: String,
    pub note: String,
}

// =====================================================
// توليد أحداث حتمية من بذرة
// =====================================================

fn generate_deterministic_events(seed: u64) -> Vec<Event> {
    let num_events = 3 + (seed % 3);
    let mut events = Vec::new();
    for i in 0..num_events {
        let tick = i + 1;
            id: seed * 100 + i,
            tick,
            stream_id: 1,
            payload: format!("sim_event_{}", tick),
        });
    }
    events
}

// =====================================================
// اختبار التطابق المباشر (Live == Replay)
// =====================================================

pub fn test_equivalence(seed: u64) -> SimulationResult {
    let _ = clear_log();
    let _ = fs::remove_file("snapshot.json");

    let events = generate_deterministic_events(seed);

    // Live
    let mut live_state = AppState::default();
    for event in &events {
        append_event(event).expect("append failed");
        live_state = reduce(live_state, event);
    }
    let live_hash = hash_state(&live_state);
    save_snapshot(&live_state, events.last().map(|e| e.tick).unwrap_or(0), events.len())
        .expect("snapshot failed");

    // Replay (بدون أي فقدان)
    let all_events = read_all_events().expect("read failed");
    let snapshot = load_snapshot().ok().flatten();
    let (mut state, start_idx) = if let Some(snap) = snapshot {
        (snap.state, snap.event_count)
    } else {
        (AppState::default(), 0)
    };
    let mut processed = 0;
    for event in all_events.iter().skip(start_idx) {
        state = reduce(state, event);
        processed += 1;
    }

    SimulationResult {
        test_type: "Equivalence".into(),
        passed: live_hash == hash_state(&state),
        live_hash,
        replay_hash: hash_state(&state),
        note: format!("processed {} events", processed),
    }
}

// =====================================================
// اختبار حتمية التعافي من الانهيار
// =====================================================

pub fn test_crash_recovery_determinism(seed: u64) -> SimulationResult {
    // التشغيل الأول: Live ثم Crash ثم Replay
    let hash_first = run_crash_scenario(seed);

    // التشغيل الثاني: نكرر نفس السيناريو ببيئة نظيفة
    let hash_second = run_crash_scenario(seed);

    SimulationResult {
        test_type: "Crash Recovery Determinism".into(),
        passed: hash_first == hash_second,
        live_hash: hash_first.clone(),
        replay_hash: hash_second,
        note: "replay after crash must be deterministic".into(),
    }
}

/// يُنفذ سيناريو Live → Crash → Replay ويُعيد تجزئة حالة الإعادة.
fn run_crash_scenario(seed: u64) -> String {
    // ضمان بيئة نظيفة جدًا
    let _ = clear_log();
    let _ = fs::remove_file("snapshot.json");

    let events = generate_deterministic_events(seed);

    // Live كامل
    let mut live_state = AppState::default();
    for event in &events {
        append_event(event).expect("append failed");
        live_state = reduce(live_state, event);
    }
    // حفظ لقطة بعد آخر حدث
    save_snapshot(&live_state, events.last().map(|e| e.tick).unwrap_or(0), events.len())
        .expect("snapshot failed");

    // محاكاة انهيار: حذف آخر حدث من السجل
    let mut all_events = read_all_events().expect("read failed");
    if all_events.len() > 1 {
        all_events.pop();
    }
    // إعادة كتابة السجل بدون الحدث الأخير
    let _ = clear_log();
    for e in &all_events {
        append_event(e).expect("rewrite failed");
    }
    // اللقطة لا تُحدَّث، تبقى تشير إلى العدد الأصلي للأحداث (سيتم تخطي بعضها أثناء الإعادة)

    // Replay بعد الانهيار
    let snapshot = load_snapshot().ok().flatten();
    let (mut state, start_idx) = if let Some(snap) = snapshot {
        (snap.state, snap.event_count)
    } else {
        (AppState::default(), 0)
    };
    let current_events = read_all_events().expect("read after crash failed");
    for event in current_events.iter().skip(start_idx) {
        state = reduce(state, event);
    }

    hash_state(&state)
}
