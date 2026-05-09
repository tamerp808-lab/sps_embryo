use crate::state::Goal;

/// يُحدد ترتيب تنفيذ المهام بناءً على الأولوية والوقت
pub struct TaskPrioritizer;

impl TaskPrioritizer {
    /// يُرتب الأهداف ويُعيد الهدف ذو أعلى أولوية
    pub fn pick_highest_priority(goals: &[Goal]) -> Option<&Goal> {
        goals.iter()
            .filter(|g| g.status == crate::state::GoalStatus::Pending)
            .max_by_key(|g| (g.priority, u64::MAX - g.created_tick))
    }

    /// يُعيد ترتيبًا لمجموعة أهداف (للجدولة)
    pub fn order(goals: &mut Vec<Goal>) {
        goals.sort_by_key(|g| (std::cmp::Reverse(g.priority), g.created_tick));
    }
}
