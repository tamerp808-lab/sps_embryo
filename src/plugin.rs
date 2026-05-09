#![allow(dead_code)]

use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct PluginAgent {
    pub agent_id: u64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    agents: BTreeMap<u64, PluginAgent>,
    next_id: u64,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            agents: BTreeMap::new(),
            next_id: 300,
        }
    }

    pub fn register(&mut self, name: String, description: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.agents.insert(
            id,
            PluginAgent {
                agent_id: id,
                name,
                description,
            },
        );
        id
    }

    pub fn list(&self) -> Vec<&PluginAgent> {
        self.agents.values().collect()
    }
}
