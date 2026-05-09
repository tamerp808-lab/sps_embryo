use crate::llm_chain;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

// ──────────────────────────────────────────────
//  هياكل البيانات
// ──────────────────────────────────────────────

struct BrandDNA {
    brand_name: String,
    tagline: String,
    personality: String,
    color_palette: String,
    typography: String,
    animation_mood: String,
    layout_style: String,
    unique_hook: String,
}

struct SiteBundle {
    html: String,
    css: String,
    animations_css: String,
    utilities_css: String,
    js: String,
}

// ──────────────────────────────────────────────
pub struct SiteBuilderAgent;
// ──────────────────────────────────────────────

impl SiteBuilderAgent {

    // ════════════════════════════════════════════
    //  PHASE 0 — Brand Genesis
    // ════════════════════════════════════════════
    fn generate_brand_dna(description: &str) -> BrandDNA {
        let prompt = format!(
            r#"You are a world-class brand strategist and creative director.

CLIENT BRIEF:
{description}

Output ONLY valid JSON:
{{
  "brand_name": "...",
  "tagline": "...",
  "personality": "...",
  "color_palette": "...",
  "typography": "...",
  "animation_mood": "...",
  "layout_style": "...",
  "unique_hook": "..."
}}"#
        );

        let raw = llm_chain::chat_sync(&prompt);

        let json: Value = serde_json::from_str(&raw).unwrap_or_else(|_| {
            let start = raw.find('{').unwrap_or(0);
            let end = raw.rfind('}').map(|i| i + 1).unwrap_or(raw.len());
            serde_json::from_str(&raw[start..end]).unwrap_or(Value::Null)
        });

        let s = |key: &str| -> String {
            json[key].as_str().unwrap_or("").to_string()
        };

        BrandDNA {
            brand_name: s("brand_name"),
            tagline: s("tagline"),
            personality: s("personality"),
            color_palette: s("color_palette"),
            typography: s("typography"),
            animation_mood: s("animation_mood"),
            layout_style: s("layout_style"),
            unique_hook: s("unique_hook"),
        }
    }

    // ════════════════════════════════════════════
    //  PHASE 1 — Sectional HTML Generation
    // ════════════════════════════════════════════

    fn generate_nav_hero(description: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate ONLY NAVBAR + HERO SECTION.

PROJECT:
{description}

BRAND:
{brand_name}

TAGLINE:
{tagline}

PERSONALITY:
{personality}

UNIQUE HOOK:
{unique_hook}

REQUIREMENTS:
- cinematic hero
- glass navbar
- animated gradients
- glowing buttons
- floating elements
- premium SaaS quality
- responsive
- semantic HTML5

OUTPUT ONLY COMPLETE HTML."#,
            description = description,
            brand_name = dna.brand_name,
            tagline = dna.tagline,
            personality = dna.personality,
            unique_hook = dna.unique_hook
        );

