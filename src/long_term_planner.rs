use crate::llm_chain;

pub struct LongTermPlanner;

impl LongTermPlanner {
    pub fn create_plan(goal: &str) -> String {
        let prompt = format!(
            "Break down this goal into concrete steps as JSON array: [{{'step':int,'task':str}}]. Goal: {}.\nOutput ONLY JSON.",
            goal
        );
        llm_chain::chat_sync(&prompt)
    }

    pub fn decompose(
        goal_id: u64,
        state: &std::sync::Arc<std::sync::Mutex<crate::state::AppState>>,
    ) -> Result<Vec<String>, String> {
        let s = state.lock().map_err(|e| e.to_string())?;
        let goal = s.goals.get(&goal_id).ok_or("Goal not found")?;
        let prompt = format!(
            "Break down goal: '{}' into 3-5 concrete steps. Output JSON array of strings.",
            goal.description
        );
        let reply = crate::llm_chain::chat_sync(&prompt);
        serde_json::from_str(&reply).map_err(|e| e.to_string())
    }
}
