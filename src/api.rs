#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use tiny_http::{Server, Response, Method, Header, StatusCode};
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use crate::outbox::EffectOutbox;
use crate::capabilities::CapabilityToken;
use crate::conversation_memory::ConversationMemory;
use crate::memory_system::{ShortTermMemory, LongTermMemory, VectorMemory};
use crate::voice_agent::VoiceAgent;
use crate::mobile_agent::MobileControlAgent;
use crate::browser_agent::BrowserAgent;
use crate::resource_monitor::ResourceMonitor;
use crate::journal::DeliveryJournal;
use crate::reducer::ReducerRegistry;
use crate::partition::PartitionManager;
use crate::plugin::PluginRegistry;
use crate::lease::LeaseManager;
use crate::state_store::StateStore;
use crate::site_builder::SiteBuilderAgent;
use crate::generation_pipeline::GenerationPipeline;
use crate::agent_architect::AgentArchitect;
use crate::backend_agent::BackendAgent;
use crate::job_queue::JobQueue;

#[derive(Clone)]
pub struct ApiState {
    pub node_id: u64,
    pub partitions: Vec<u64>,
    pub peers: BTreeMap<u64, String>,
}

const MAX_BODY_SIZE: usize = 1024 * 1024;

pub fn start_api_server(
    state_store: Arc<StateStore>,
    journal: Arc<Mutex<DeliveryJournal>>,
    registry: Arc<ReducerRegistry>,
    port: u16,
    partition_mgr: Arc<Mutex<PartitionManager>>,
    api_state: Arc<Mutex<ApiState>>,
    plugin_registry: Arc<Mutex<PluginRegistry>>,
    outbox: Arc<Mutex<EffectOutbox>>,
    lease_mgr: Arc<Mutex<LeaseManager>>,
    token: Arc<CapabilityToken>,
    conv_mem: Arc<ConversationMemory>,
    short_mem: Arc<Mutex<ShortTermMemory>>,
    long_mem: Arc<LongTermMemory>,
    vec_mem: Arc<Mutex<VectorMemory>>,
    job_queue: Arc<JobQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)?;
    println!("API server listening on http://{}", addr);

    let dashboard_html = include_str!("../dashboard.html");

    for mut request in server.incoming_requests() {
        let method = request.method().clone();
        let url = request.url().to_string();

        if url == "/" || url == "/dashboard" {
            let header = Header::from_bytes("Content-Type", "text/html; charset=utf-8")
                .unwrap_or_else(|_| Header::from_bytes("Content-Type", "text/plain").unwrap());
            request.respond(Response::from_data(dashboard_html.as_bytes().to_vec()).with_header(header))?;
            continue;
        }

        if url == "/health" {
            request.respond(Response::from_string("OK\n"))?;
            continue;
        }

        let mut body = String::new();
        let _ = request.as_reader().take(MAX_BODY_SIZE as u64).read_to_string(&mut body);
        let params: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();

        let result = match (method, url.as_str()) {
            (Method::Get, "/state") => {
                let s = state_store.read_state();
                let json = serde_json::to_string_pretty(&s).unwrap_or_default();
                let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                Ok(Response::from_data(json.into_bytes()).with_header(header))
            }
            (Method::Post, "/voice/speak") => {
                let text = params["text"].as_str().unwrap_or("hello");
                let lang = params["lang"].as_str().unwrap_or("en");
                let mut ob = outbox.lock().map_err(|e| e.to_string())?;
                VoiceAgent::speak(text, lang, &mut ob);
                Ok(Response::from_string("Speak queued\n"))
            }
            (Method::Post, "/mobile/open_app") => {
                let pkg = params["package"].as_str().unwrap_or("com.android.chrome");
                let mut ob = outbox.lock().map_err(|e| e.to_string())?;
                MobileControlAgent::open_app(pkg, &mut ob);
                Ok(Response::from_string("Open app queued\n"))
            }
            (Method::Post, "/browser/fetch") => {
                let url_str = params["url"].as_str().unwrap_or("https://example.com");
                let mut ob = outbox.lock().map_err(|e| e.to_string())?;
                BrowserAgent::fetch_page(url_str, &mut ob);
                Ok(Response::from_string("Fetch queued\n"))
            }
            (Method::Post, "/execute") => {
                let goal = params["goal"].as_str().unwrap_or("");
                let reply = crate::llm_chain::chat_sync(goal);
                Ok(Response::from_string(reply))
            }
            (Method::Post, "/repair") => {
                let task = params["task"].as_str().unwrap_or("");
                let code = params["code"].as_str().unwrap_or("");
                let err = params["error"].as_str().unwrap_or("");
                let reply = crate::self_repair_agent::SelfRepairAgent::attempt_repair(task, code, err);
                Ok(Response::from_string(reply))
            }
            (Method::Post, "/plan") => {
                let goal = params["goal"].as_str().unwrap_or("");
                let reply = crate::long_term_planner::LongTermPlanner::create_plan(goal);
                Ok(Response::from_string(reply))
            }
            (Method::Post, "/site/build_project") => {
                let desc = params["description"].as_str().unwrap_or("");
                match SiteBuilderAgent::build_project(desc) {
                    Ok((_project_id, html, css, js)) => {
                        let json = serde_json::json!({"html": html, "css": css, "js": js});
                        let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                        Ok(Response::from_data(json.to_string().into_bytes()).with_header(header))
                    }
                    Err(e) => Err(e),
                }
            }
            (Method::Post, "/site/build_backend") => {
                let desc = params["description"].as_str().unwrap_or("");
                let code = BackendAgent::build_backend(desc);
                Ok(Response::from_string(code))
            }
            (Method::Post, "/site/review") => {
                let desc = params["description"].as_str().unwrap_or("");
                let review = crate::llm_chain::chat_sync(&format!("Review this website: {}. Suggest 3 improvements.", desc));
                Ok(Response::from_string(review))
            }
            (Method::Post, "/site/build_factory") => {
                let name = params["name"].as_str().unwrap_or("My Site");
                let desc = params["description"].as_str().unwrap_or("");
                let keywords = params["keywords"].as_str().unwrap_or("");
                let job_id = job_queue.enqueue(name.to_string(), desc.to_string(), keywords.to_string());
                let json = serde_json::json!({"job_id": job_id, "status": "queued"});
                let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                Ok(Response::from_data(json.to_string().into_bytes()).with_header(header))
            }
            (Method::Get, url) if url.starts_with("/job/status/") => {
                let job_id = url.trim_start_matches("/job/status/");
                if let Some(job) = job_queue.get_status(job_id) {
                    let json = serde_json::json!({"job_id": job.id, "status": job.status, "result": job.result});
                    let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                    Ok(Response::from_data(json.to_string().into_bytes()).with_header(header))
                } else {
                    Err("Job not found".to_string())
                }
            }
            (Method::Get, url) if url.starts_with("/goal/list") => {
                let state = state_store.read_state();
                let goals: Vec<_> = state.goals.values().cloned().collect();
                let json = serde_json::to_string_pretty(&goals).unwrap_or_default();
                let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                Ok(Response::from_data(json.into_bytes()).with_header(header))
            }
            (Method::Post, "/goal/set") => {
                let desc = params["description"].as_str().unwrap_or("");
                let prio = params["priority"].as_u64().unwrap_or(1);
                let id = uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("0").to_string();
                let goal = crate::state::Goal {
                    id: id.parse().unwrap_or(0),
                    description: desc.to_string(),
                    status: crate::state::GoalStatus::Pending,
                    priority: prio,
                    parent_id: None,
                    children: vec![],
                    depends_on: vec![],
                    created_tick: 0,
                    updated_tick: 0,
                };
                let mut s = state_store.read_state();
                s.goals.insert(goal.id, goal);
                Ok(Response::from_string("Goal added\n"))
            }
            (Method::Post, "/memory/search") => {
                let query = params["query"].as_str().unwrap_or("");
                let emb = crate::llm_chain::embed_sync(query);
                let vm = vec_mem.lock().map_err(|e| e.to_string())?;
                let results = vm.search(&emb, 5);
                let json = serde_json::to_string_pretty(&results).unwrap_or_default();
                let header = Header::from_bytes("Content-Type", "application/json; charset=utf-8").unwrap();
                Ok(Response::from_data(json.into_bytes()).with_header(header))
            }
            (Method::Get, "/resource/stats") => {
                Ok(Response::from_string(ResourceMonitor::stats()))
            }
            _ => Ok(Response::from_string("SPS API ready\n")),
        };

        match result {
            Ok(response) => request.respond(response)?,
            Err(e) => {
                request.respond(Response::from_string(format!("Error: {}\n", e)).with_status_code(StatusCode(500)))?;
            }
        }
    }
    Ok(())
}
