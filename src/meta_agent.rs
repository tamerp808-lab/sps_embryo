use crate::state::{Goal, GoalStatus};
use crate::swarm_bus::SwarmBus;
use crate::tool_registry::ToolRegistry;

pub struct MetaAgent;

impl MetaAgent {
    /// يُفوض مهمة إلى الوكيل الأنسب مع محاولات إعادة التشغيل عند الفشل
    pub fn delegate_task(action: &str, detail: &str) -> String {
        let target = Self::best_agent_for(action);
        if !ToolRegistry::is_allowed(target) {
            return format!("❌ Agent '{}' is currently disabled.", target);
        }

        let mut last_error = String::new();
        for attempt in 1..=3 {
            match SwarmBus::request(target, detail, 5000) {
                Ok(reply) => {
                    return format!("✅ {} completed (attempt {}): {}", target, attempt, reply)
                }
                Err(e) => {
                    last_error = e;
                    std::thread::sleep(std::time::Duration::from_millis(200 * attempt));
                }
            }
        }
        format!("❌ {} failed after 3 attempts: {}", target, last_error)
    }

    /// يُحدد أفضل وكيل للإجراء
    fn best_agent_for(action: &str) -> &str {
        match action {
            "speak" | "listen" => "voice",
            "open_app" | "screenshot" | "input_text" | "swipe" | "tap" => "mobile",
            "search" | "fetch" | "browse" => "browser",
            "code" | "build_app" => "coder",
            "calendar" => "calendar",
            "email" | "send_email" => "gmail",
            "repair" | "plan" | "decompose" => "repair",
            _ => "brain",
        }
    }

    /// يُعيد أفضل هدف للتنفيذ الفوري (أعلى أولوية، أقدم تاريخ)
    pub fn next_goal(goals: &[Goal]) -> Option<Goal> {
        let mut pending: Vec<&Goal> = goals
            .iter()
            .filter(|g| g.status == GoalStatus::Pending)
            .collect();

        if pending.is_empty() {
            return None;
        }

        pending.sort_by_key(|g| (g.priority, g.created_tick));
        pending.first().map(|g| (*g).clone())
    }

    /// تحليل أداء النظام الكلي
    pub fn analyze_performance() -> String {
        let agents = SwarmBus::list_agents();
        if agents.is_empty() {
            return "No active agents.".to_string();
        }
        format!(
            "Active agents ({}): {}. System: {}",
            agents.len(),
            agents.join(", "),
            crate::resource_monitor::ResourceMonitor::stats()
        )
    }
}
