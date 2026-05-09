use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use crate::event::Event;

#[derive(Debug, Clone)]
pub struct JobState {
    pub id: String,
    pub name: String,
    pub status: String,
    pub result: Option<String>,
    pub created_tick: u64,
    pub updated_tick: u64,
}

pub struct JobStore {
    jobs: RwLock<BTreeMap<String, JobState>>,
}

impl JobStore {
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(BTreeMap::new()),
        }
    }

    /// المخفض الرسمي لتحديث حالة الوظائف
    pub fn apply_event(&self, event: &Event) {
        let mut jobs = self.jobs.write().unwrap();
        match event.event_type.as_str() {
            "JobSubmitted" => {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload) {
                    let id = payload["job_id"].as_str().unwrap_or("").to_string();
                    let name = payload["name"].as_str().unwrap_or("").to_string();
                    jobs.insert(id.clone(), JobState {
                        id,
                        name,
                        status: "Queued".into(),
                        result: None,
                        created_tick: event.tick,
                        updated_tick: event.tick,
                    });
                }
            }
            "JobStarted" => {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload) {
                    if let Some(job_id) = payload["job_id"].as_str() {
                        if let Some(job) = jobs.get_mut(job_id) {
                            job.status = "Running".into();
                            job.updated_tick = event.tick;
                        }
                    }
                }
            }
            "JobCompleted" => {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload) {
                    if let Some(job_id) = payload["job_id"].as_str() {
                        if let Some(job) = jobs.get_mut(job_id) {
                            job.status = "Completed".into();
                            job.result = payload["site_id"].as_str().map(|s| s.to_string());
                            job.updated_tick = event.tick;
                        }
                    }
                }
            }
            "JobFailed" => {
                if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&event.payload) {
                    if let Some(job_id) = payload["job_id"].as_str() {
                        if let Some(job) = jobs.get_mut(job_id) {
                            job.status = "Failed".into();
                            job.result = payload["error"].as_str().map(|s| s.to_string());
                            job.updated_tick = event.tick;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_status(&self, job_id: &str) -> Option<JobState> {
        self.jobs.read().unwrap().get(job_id).cloned()
    }
}
