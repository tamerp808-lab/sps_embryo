use std::collections::BTreeMap;
use std::sync::Mutex;

pub struct AgentMetrics {
    counters: Mutex<BTreeMap<String, u64>>,
}

impl AgentMetrics {
    pub fn new() -> Self {
        Self {
            counters: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn record(&self, agent: &str, success: bool) {
        let mut counters = self.counters.lock().unwrap();
        let entry = counters.entry(agent.to_string()).or_insert(0);
        if success {
            *entry += 1;
        }
    }

    pub fn report(&self) -> String {
        let counters = self.counters.lock().unwrap();
        let mut output = String::new();
        for (agent, count) in counters.iter() {
            output.push_str(&format!("{}: {} successful tasks\n", agent, count));
        }
        output
    }
}
