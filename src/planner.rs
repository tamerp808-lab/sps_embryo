use crate::event::Event;
use crate::workflow::WorkflowCommand;
use crate::state::WorkflowState;
use crate::sandbox;

pub struct PlannerAgent {
    agent_id: u64,
}

impl PlannerAgent {
    pub fn new(agent_id: u64) -> Self { Self { agent_id } }
    pub fn think_deep(&self, events: &[Event]) -> Vec<WorkflowCommand> {
        let prompt = self.build_prompt(events);
        let llm_response = self.query_llm(&prompt);
        self.parse_plan(&llm_response)
    }
    fn build_prompt(&self, events: &[Event]) -> String {
        let mut event_list = String::new();
        for e in events.iter().rev().take(5) {
            event_list.push_str(&format!("- [tick {}] {}\n", e.tick, e.payload));
        }
        format!("You are a planning agent...\n{}\nSuggest next command.", event_list)
    }
    fn query_llm(&self, prompt: &str) -> String {
        if let Ok(r) = sandbox::call_ollama(prompt) { return r; }
        if let Ok(r) = sandbox::call_groq_via_curl(prompt) { return r; }
        format!(r#"LLM simulation: {{"command":"Register","workflow_id":{}}}"#, 200 + self.agent_id)
    }
    fn parse_plan(&self, response: &str) -> Vec<WorkflowCommand> {
        let json_str = response.find('{').map(|i| &response[i..]).unwrap_or("");
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
            match json["command"].as_str().unwrap_or("") {
                "Register" => {
                    vec![WorkflowCommand::Register(crate::workflow::WorkflowDefinition {
                        workflow_id: json["workflow_id"].as_u64().unwrap_or(200+self.agent_id),
                        name: format!("llm_wf_{}", self.agent_id),
                        version: 1,
                    })]
                }
                "Transition" => {
                    let state = match json["new_state"].as_str().unwrap_or("Ready") {
                        "Running" => WorkflowState::Running,
                        _ => WorkflowState::Ready,
                    };
                    vec![WorkflowCommand::Transition {
                        workflow_id: json["workflow_id"].as_u64().unwrap_or(200+self.agent_id),
                        new_state: state,
                    }]
                }
                _ => self.basic_rules(),
            }
        } else { self.basic_rules() }
    }
    fn basic_rules(&self) -> Vec<WorkflowCommand> {
        vec![WorkflowCommand::Register(crate::workflow::WorkflowDefinition {
            workflow_id: 200+self.agent_id,
            name: format!("llm_wf_{}", self.agent_id),
            version: 1,
        })]
    }
    pub fn commands_to_events(&self, commands: Vec<WorkflowCommand>, base_id: u64, tick: u64) -> Vec<Event> {
        commands.into_iter().enumerate().map(|(i, cmd)| {
            let payload = format!("WF:{}", serde_json::to_string(&cmd).unwrap());
            Event {
                id: base_id + i as u64, tick,
                event_type: "WorkflowCommand".to_string(), payload,
                parent_id: None, vector_clock: String::new(),
            }
        }).collect()
    }
}
