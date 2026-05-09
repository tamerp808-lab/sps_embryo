#![allow(dead_code)]
use std::collections::BTreeSet;

/// سجل تسليم حتمي. يضمن أن كل حدث يُعالج مرة واحدة فقط.
#[derive(Debug, Clone)]
pub struct DeliveryJournal {
    processed: BTreeSet<u64>, // معرفات الأحداث التي عولجت
}

impl DeliveryJournal {
    pub fn new() -> Self {
        Self {
            processed: BTreeSet::new(),
        }
    }

    /// هل سُجّل الحدث مسبقًا؟
    pub fn already_processed(&self, event_id: u64) -> bool {
        self.processed.contains(&event_id)
    }

    /// يُسجّل أن الحدث قد عولج.
    pub fn record(&mut self, event_id: u64) {
        self.processed.insert(event_id);
    }

    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}
