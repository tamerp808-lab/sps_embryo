use crate::llm_chain;

pub struct SEOAgent;

impl SEOAgent {
    pub fn optimize_seo(html: &str, target_keywords: &str) -> String {
        let prompt = format!(
            r#"You are an SEO expert who always achieves 100/100 on Google Lighthouse.

Target Keywords: {}

HTML Page:
{}

Apply these SEO improvements:
1. Add proper meta tags (title, description, keywords, viewport)
2. Add Open Graph tags (og:title, og:description, og:image, og:url, og:type)
3. Add Twitter Card tags
4. Add Schema.org structured data (JSON-LD) for Organization/Website
5. Add semantic HTML5 elements (header, main, nav, footer, article, section)
6. Use proper heading hierarchy (h1 → h2 → h3)
7. Add alt text to all images
8. Create a sitemap.xml and robots.txt links

Output the COMPLETE improved HTML code. No explanations."#,
            target_keywords, html
        );
        llm_chain::chat_sync(&prompt)
    }
}
