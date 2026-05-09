use std::collections::BTreeMap;

pub struct SimpleRLScorer {
    scores: BTreeMap<String, f64>,
    total_calls: u64,
}

impl SimpleRLScorer {
    pub fn new() -> Self {
        Self {
            scores: BTreeMap::new(),
            total_calls: 0,
        }
    }
    pub fn choose_action(&self, actions: &[String]) -> String {
        actions
            .iter()
            .max_by(|a, b| {
                self.scores
                    .get(*a)
                    .unwrap_or(&0.0)
                    .partial_cmp(self.scores.get(*b).unwrap_or(&0.0))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(&actions[0])
            .clone()
    }
    pub fn update(&mut self, action: &str, reward: f64) {
        let entry = self.scores.entry(action.to_string()).or_insert(0.0);
        *entry = (*entry * self.total_calls as f64 + reward) / (self.total_calls + 1) as f64;
        self.total_calls += 1;
    }
    pub fn report(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.scores {
            out.push_str(&format!("{}: {:.3}\n", k, v));
        }
        out
    }
}
