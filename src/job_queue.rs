use std::sync::Arc;
use crossbeam_channel::unbounded;
use crate::agent_visualizer::AgentVisualizer;
use crate::master_architect::MasterArchitect;
use crate::memory_system::LongTermMemory;
use crate::event::Event;
use crate::runtime_dispatcher::RuntimeDispatcher;
use crate::logical_clock::LogicalClock;
use tokio::sync::broadcast;

pub struct WorkerPool {
    sender: crossbeam_channel::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = unbounded::<Box<dyn FnOnce() + Send + 'static>>();
        for _ in 0..size {
            let rx = rx.clone();
            std::thread::spawn(move || { while let Ok(job) = rx.recv() { job(); } });
        }
        Self { sender: tx }
    }
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static { let _ = self.sender.send(Box::new(f)); }
}

pub struct JobQueue {
    pool: Arc<WorkerPool>,
    tx: broadcast::Sender<String>,
    long_mem: Arc<LongTermMemory>,
    dispatcher: Arc<RuntimeDispatcher>,
    clock: Arc<LogicalClock>,
}

impl JobQueue {
    pub fn new(
        tx: broadcast::Sender<String>,
        long_mem: Arc<LongTermMemory>,
        pool: Arc<WorkerPool>,
        dispatcher: Arc<RuntimeDispatcher>,
        clock: Arc<LogicalClock>,
    ) -> Self {
        Self { pool, tx, long_mem, dispatcher, clock }
    }

    pub fn enqueue(&self, name: String, description: String, keywords: String) -> String {
        let job_id = uuid::Uuid::new_v4().to_string();
        let root_id = self.clock.next_id();
        let tick = self.clock.next_tick();

        let event = Event {
            id: root_id, tick,
            event_type: "JobSubmitted".into(),
            payload: serde_json::json!({"job_id":&job_id,"name":&name}).to_string(),
            parent_id: None,
            vector_clock: self.clock.vector_clock_string(),
        };
        self.dispatcher.dispatch(event);

        let pool = self.pool.clone();
        let tx = self.tx.clone();
        let long_mem = self.long_mem.clone();
        let dispatcher = self.dispatcher.clone();
        let clock = self.clock.clone();
        let job_id_clone = job_id.clone();
        let name_clone = name;
        let desc_clone = description;
        let keywords_clone = keywords;

        pool.execute(move || {
            let start_id = clock.next_id();
            let start_tick = clock.next_tick();
            dispatcher.dispatch(Event {
                id: start_id, tick: start_tick,
                event_type: "JobStarted".into(),
                payload: serde_json::json!({"job_id":&job_id_clone}).to_string(),
                parent_id: Some(root_id),
                vector_clock: clock.vector_clock_string(),
            });

            let result = MasterArchitect::build_professional_site(
                &name_clone, &desc_clone, &keywords_clone, &long_mem, &tx,
            );

            let end_id = clock.next_id();
            let end_tick = clock.next_tick();
            let (event_type, payload) = match result {
                Ok(site_id) => ("JobCompleted", serde_json::json!({"job_id":&job_id_clone,"site_id":&site_id})),
                Err(e) => ("JobFailed", serde_json::json!({"job_id":&job_id_clone,"error":&e})),
            };
            dispatcher.dispatch(Event {
                id: end_id, tick: end_tick,
                event_type: event_type.into(),
                payload: payload.to_string(),
                parent_id: Some(start_id),
                vector_clock: clock.vector_clock_string(),
            });
        });

        job_id
    }

    pub fn get_status(&self, _job_id: &str) -> Option<crate::job_store::JobState> { None }
}
