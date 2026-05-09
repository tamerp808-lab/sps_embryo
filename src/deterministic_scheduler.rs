use crate::logical_clock::LogicalClock;
use crate::state::{AppState, Goal, GoalStatus};

/// مجدول حتمي: يُحدد ترتيب تنفيذ المهام بناءً على `logical_tick` والأولوية فقط.
pub struct DeterministicScheduler;

impl DeterministicScheduler {
    /// يُعيد أفضل هدف للتنفيذ الفوري
    pub fn next_task(state: &AppState, _clock: &LogicalClock) -> Option<Goal> {
        let mut pending: Vec<&Goal> = state
            .goals
            .values()
            .filter(|g| g.status == GoalStatus::Pending)
            .collect();

        if pending.is_empty() {
            return None;
        }

        // ترتيب حتمي: أولاً حسب الأولوية (الأقل قيمة = أعلى أولوية)، ثم حسب created_tick الأقدم
        pending.sort_by_key(|g| (g.priority, g.created_tick));

        pending.first().map(|g| (*g).clone())
    }

    /// يُخطط لتنفيذ جميع الأهداف بالترتيب الحتمي
    pub fn schedule(state: &AppState) -> Vec<Goal> {
        let mut goals: Vec<Goal> = state
            .goals
            .values()
            .filter(|g| g.status == GoalStatus::Pending)
            .cloned()
            .collect();

        goals.sort_by_key(|g| (g.priority, g.created_tick));
        goals
    }
}
