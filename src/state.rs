use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use crate::event::Event;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AppState {
    pub counter: u64,
    pub history: Vec<String>,
    pub workflows: BTreeMap<u64, WorkflowInstance>,
    pub goals: BTreeMap<u64, Goal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkflowState {
    Pending, Ready, Running, Completed, Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowInstance {
    pub workflow_id: u64,
    pub definition_hash: String,
    pub state: WorkflowState,
    pub created_tick: u64,
    pub updated_tick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Goal {
    pub id: u64,
    pub description: String,
    pub status: GoalStatus,
    pub priority: u64,
    pub parent_id: Option<u64>,
    pub children: Vec<u64>,
    pub depends_on: Vec<u64>,
    pub created_tick: u64,
    pub updated_tick: u64,
}

impl Default for Goal {
    fn default() -> Self {
        Self {
            id: 0, description: String::new(), status: GoalStatus::Pending,
            priority: 1, parent_id: None, children: vec![], depends_on: vec![],
            created_tick: 0, updated_tick: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GoalStatus {
    Pending, Active, Completed, Failed,
}

impl AppState {
    pub fn apply_event(&mut self, event: &Event) {
        self.counter += 1;
        self.history.push(event.payload.clone());
    }
}
