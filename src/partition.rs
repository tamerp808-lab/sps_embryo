#![allow(dead_code)]

use crate::event_log::{append_event, Event};
use crate::journal::DeliveryJournal;
use crate::reducer::idempotent_reduce;
use crate::state::AppState;
use std::collections::BTreeMap;

/// قسم منطقي: يحتفظ بـ EventLog خاص به.
#[derive(Debug, Clone)]
pub struct Partition {
    pub partition_id: u64,
    pub owner_node: u64,
    pub events: Vec<Event>,
}

impl Partition {
    pub fn new(partition_id: u64, owner_node: u64) -> Self {
        Self {
            partition_id,
            owner_node,
            events: Vec::new(),
        }
    }

    /// إضافة حدث إلى القسم (يُكتب في السجل المحلي).
    pub fn push_event(&mut self, event: Event) {
        append_event(&event).expect("partition: append failed");
        self.events.push(event);
    }
}

/// مدير الأقسام المحلي.
#[derive(Debug)]
pub struct PartitionManager {
    partitions: BTreeMap<u64, Partition>,
    local_node: u64,
}

impl PartitionManager {
    pub fn new(local_node: u64) -> Self {
        Self {
            partitions: BTreeMap::new(),
            local_node,
        }
    }

    /// تعيين أو إنشاء قسم.
    pub fn assign_partition(&mut self, partition_id: u64, owner_node: u64) {
        let partition = Partition::new(partition_id, owner_node);
        self.partitions.insert(partition_id, partition);
    }

    /// الحصول على قسم.
    pub fn get_partition(&self, partition_id: u64) -> Option<&Partition> {
        self.partitions.get(&partition_id)
    }

    /// الحصول على قسم متغير.
    pub fn get_partition_mut(&mut self, partition_id: u64) -> Option<&mut Partition> {
        self.partitions.get_mut(&partition_id)
    }

    /// هل هذا القسم مملوك لهذه العقدة؟
    pub fn is_owned(&self, partition_id: u64) -> bool {
        self.partitions
            .get(&partition_id)
            .map_or(false, |p| p.owner_node == self.local_node)
    }

    /// إضافة حدث إلى قسم محدد.
    pub fn push_to(&mut self, partition_id: u64, event: Event) {
        if let Some(partition) = self.partitions.get_mut(&partition_id) {
            partition.push_event(event);
        }
    }

    /// إعادة بناء حالة المخفضات من جميع الأقسام المحلية.
    pub fn rebuild_state(&self) -> AppState {
        let mut state = AppState::default();
        let mut journal = DeliveryJournal::new();
        // نجمع كل الأحداث من جميع الأقسام المحلية
        let mut all_events: Vec<Event> = self
            .partitions
            .values()
            .flat_map(|p| p.events.clone())
            .collect();
        all_events.sort_by_key(|e| (e.tick, e.id));
        for event in &all_events {
            state = idempotent_reduce(state, event, &mut journal);
        }
        state
    }
}
