use crate::ui_designer::UIDesignerAgent;
use crate::ux_optimizer::UXOptimizerAgent;
use crate::seo_agent::SEOAgent;
use crate::memory_system::LongTermMemory;
use crate::creator_agent::CreatorAgent;
use crate::agent_visualizer::AgentVisualizer;
use tokio::sync::broadcast;

pub struct MasterArchitect;

impl MasterArchitect {
    pub fn build_professional_site(
        project_name: &str,
        description: &str,
        keywords: &str,
        memory: &LongTermMemory,
        tx: &broadcast::Sender<String>,
    ) -> Result<String, String> {
        AgentVisualizer::send_agent_state(tx, "brain", "thinking", "Analyzing...");
        let html = UIDesignerAgent::design_page(description, memory);
        let optimized = UXOptimizerAgent::optimize_page(&html);
        let seo_optimized = SEOAgent::optimize_seo(&optimized, keywords);

        let site_id = CreatorAgent::build_site(&format!("{} - {}", project_name, description))?;
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let path = std::path::PathBuf::from(home).join("sps_sites").join(&site_id).join("index.html");
        std::fs::create_dir_all(path.parent().unwrap()).ok();
        std::fs::write(&path, &seo_optimized).map_err(|e| e.to_string())?;

        memory.store("design", &seo_optimized, None, 0.95).ok();
        AgentVisualizer::send_log(tx, "success", &format!("Site built: {}", site_id));

        Ok(site_id)
    }
}
