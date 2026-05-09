#![allow(dead_code)]

mod event;
mod event_log;
mod state;
mod reducer;
mod hash;
mod snapshot;
mod replay;
mod journal;
mod workflow;
mod outbox;
mod sandbox;
mod execution;
mod scheduler;
mod agent;
mod planner;
mod lease;
mod capabilities;
mod governance;
mod api;
mod partition;
mod consensus;
mod cross_partition;
mod discovery;
mod observability;
mod plugin;
mod audit;

mod voice_agent;
mod browser_agent;
mod ocr_agent;
mod calendar_agent;
mod gmail_agent;
mod self_repair_agent;
mod long_term_planner;
mod rl_scorer;
mod self_update;
mod mobile_agent;
mod coder_agent;
mod vision_agent;
mod swarm_bus;
mod resource_monitor;
mod meta_agent;
mod execution_engine;
mod creator_agent;
mod telegram_bot;
mod database;
mod llm_chain;
mod signer;
mod task_scheduler;

mod auto_capability;
mod auto_lease;
mod event_chain;
mod agent_architect;
mod isolated_proxy;
mod faiss_memory;
mod stealth;
mod auto_installer;
mod ws_server;
mod brain_agent;
mod cognitive_loop;
mod conversation_memory;
mod memory_system;
mod memory_graph;
mod unified_memory;
mod command;
mod command_handler;
mod goal_handler;
mod logical_clock;
mod runtime_guard;
mod tool_registry;
mod autonomous_worker;
mod generation_pipeline;
mod vector_clock;
mod log_compaction;
mod event_chain_verifier;
mod log_encryption;
mod agent_metrics;
mod agent_marketplace;
mod task_pipeline;
mod latency_metrics;
mod cli;
mod peer_discovery;
mod consensus_engine;
mod log_replication;
mod auto_audit;
mod metrics_server;
mod effect_runner;
mod deterministic_scheduler;
mod replay_benchmark;
mod replay_mode;
mod state_store;
mod job_queue;
mod job_store;
mod runtime_dispatcher;
mod job_reducer;
mod master_architect;
mod site_builder;
mod backend_agent;
mod ui_designer;
mod ux_optimizer;
mod seo_agent;
mod api_builder;
mod agent_visualizer;

use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;
use reducer::{ReducerRegistry, core_reducer};
use state_store::StateStore;
use journal::DeliveryJournal;
use partition::PartitionManager;
use consensus::ConsensusState;
use discovery::announce_to_peer;
use api::ApiState;
use plugin::PluginRegistry;
use outbox::EffectOutbox;
use database::Database;
use task_scheduler::TaskScheduler;
use effect_runner::EffectRunner;
use std::time::Duration;
use std::thread;
use tokio::sync::broadcast;
use memory_system::{ShortTermMemory, LongTermMemory, VectorMemory};
use conversation_memory::ConversationMemory;
use memory_graph::CognitiveMemoryGraph;
use unified_memory::UnifiedMemory;
use capabilities::CapabilityToken;
use autonomous_worker::AutonomousWorker;
use job_queue::{JobQueue, WorkerPool};
use job_store::JobStore;
use runtime_dispatcher::RuntimeDispatcher;
use logical_clock::LogicalClock;

