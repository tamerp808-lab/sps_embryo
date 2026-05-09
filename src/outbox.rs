#![allow(dead_code)]
use std::collections::BTreeMap;

// ... (نفس المحتوى السابق مع إضافة الدالة الجديدة)

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffectType {
    Subprocess,
    ApiCall,
    LlmInference,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectRequest {
    pub id: u64,
    pub tick: u64,
    pub effect_type: EffectType,
    pub payload: String,
}

#[derive(Debug)]
pub struct EffectOutbox {
    pending: BTreeMap<u64, EffectRequest>,
}

impl EffectOutbox {
    pub fn new() -> Self {
        Self {
            pending: BTreeMap::new(),
        }
    }

    pub fn enqueue(&mut self, effect: EffectRequest) {
        self.pending.insert(effect.id, effect);
    }

    pub fn complete(&mut self, effect_id: u64) -> Option<EffectRequest> {
        self.pending.remove(&effect_id)
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn contains(&self, effect_id: u64) -> bool {
        self.pending.contains_key(&effect_id)
    }

    /// دالة جديدة: تُعيد قائمة بمعرفات جميع الآثار المعلقة
    pub fn pending_ids(&self) -> Vec<u64> {
        self.pending.keys().cloned().collect()
    }
}
