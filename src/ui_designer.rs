use crate::llm_chain;
use crate::memory_system::LongTermMemory;

pub struct UIDesignerAgent;

impl UIDesignerAgent {
    pub fn design_page(project_description: &str, memory: &LongTermMemory) -> String {
        let past_designs = memory
            .retrieve_top(5)
            .iter()
            .filter(|(role, _)| role == "design")
            .map(|(_, content)| content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"You are a world-class UI designer with 20 years of experience.
Your designs are always rated 10/10 for aesthetics and usability.

Past successful designs (for inspiration):
{}

Project: {}

Requirements:
- Create a COMPLETE, production-ready HTML page.
- Use Tailwind CSS CDN for styling.
- Include these elements (tailor to the project):
  - Responsive navigation bar with logo and links
  - Hero section with headline, subtext, and CTA button
  - Features/services section with cards/icons
  - About/Testimonials section
  - Pricing section (if applicable)
  - Contact form with validation
  - Footer with social links
- Add smooth scroll behavior and fade-in animations.
- Use a professional color palette (dark/light mode compatible).
- Ensure mobile responsiveness (all breakpoints).
- Output ONLY the complete HTML code. No explanations.
- The code MUST be ready to deploy immediately.

HTML Code:"#,
            past_designs, project_description
        );

        let mut design = llm_chain::chat_sync(&prompt);

        // Self-critique
        let critique_prompt = format!(
            r#"You are a strict UI critic. Rate this design on a scale of 1-10 for:
- Visual appeal
- Professional look
- Responsive design
- Code quality

Design:
{}

Output ONLY a JSON with: {{ "score": <number>, "issues": [<list of problems>], "improved": false }}"#,
            design
        );

        let critique = llm_chain::chat_sync(&critique_prompt);
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&critique) {
            let score = json["score"].as_f64().unwrap_or(10.0);
            if score < 9.0 {
                let improve_prompt = format!(
                    r#"The following design scored {}/10. Fix these issues and improve the design to score 10/10:
Issues: {}

Original Design:
{}

Output ONLY the improved complete HTML code:"#,
                    score, json["issues"], design
                );
                design = llm_chain::chat_sync(&improve_prompt);
            }
        }

        memory.store("design", &design, None, 0.9).ok();
        memory
            .record_episode(
                "design_created",
                &format!("Project: {} | Score: 9+", project_description),
                0.8,
            )
            .ok();

        design
    }
}
