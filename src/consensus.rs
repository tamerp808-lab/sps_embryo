#![allow(dead_code)]

use std::collections::BTreeMap;

/// حالة الإجماع: قائد لكل قسم.
#[derive(Debug, Clone)]
pub struct ConsensusState {
    pub partition_leaders: BTreeMap<u64, u64>, // partition_id -> leader_node_id
}

impl ConsensusState {
    pub fn new() -> Self {
        Self {
            partition_leaders: BTreeMap::new(),
        }
    }

    /// تسجيل قائد لقسم (عادة عبر حدث Lease أو انتخاب).
    pub fn set_leader(&mut self, partition_id: u64, leader_node: u64) {
        self.partition_leaders.insert(partition_id, leader_node);
    }

    /// من هو قائد هذا القسم؟
    pub fn leader_of(&self, partition_id: u64) -> Option<u64> {
        self.partition_leaders.get(&partition_id).copied()
    }
}
