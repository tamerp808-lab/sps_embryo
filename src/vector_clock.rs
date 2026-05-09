use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClock {
    pub ticks: BTreeMap<u64, u64>,
}

impl VectorClock {
    pub fn new(node_id: u64) -> Self {
        let mut ticks = BTreeMap::new();
        ticks.insert(node_id, 0);
        Self { ticks }
    }

    pub fn increment(&mut self, node_id: u64) {
        let tick = self.ticks.entry(node_id).or_insert(0);
        *tick += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node_id, tick) in &other.ticks {
            let current = self.ticks.entry(*node_id).or_insert(0);
            *current = (*current).max(*tick);
        }
    }

    pub fn happened_before(&self, other: &VectorClock) -> bool {
        let mut at_least_one_smaller = false;
        let all_keys: Vec<&u64> = self.ticks.keys().chain(other.ticks.keys()).collect();
        for key in all_keys {
            let self_tick = self.ticks.get(key).copied().unwrap_or(0);
            let other_tick = other.ticks.get(key).copied().unwrap_or(0);
            if self_tick > other_tick {
                return false;
            }
            if self_tick < other_tick {
                at_least_one_smaller = true;
            }
        }
        at_least_one_smaller
    }

    pub fn is_concurrent(&self, other: &VectorClock) -> bool {
        !self.happened_before(other) && !other.happened_before(self)
    }

    pub fn to_string(&self) -> String {
        let items: Vec<String> = self
            .ticks
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect();
        items.join(", ")
    }
}
