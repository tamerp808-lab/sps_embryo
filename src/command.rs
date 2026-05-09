use serde::{Deserialize, Serialize};

/// الأوامر التي تصل من API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiCommand {
    ExecuteTask { goal: String },
    SetGoal { description: String, priority: u64 },
    DecomposeGoal { goal_id: u64 },
    ListGoals,
    GoalProgress { goal_id: u64 },
}
