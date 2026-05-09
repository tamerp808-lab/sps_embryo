use crate::event::Event;
use crate::hash::hash_state;
use crate::reducer::reduce;
use crate::state::AppState;
use std::collections::HashMap;

pub struct EventChainVerifier;

impl EventChainVerifier {
    /// يتحقق من أن سلسلة الأحداث سليمة (تسلسل tick و parent_id)
    pub fn verify_chain(events: &[Event]) -> bool {
        if events.is_empty() {
            return true;
        }

        // 1. التحقق من تزايد tick
        for window in events.windows(2) {
            if window[1].tick < window[0].tick {
                eprintln!(
                    "❌ Chain broken: tick {} appears after {}",
                    window[1].tick, window[0].tick
                );
                return false;
            }
        }

        // 2. التحقق من مراجع parent_id (كل parent_id يجب أن يكون موجودًا)
        let mut ids = HashMap::new();
        for event in events {
            ids.insert(event.id, event.clone());
        }

        for event in events {
            if let Some(parent_id) = event.parent_id {
                if !ids.contains_key(&parent_id) {
                    eprintln!(
                        "❌ Orphan event: {} references missing parent {}",
                        event.id, parent_id
                    );
                    return false;
                }
            }
        }

        // 3. التحقق من تجزئة الحالة (اختياري – مكلف لكن مؤكد)
        let mut state = AppState::default();
        for event in events {
            state = reduce(state, event);
        }
        let final_hash = hash_state(&state);

        // نقارنها بآخر تجزئة محفوظة في اللقطة
        if let Ok(Some(snapshot)) = crate::snapshot::load_snapshot() {
            if snapshot.state_hash != final_hash && !events.is_empty() {
                eprintln!(
                    "❌ Hash mismatch: snapshot {} != computed {}",
                    snapshot.state_hash, final_hash
                );
                return false;
            }
        }

        true
    }

    /// يُصلح السلسلة إذا أمكن (يزيل الأحداث المعزولة)
    pub fn repair_chain(events: &mut Vec<Event>) -> usize {
        let mut removed = 0;

        // إزالة الأحداث المعزولة (التي تشير إلى parent_id مفقود)
        let ids: Vec<u64> = events.iter().map(|e| e.id).collect();
        events.retain(|event| {
            if let Some(parent_id) = event.parent_id {
                if !ids.contains(&parent_id) {
                    removed += 1;
                    return false;
                }
            }
            true
        });

        removed
    }
}
