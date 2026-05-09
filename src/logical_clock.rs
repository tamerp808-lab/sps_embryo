use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use crate::vector_clock::VectorClock;

pub struct LogicalClock {
    tick: AtomicU64,
    id_counter: AtomicU64,
    vector_clock: Mutex<VectorClock>,
}

impl LogicalClock {
    pub fn new(node_id: u64) -> Self {
        Self {
            tick: AtomicU64::new(1),
            id_counter: AtomicU64::new(1),
            vector_clock: Mutex::new(VectorClock::new(node_id)),
        }
    }

    pub fn next_tick(&self) -> u64 {
        let t = self.tick.fetch_add(1, Ordering::SeqCst);
        self.vector_clock.lock().unwrap().increment(0);
        t
    }

    pub fn next_id(&self) -> u64 {
        self.id_counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn current_tick(&self) -> u64 {
        self.tick.load(Ordering::SeqCst)
    }

    pub fn last_id(&self) -> Option<u64> {
        let c = self.id_counter.load(Ordering::SeqCst);
        if c > 1 { Some(c - 1) } else { None }
    }

    pub fn vector_clock_string(&self) -> String {
        self.vector_clock.lock().unwrap().to_string()
    }

    pub fn advance(&self, _node_id: u64) {
        // الطريقة القديمة موجودة الآن لتجنب الأخطاء، لكنها لا تفعل شيئًا
    }
}
