use crate::llm_chain;
use crate::memory_system::LongTermMemory;

/// خط أنابيب التوليد المتكامل: Observe → Generate → Evaluate → Repair → Refactor
pub struct GenerationPipeline;

impl GenerationPipeline {
    /// ينفذ دورة كاملة لتوليد مشروع عالي الجودة
    pub async fn run(
        description: &str,
        memory: &LongTermMemory,
    ) -> Result<(String, String, String), String> {
        // 1. Observe: تحليل المتطلبات
        println!("🔍 Pipeline: Analyzing requirements...");
        let analysis = llm_chain::chat_sync(&format!(
            "Analyze this project requirement and identify: 1) Key features, 2) Technical stack, 3) Potential challenges. Requirement: {}", description
        ));

        // 2. Generate: توليد الكود الأولي
        println!("⚙️ Pipeline: Generating initial code...");
        let html = llm_chain::chat_sync(&format!(
            "Create COMPLETE HTML for: {}. Analysis: {}. Use semantic HTML5, modern design.", description, analysis
        ));
        let css = llm_chain::chat_sync(&format!(
            "Create professional CSS for this HTML: {}. Responsive, animations, modern.", html
        ));
        let js = llm_chain::chat_sync(&format!(
            "Create interactive JS for this HTML: {}. Form validation, smooth scroll, mobile menu.", html
        ));

        // 3. Evaluate: تقييم الجودة
        println!("📊 Pipeline: Evaluating quality...");
        let evaluation = llm_chain::chat_sync(&format!(
            "Rate this project on a scale of 1-10 for: design, responsiveness, interactivity.\nHTML: {}\nCSS: {}\nJS: {}\nOutput ONLY JSON with scores.", html, css, js
        ));

        // 4. Repair: إصلاح المشكلات
        let mut final_html = html.clone();
        let mut final_css = css.clone();
        let mut final_js = js.clone();

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&evaluation) {
            let score = json["design"].as_f64().unwrap_or(10.0);
            if score < 8.0 {
                println!("🔧 Pipeline: Repairing issues (score: {})...", score);
                final_html = llm_chain::chat_sync(&format!(
                    "Improve this HTML to score 9+/10 for design. Original: {}", html
                ));
                final_css = llm_chain::chat_sync(&format!(
                    "Improve this CSS for better responsiveness. Original: {}", css
                ));
            }
        }

        // 5. Refactor: تحسين المعمارية
        println!("✨ Pipeline: Final polish...");
        let _polished = llm_chain::chat_sync(&format!(
            "Add final professional touches to this project. Add comments, improve accessibility, ensure best practices."
        ));

        // 6. تخزين في الذاكرة
        memory.store("project", &description, None, 0.9).ok();
        memory.record_episode("project_generated", &format!("Project: {} | Status: complete", description), 0.8).ok();

        Ok((final_html, final_css, final_js))
    }
}
