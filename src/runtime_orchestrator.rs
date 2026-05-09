use crate::agent_visualizer::AgentVisualizer;
use crate::llm_chain;
use crate::memory_system::LongTermMemory;
use tokio::sync::broadcast;

pub struct RuntimeOrchestrator;

impl RuntimeOrchestrator {
    /// يُشغّل دورة حياة مشروع كاملة ويُصدر أحداثًا حية عبر WebSocket
    pub async fn run_project_lifecycle(
        description: &str,
        memory: &LongTermMemory,
        tx: &broadcast::Sender<String>,
    ) -> Result<(String, String, String), String> {
        // 1. مرحلة التحليل (BrainAgent)
        AgentVisualizer::send_agent_state(tx, "brain", "thinking", "Analyzing requirements...");
        let analysis = llm_chain::chat_sync(&format!(
            "Analyze this project: {}. Identify features, tech stack, challenges.", description
        ));
        AgentVisualizer::send_agent_state(tx, "brain", "idle", "Analysis complete");
        AgentVisualizer::send_log(tx, "info", &format!("Analysis: {}", &analysis[..50.min(analysis.len())]));

        // 2. مرحلة التخطيط (Planner)
        AgentVisualizer::send_agent_state(tx, "planner", "thinking", "Creating task plan...");
        let plan = llm_chain::chat_sync(&format!(
            "Create a step-by-step plan to build: {}. Analysis: {}", description, analysis
        ));
        AgentVisualizer::send_agent_state(tx, "planner", "idle", "Plan created");
        AgentVisualizer::send_log(tx, "info", "Project plan generated.");

        // 3. مرحلة التوليد (CoderAgent)
        AgentVisualizer::send_agent_state(tx, "coder", "running", "Generating HTML...");
        let html = llm_chain::chat_sync(&format!(
            "Create COMPLETE HTML for: {}. Plan: {}", description, plan
        ));
        AgentVisualizer::send_agent_state(tx, "coder", "running", "Generating CSS...");
        let css = llm_chain::chat_sync(&format!(
            "Create professional CSS for this HTML. Modern, responsive.", 
        ));
        AgentVisualizer::send_agent_state(tx, "coder", "running", "Generating JavaScript...");
        let js = llm_chain::chat_sync(&format!(
            "Create interactive JS for this HTML. Smooth, accessible.", 
        ));
        AgentVisualizer::send_agent_state(tx, "coder", "idle", "Code generation complete");

        // 4. مرحلة التقييم (RepairAgent)
        AgentVisualizer::send_agent_state(tx, "repair", "thinking", "Evaluating quality...");
        let evaluation = llm_chain::chat_sync(&format!(
            "Rate this project 1-10 for design, responsiveness, accessibility. Output JSON."
        ));
        AgentVisualizer::send_agent_state(tx, "repair", "idle", "Evaluation complete");
        AgentVisualizer::send_log(tx, "info", &format!("Quality assessment: {}", evaluation));

        // 5. مرحلة التحسين (UX & SEO)
        AgentVisualizer::send_agent_state(tx, "ux", "running", "Optimizing user experience...");
        let final_html = llm_chain::chat_sync(&format!(
            "Improve this HTML for UX: {}", html
        ));
        AgentVisualizer::send_agent_state(tx, "seo", "running", "Optimizing for search engines...");
        let final_css = llm_chain::chat_sync(&format!(
            "Add SEO best practices to this CSS: {}", css
        ));
        AgentVisualizer::send_agent_state(tx, "ux", "idle", "UX optimization done");
        AgentVisualizer::send_agent_state(tx, "seo", "idle", "SEO optimization done");

        // 6. تخزين في الذاكرة طويلة المدى
        memory.store("project", &description, None, 0.9).ok();
        memory.record_episode("project_lifecycle", &format!("Project: {}", description), 0.8).ok();
        AgentVisualizer::send_log(tx, "success", "Project lifecycle completed successfully.");

        Ok((final_html, final_css, js))
    }
}