        llm_chain::chat_sync(&prompt)
    }

    fn generate_features_stats(description: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate FEATURES + STATS sections.

PROJECT:
{description}

STYLE:
{personality}

REQUIREMENTS:
- 6 feature cards
- hover effects
- glowing borders
- responsive grid
- animated counters
- modern premium layout

OUTPUT ONLY HTML."#,
            description = description,
            personality = dna.personality
        );

        llm_chain::chat_sync(&prompt)
    }

    fn generate_showcase_cta(description: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate SHOWCASE + CTA sections.

PROJECT:
{description}

STYLE:
{personality}

HOOK:
{hook}

REQUIREMENTS:
- showcase cards
- cinematic layout
- hover animations
- dramatic CTA
- premium UI

OUTPUT ONLY HTML."#,
            description = description,
            personality = dna.personality,
            hook = dna.unique_hook
        );

        llm_chain::chat_sync(&prompt)
    }

    fn generate_footer(description: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate premium FOOTER.

PROJECT:
{description}

BRAND:
{brand}

REQUIREMENTS:
- 4 column footer
- social icons
- responsive
- copyright
- elegant dark style

OUTPUT ONLY FOOTER HTML."#,
            description = description,
            brand = dna.brand_name
        );

        llm_chain::chat_sync(&prompt)
    }

    fn assemble_html(
        nav_hero: &str,
        features_stats: &str,
        showcase_cta: &str,
        footer: &str
    ) -> String {

        format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Generated Site</title>

<link rel="stylesheet" href="style.css">
<link rel="stylesheet" href="animations.css">
<link rel="stylesheet" href="utilities.css">
</head>

<body>

{}

{}

{}

{}

<script src="script.js"></script>
</body>
</html>"#,
            strip_html_boilerplate(nav_hero),
            strip_html_boilerplate(features_stats),
            strip_html_boilerplate(showcase_cta),
            strip_html_boilerplate(footer),
        )
    }

    // ════════════════════════════════════════════
    //  PHASE 2 — CSS Architecture
    // ════════════════════════════════════════════

    fn generate_main_css(html: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate COMPLETE premium cinematic CSS.

PERSONALITY:
{}

LAYOUT:
{}

TYPOGRAPHY:
{}

PALETTE:
{}

HTML:
{}

REQUIREMENTS:
- glassmorphism
- animated gradients
- responsive
- hover effects
- cinematic layout
- mobile first
- modern SaaS style

OUTPUT ONLY CSS."#,
            dna.personality,
            dna.layout_style,
            dna.typography,
            dna.color_palette,
            &html[..html.len().min(5000)]
        );

        llm_chain::chat_sync(&prompt)
    }

    fn generate_animations_css(dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate COMPLETE animations.css.

MOOD:
{}

REQUIREMENTS:
- fadeInUp
- fadeInLeft
- fadeInRight
- scaleIn
- glowPulse
- gradientShift
- floatY
- ripple
- shimmer
- reveal animations

OUTPUT ONLY CSS."#,
            dna.animation_mood
        );

        llm_chain::chat_sync(&prompt)
    }

    fn generate_utilities_css(dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate COMPLETE utilities.css.

PALETTE:
{}

INCLUDE:
- containers
- spacing
- grids
- flex helpers
- typography helpers
- responsive helpers
- utility classes

OUTPUT ONLY CSS."#,
            dna.color_palette
        );

        llm_chain::chat_sync(&prompt)
    }

    // ════════════════════════════════════════════
    //  PHASE 3 — Advanced JavaScript
    // ════════════════════════════════════════════

    fn generate_advanced_js(html: &str, dna: &BrandDNA) -> String {
        let prompt = format!(
            r#"Generate COMPLETE professional JavaScript.

PERSONALITY:
{}

HOOK:
{}

FEATURES:
- mobile menu
- smooth scroll
- reveal animations
- intersection observer
- animated counters
- active nav links
- particles
- ripple effects
- scroll progress bar
- parallax
- footer year update

HTML:
{}

OUTPUT ONLY JAVASCRIPT."#,
            dna.personality,
            dna.unique_hook,
            &html[..html.len().min(4000)]
        );

        llm_chain::chat_sync(&prompt)
    }

    // ════════════════════════════════════════════
    //  PHASE 4 — Design Council
    // ════════════════════════════════════════════

    fn design_council_critique(
        html: &str,
        css: &str,
        dna: &BrandDNA
    ) -> Vec<String> {

        let ux_prompt = format!(
            r#"You are a UX Director.

Rate this design from 1-10.

HTML:
{}

CSS:
{}

OUTPUT JSON:
{{
  "score": 0,
  "improvements": [
    "...",
    "...",
    "..."
  ]
}}"#,
            &html[..html.len().min(3000)],
            &css[..css.len().min(3000)]
        );

        let motion_prompt = format!(
            r#"You are a Motion Designer.

Animation mood:
{}

Rate this design.

OUTPUT JSON."#,
            dna.animation_mood
        );

        let brand_prompt = format!(
            r#"You are a Brand Strategist.

Personality:
{}

Hook:
{}

Rate consistency and uniqueness.

OUTPUT JSON."#,
            dna.personality,
            dna.unique_hook
        );

        vec![
            llm_chain::chat_sync(&ux_prompt),
            llm_chain::chat_sync(&motion_prompt),
            llm_chain::chat_sync(&brand_prompt),
        ]
    }

    fn parse_council_feedback(
        critiques: &[String]
    ) -> (f64, Vec<String>) {

        let mut total_score = 0.0;
        let mut count = 0;
        let mut improvements = Vec::new();

        for raw in critiques {

            let start = raw.find('{').unwrap_or(0);
            let end = raw.rfind('}').map(|i| i + 1).unwrap_or(raw.len());

            if let Ok(json) =
                serde_json::from_str::<Value>(&raw[start..end])
            {
                if let Some(score) = json["score"].as_f64() {
                    total_score += score;
                    count += 1;
                }

                if let Some(arr) = json["improvements"].as_array() {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            improvements.push(s.to_string());
                        }
                    }
                }
            }
        }

        let avg = if count > 0 {
            total_score / count as f64
        } else {
            0.0
        };

        (avg, improvements)
    }

    // ════════════════════════════════════════════
    //  PHASE 5 — Iterative Polish
    // ════════════════════════════════════════════

    fn iterative_polish(
        mut bundle: SiteBundle,
        dna: &BrandDNA,
        max_rounds: u8,
    ) -> SiteBundle {

        for round in 0..max_rounds {

            let critiques = Self::design_council_critique(
                &bundle.html,
                &bundle.css,
                dna
            );

            let (score, improvements) =
                Self::parse_council_feedback(&critiques);

            eprintln!(
                "[SiteBuilderAgent] Round {} Score {:.1}/10",
                round + 1,
                score
            );

            if score >= 8.5 {
                break;
            }

            let improve_prompt = format!(
                r#"Improve this design.

IMPROVEMENTS:
{}

HTML:
{}

CSS:
{}

OUTPUT COMPLETE IMPROVED HTML."#,
                improvements.join("\n"),
                bundle.html,
                bundle.css
            );

            bundle.html = llm_chain::chat_sync(&improve_prompt);

            let css_prompt = format!(
                r#"Improve this CSS.

IMPROVEMENTS:
{}

CSS:
{}

OUTPUT COMPLETE IMPROVED CSS."#,
                improvements.join("\n"),
                bundle.css
            );

            bundle.css = llm_chain::chat_sync(&css_prompt);
        }

        bundle
    }

    // ════════════════════════════════════════════
    //  PHASE 6 — Final Consistency Pass
    // ════════════════════════════════════════════

    fn final_consistency_pass(
        bundle: &SiteBundle,
        dna: &BrandDNA
    ) -> (String, String) {

        let html_prompt = format!(
            r#"Final consistency pass.

Ensure:
- accessibility
- responsive structure
- semantic HTML
- valid structure
- animation hooks
- meta tags
- mobile support

HTML:
{}

OUTPUT COMPLETE HTML."#,
            bundle.html
        );

        let css_prompt = format!(
            r#"Final CSS cleanup.

Ensure:
- no duplicate rules
- valid responsive breakpoints
- consistent variables
- keyboard focus styles

CSS:
{}

OUTPUT COMPLETE CSS."#,
            bundle.css
        );

        let final_html = llm_chain::chat_sync(&html_prompt);
        let final_css = llm_chain::chat_sync(&css_prompt);

        (final_html, final_css)
    }

    // ════════════════════════════════════════════
    //  PUBLIC ENTRY
    // ════════════════════════════════════════════

    pub fn build_project(
        description: &str
    ) -> Result<(String, String, String, String), String> {

        eprintln!("[SiteBuilderAgent] Phase 0 → Brand DNA");

        let dna = Self::generate_brand_dna(description);

        eprintln!("[SiteBuilderAgent] Phase 1 → HTML");

        let nav_hero =
            Self::generate_nav_hero(description, &dna);

        let features_stats =
            Self::generate_features_stats(description, &dna);

        let showcase_cta =
            Self::generate_showcase_cta(description, &dna);

        let footer =
            Self::generate_footer(description, &dna);

        let html = Self::assemble_html(
            &nav_hero,
            &features_stats,
            &showcase_cta,
            &footer
        );

        eprintln!("[SiteBuilderAgent] Phase 2 → CSS");

        let css =
            Self::generate_main_css(&html, &dna);

        let animations_css =
            Self::generate_animations_css(&dna);

        let utilities_css =
            Self::generate_utilities_css(&dna);

        eprintln!("[SiteBuilderAgent] Phase 3 → JavaScript");

        let js =
            Self::generate_advanced_js(&html, &dna);

        let mut bundle = SiteBundle {
            html,
            css,
            animations_css,
            utilities_css,
            js,
        };

        eprintln!("[SiteBuilderAgent] Phase 4 → Design Council");

        bundle =
            Self::iterative_polish(bundle, &dna, 3);

        eprintln!("[SiteBuilderAgent] Phase 5 → Final Pass");

        let (final_html, final_css) =
            Self::final_consistency_pass(
                &bundle,
                &dna
            );

        // SAVE

        let project_id =
            uuid::Uuid::new_v4().to_string();

        let home =
            std::env::var("HOME")
                .unwrap_or_else(|_| "/tmp".to_string());

        let project_dir =
            PathBuf::from(home)
                .join("sps_sites")
                .join(&project_id);

        fs::create_dir_all(&project_dir)
            .map_err(|e| e.to_string())?;

        fs::write(
            project_dir.join("index.html"),
            &final_html
        ).map_err(|e| e.to_string())?;

        fs::write(
            project_dir.join("style.css"),
            &final_css
        ).map_err(|e| e.to_string())?;

        fs::write(
            project_dir.join("animations.css"),
            &bundle.animations_css
        ).map_err(|e| e.to_string())?;

        fs::write(
            project_dir.join("utilities.css"),
            &bundle.utilities_css
        ).map_err(|e| e.to_string())?;

        fs::write(
            project_dir.join("script.js"),
            &bundle.js
        ).map_err(|e| e.to_string())?;

        eprintln!(
            "[SiteBuilderAgent] Saved → {}",
            project_dir.display()
        );

        Ok((
            project_id,
            final_html,
            final_css,
            bundle.js
        ))
    }
}

// ──────────────────────────────────────────────
//  Utility
// ──────────────────────────────────────────────

fn strip_html_boilerplate(html: &str) -> String {

    if let Some(body_start) = html.find("<body") {

        let content_start =
            html[body_start..]
                .find('>')
                .map(|i| body_start + i + 1)
                .unwrap_or(body_start);

        let content_end =
            html.rfind("</body>")
                .unwrap_or(html.len());

        return html[content_start..content_end]
            .trim()
            .to_string();
    }

    html.to_string()
}
