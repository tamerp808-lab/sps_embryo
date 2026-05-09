use crate::conversation_memory::ConversationMemory;
use crate::event::Event;
use crate::event_log;
use crate::llm_chain;
use crate::logical_clock::LogicalClock;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};
use crate::outbox::EffectOutbox;
use crate::tool_registry::ToolRegistry;
use std::sync::{Arc, Mutex};

pub struct CognitiveLoop;

impl CognitiveLoop {
    pub fn run(
        goal: &str,
        short_mem: &Arc<Mutex<ShortTermMemory>>,
        long_mem: &Arc<LongTermMemory>,
        vec_mem: &Arc<Mutex<VectorMemory>>,
        conv_mem: &Arc<ConversationMemory>,
        outbox: &Arc<Mutex<EffectOutbox>>,
        clock: &Arc<LogicalClock>,
    ) -> String {
        // 1. Observe
        let mut sm = short_mem.lock().unwrap();
        sm.add("user", goal);
        let context = sm.formatted();
        let long_context: String = long_mem
            .retrieve_top(5)
            .iter()
            .map(|(r, c)| format!("[Memory] {}: {}", r, c))
            .collect::<Vec<_>>()
            .join("\n");
        let sim = {
            let vm = vec_mem.lock().unwrap();
            let emb = llm_chain::embed_sync(goal);
            vm.search(&emb, 3).join("\n")
        };
        drop(sm);

        // 2. Think
        let think_prompt = format!(
            "You are the cognitive core of an AI OS. Analyze the goal and decide the SINGLE best immediate action.\n\
             Context:\n{}\n\nLong-term:\n{}\n\nSimilar:\n{}\n\nGoal: \"{}\"\n\n\
             Output ONLY a JSON with:\n\
             {{\"action\":\"chat\"|\"build_app\"|\"open_app\"|\"send_email\"|\"speak\"|\"search\"|\"calendar\"|\"screenshot\"|\"code\"|\"plan\", \"detail\":\"...\"}}",
            context, long_context, sim, goal
        );
        let decision_str = llm_chain::chat_sync(&think_prompt);

        let default_action = format!("{{\"action\":\"chat\", \"detail\":\"{}\"}}", goal);
        let decision: serde_json::Value = serde_json::from_str(&decision_str)
            .unwrap_or_else(|_| serde_json::from_str(&default_action).unwrap());
        let action = decision["action"].as_str().unwrap_or("chat");
        let detail = decision["detail"].as_str().unwrap_or(goal);

        // تسجيل القرار كحدث (حتمي)
        let decision_event = Event {

            id: clock.next_id(),
            tick: clock.current_tick(),
            event_type: "DecisionMade".to_string(),
            payload: format!("Action: {}, Detail: {}", action, detail),
            parent_id: clock.last_id(),
            vector_clock: clock.vector_clock_string(),
        };
        let _ = event_log::append_event(&decision_event);

        // 3. Execute
        let result = match action {
            "open_app" => {
                if !ToolRegistry::is_allowed("mobile") {
                    "❌ Mobile agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::mobile_agent::MobileControlAgent::open_app(detail, &mut ob);
                    format!("📱 Opening app: {}", detail)
                }
            }
            "search" => {
                if !ToolRegistry::is_allowed("browser") {
                    "❌ Browser agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::browser_agent::BrowserAgent::search_web(detail, &mut ob);
                    format!("🔍 Searching for: {}", detail)
                }
            }
            "speak" => {
                if !ToolRegistry::is_allowed("voice") {
                    "❌ Voice agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::voice_agent::VoiceAgent::speak(detail, "ar", &mut ob);
                    format!("🔊 Speaking: {}", detail)
                }
            }
            "screenshot" => {
                if !ToolRegistry::is_allowed("mobile") {
                    "❌ Mobile agent is disabled".to_string()
                } else {
                    let path = format!("/sdcard/sps_{}.png", uuid::Uuid::now_v7());
                    let mut ob = outbox.lock().unwrap();
                    crate::mobile_agent::MobileControlAgent::screenshot(&path, &mut ob);
                    format!("📸 Screenshot saved to {}", path)
                }
            }
            "calendar" => {
                if !ToolRegistry::is_allowed("calendar") {
                    "❌ Calendar agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::calendar_agent::CalendarAgent::add_event(
                        detail, "now", "+1h", "", "", &mut ob,
                    );
                    format!("📅 Event added: {}", detail)
                }
            }
            "email" => {
                if !ToolRegistry::is_allowed("gmail") {
                    "❌ Gmail agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::gmail_agent::GmailAgent::send_email(
                        "user@example.com",
                        "SPS Task",
                        detail,
                        &mut ob,
                    );
                    format!("📧 Email sent: {}", detail)
                }
            }
            "code" => {
                if !ToolRegistry::is_allowed("coder") {
                    "❌ Coder agent is disabled".to_string()
                } else {
                    let mut ob = outbox.lock().unwrap();
                    crate::coder_agent::CoderAgent::write_code(detail, &mut ob);
                    format!("💻 Generating code for: {}", detail)
                }
            }
            "build_app" => {
                if !ToolRegistry::is_allowed("coder") {
                    "❌ Coder agent is disabled".to_string()
                } else {
                    match crate::creator_agent::CreatorAgent::build_site(detail) {
                        Ok(site_id) => format!("🏗️ App built! <a href=\"/view/{}\" target=\"_blank\">Preview</a> | <a href=\"/download/{}\">Download</a>", site_id, site_id),
                        Err(e) => format!("❌ Build failed: {}", e),
                    }
                }
            }
            "plan" => {
                if !ToolRegistry::is_allowed("repair") {
                    "❌ Planner is disabled".to_string()
                } else {
                    format!("📋 Plan created for: {}", detail)
                }
            }
            _ => llm_chain::chat_sync(&format!(
                "You are a helpful AI assistant. Previous context:\n{}\nUser: {}\nAssistant:",
                context, goal
            )),
        };

        // 4. Store
        let mut sm = short_mem.lock().unwrap();
        sm.add("assistant", &result);
        long_mem.store("assistant", &result, None, 0.5).ok();
        conv_mem.add_message("assistant", &result).ok();
        drop(sm);

        result
    }
}
