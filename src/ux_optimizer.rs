use crate::llm_chain;

pub struct UXOptimizerAgent;

impl UXOptimizerAgent {
    pub fn optimize_page(html: &str) -> String {
        let prompt = format!(
            r#"You are a UX expert with 15 years of experience.
Analyze this HTML page and improve its user experience.

Page:
{}

Improvements to apply:
1. Add proper ARIA labels for accessibility
2. Improve loading speed (optimize images, lazy loading)
3. Add micro-interactions (hover effects, transitions)
4. Ensure keyboard navigation works
5. Add skip-to-content link
6. Improve form validation with real-time feedback
7. Add loading states for buttons
8. Optimize for mobile touch targets (min 44px)

Output the COMPLETE improved HTML code. No explanations."#,
            html
        );
        llm_chain::chat_sync(&prompt)
    }
}
