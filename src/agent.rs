use crate::event::Event;
use crate::event_log::read_all_events;
use crate::state::WorkflowState;
use crate::workflow::WorkflowCommand;

pub struct CognitiveAgent {
    pub agent_id: u64,
}

impl CognitiveAgent {
    pub fn new(agent_id: u64) -> Self {
        Self { agent_id }
    }

    pub fn think(&self) -> Vec<WorkflowCommand> {
        let events = read_all_events().unwrap_or_default();
        let mut commands = Vec::new();

        let has_init = events.iter().any(|e| e.payload == "init");
        if has_init {
            commands.push(WorkflowCommand::Register(
                crate::workflow::WorkflowDefinition { workflow_id: 100 + self.agent_id,
                    name: format!("cognitive_wf_{}", self.agent_id),
                    version: 1,
                },
            ));
        }

        let has_side_effect = events.iter().any(|e| e.payload.contains("side_effect"));
        if has_side_effect {
            commands.push(WorkflowCommand::Transition { workflow_id: 100 + self.agent_id,
                new_state: WorkflowState::Ready,
            });
        }

        commands
    }

    pub fn commands_to_events(
        &self,
        commands: Vec<WorkflowCommand>,
        base_id: u64,
        tick: u64,
    ) -> Vec<Event> {
        commands
            .into_iter()
            .enumerate()
            .map(|(i, cmd)| {
                let payload = format!("WF:{}", serde_json::to_string(&cmd).unwrap());
                Event {

                    id: base_id + i as u64,
                    tick,
                    event_type: "WorkflowCommand".to_string(),
                    payload,
                    parent_id: None,
                    vector_clock: String::new(),
                }
            })
            .collect()
    }
}
