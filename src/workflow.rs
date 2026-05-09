use serde::{Deserialize, Serialize};
use crate::state::{WorkflowInstance, WorkflowState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub workflow_id: u64,
    pub name: String,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowCommand {
    Register(WorkflowDefinition),
    Transition { workflow_id: u64, new_state: WorkflowState },
}

pub fn workflow_reducer(
    instance: Option<&WorkflowInstance>,
    command: &WorkflowCommand,
    tick: u64,
) -> WorkflowInstance {
    match command {
        WorkflowCommand::Register(def) => {
            WorkflowInstance {
                workflow_id: def.workflow_id,
                definition_hash: String::new(),
                state: WorkflowState::Pending,
                created_tick: tick,
                updated_tick: tick,
            }
        }
        WorkflowCommand::Transition { workflow_id: _, new_state } => {
            let mut inst = instance.unwrap().clone();
            inst.state = new_state.clone();
            inst.updated_tick = tick;
            inst
        }
    }
}
