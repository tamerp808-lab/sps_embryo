use crate::llm_chain;

pub struct AgentArchitect;

impl AgentArchitect {
    pub fn create_agent(description: &str) -> String {
        let prompt = format!(
            "Design and write Python code for an AI agent that can: {}. Output ONLY the Python class code.",
            description
        );
        llm_chain::chat_sync(&prompt)
    }
}
