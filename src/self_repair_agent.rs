use crate::llm_chain;

pub struct SelfRepairAgent;

impl SelfRepairAgent {
    pub fn attempt_repair(task: &str, failed_code: &str, error: &str) -> String {
        let prompt = format!(
            "Fix this Python code that has the following error.\nError: {}\nTask: {}\nCode:\n{}\nOutput only corrected Python code.",
            error, task, failed_code
        );
        llm_chain::chat_sync(&prompt)
    }
}