fn main() {
    let config_str = std::fs::read_to_string("config.toml")
        .expect("Missing config.toml");
    let config: toml::Value = toml::from_str(&config_str).expect("Invalid config.toml");

    let node_id = config.get("node_id").and_then(|v| v.as_integer()).unwrap_or(1) as u64;
    let port = config.get("port").and_then(|v| v.as_integer()).unwrap_or(9101) as u16;
    let metrics_port = config.get("metrics_port").and_then(|v| v.as_integer()).unwrap_or(port as i64 + 100) as u16;
    let partitions: Vec<u64> = config.get("partitions").and_then(|v| v.as_array())
        .map_or(vec![], |arr| arr.iter().filter_map(|v| v.as_integer()).map(|i| i as u64).collect());
    let peers: Vec<String> = config.get("peers").and_then(|v| v.as_array())
        .map_or(vec![], |arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect());


    let mut registry = ReducerRegistry::new();
    registry.register("core", core_reducer);
    let state_store = Arc::new(StateStore::new());
    let journal = Arc::new(Mutex::new(DeliveryJournal::new()));
    let outbox = Arc::new(Mutex::new(EffectOutbox::new()));

    let outbox_clone = Arc::clone(&outbox);
    thread::spawn(move || loop {
        {
            let mut o = outbox_clone.lock().unwrap();
            let results = EffectRunner::run_all(&mut o);
            for res in results { println!("Effect: {}", res); }
        }
        thread::sleep(Duration::from_secs(1));
    });

    let mut partition_mgr = PartitionManager::new(node_id);
    let mut consensus = ConsensusState::new();
    let mut plugin_registry = PluginRegistry::new();

    let db = Database::open(std::path::PathBuf::from("sps_data.db")).expect("Failed to open database");
    db.save_task("init", "System started", "done", "", 0.0).ok();

    TaskScheduler::start(Duration::from_secs(60), || {
        println!("Resource: {}", resource_monitor::ResourceMonitor::stats());
    });

    plugin_registry.register("CognitiveAgent".into(), "Basic reasoning agent".into());
    plugin_registry.register("PlannerAgent".into(), "LLM-based workflow planner".into());

    for &pid in &partitions {
        consensus.set_leader(pid, node_id);
        partition_mgr.assign_partition(pid, node_id);
    }

    let api_state = ApiState { node_id, partitions: partitions.clone(), peers: BTreeMap::new() };
    for peer_addr in &peers { let _ = announce_to_peer(peer_addr, node_id, &partitions); }

    println!("Node {} with {} plugins starting on port {} (metrics: {})...",
             node_id, plugin_registry.list().len(), port, metrics_port);

    thread::spawn(|| loop {
        stealth::StealthMode::run_cycle();
        thread::sleep(Duration::from_secs(3600));
    });

    auto_installer::check_and_install_tools();


    let (ws_tx, _) = broadcast::channel::<String>(32);
    let ws_rx = ws_tx.subscribe();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            ws_server::start_ws_server(9103, ws_rx).await;
        });
    });
    ws_server::spawn_full_state_broadcast(ws_tx.clone());
    ws_server::set_global_tx(ws_tx.clone());

    println!("WebSocket server on ws://0.0.0.0:9103");

    let short_mem = Arc::new(Mutex::new(ShortTermMemory::new(20)));
    let long_mem = Arc::new(LongTermMemory::open(std::path::PathBuf::from("sps_longterm.db"))
        .expect("Failed to open long-term memory"));
    let vec_mem = Arc::new(Mutex::new(VectorMemory::new()));
    let conv_mem = Arc::new(ConversationMemory::open(std::path::PathBuf::from("sps_conversations.db"))
        .expect("Failed to open conversation memory"));

    let _cognitive_graph = Arc::new(CognitiveMemoryGraph::new(
        Arc::clone(&short_mem),
        Arc::clone(&long_mem),
        Arc::clone(&vec_mem),
        Arc::clone(&conv_mem),
    ));

    let _unified_mem = Arc::new(UnifiedMemory::new(
        Arc::clone(&short_mem),
        Arc::clone(&long_mem),
        Arc::clone(&vec_mem),
        Arc::clone(&conv_mem),
    ));

    let clock = Arc::new(LogicalClock::new(node_id));
    let worker_pool = Arc::new(WorkerPool::new(4));
    let job_store = Arc::new(JobStore::new());
    let dispatcher = Arc::new(RuntimeDispatcher::new(Arc::clone(&job_store)));
    let job_queue = Arc::new(JobQueue::new(ws_tx.clone(), Arc::clone(&long_mem), Arc::clone(&worker_pool), Arc::clone(&dispatcher), Arc::clone(&clock)));

    AutonomousWorker::start(
        Arc::clone(&state_store),
        Arc::clone(&short_mem),
        Arc::clone(&long_mem),
        Arc::clone(&vec_mem),
        Arc::clone(&conv_mem),
        Arc::clone(&outbox),
        Duration::from_secs(30),
    );

    let lease_mgr = Arc::new(Mutex::new(lease::LeaseManager::new()));
    let token = Arc::new(CapabilityToken::root());

    if let Err(e) = runtime_guard::RuntimeGuard::verify() {
        eprintln!("Runtime guard failed: {}", e);
        std::process::exit(1);
    }
    println!("✅ Runtime guard passed.");

    let live_hash = {
        let s = state_store.read_state();
        crate::hash::hash_state(&s)
    };
    let engine = replay::ReplayEngine::new().expect("Failed to create replay engine");
    if !engine.validate_determinism(&live_hash) {
        eprintln!("CRITICAL: Determinism violation! Replay != Live");
        std::process::exit(1);
    }
    println!("✅ Replay verification passed. Determinism confirmed.");

    let server_state = Arc::clone(&state_store);
    let server_journal = Arc::clone(&journal);
    let server_outbox = Arc::clone(&outbox);
    let server_lease_mgr = Arc::clone(&lease_mgr);
    let server_short_mem = Arc::clone(&short_mem);
    let server_long_mem = Arc::clone(&long_mem);
    let server_vec_mem = Arc::clone(&vec_mem);
    let server_conv_mem = Arc::clone(&conv_mem);

    thread::spawn(move || {
        if let Err(e) = api::start_api_server(
            server_state,
            server_journal,
            Arc::new(registry),
            port,
            Arc::new(Mutex::new(partition_mgr)),
            Arc::new(Mutex::new(api_state)),
            Arc::new(Mutex::new(plugin_registry)),
            server_outbox,
            server_lease_mgr,
            token,
            server_conv_mem,
            server_short_mem,
            server_long_mem,
            server_vec_mem,
            Arc::clone(&job_queue),
        ) {
            eprintln!("API server error: {}", e);
        }
    });

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
