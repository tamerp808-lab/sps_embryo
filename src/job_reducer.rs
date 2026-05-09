use std::collections::BTreeMap;

use crate::event::Event;
use crate::job_store::JobState;

pub fn reduce(jobs: &mut BTreeMap<String, JobState>, event: &Event) {
    match event.event_type.as_str() {
        "JobSubmitted" => {
            let payload: serde_json::Value =
                serde_json::from_str(&event.payload).unwrap_or_default();

            let job_id = payload["job_id"].as_str().unwrap_or("").to_string();

            let name = payload["name"].as_str().unwrap_or("").to_string();

            jobs.insert(
                job_id.clone(),
                JobState {
                    id: job_id,
                    name,

                    status: "Queued".to_string(),

                    result: None,


                    created_tick: event.tick,

                    updated_tick: event.tick,
                },
            );
        }

        "JobStarted" => {
            let payload: serde_json::Value =
                serde_json::from_str(&event.payload).unwrap_or_default();

            if let Some(job_id) = payload["job_id"].as_str() {
                if let Some(job) = jobs.get_mut(job_id) {
                    job.status = "Running".to_string();

                    job.updated_tick = event.tick;
                }
            }
        }

        "JobCompleted" => {
            let payload: serde_json::Value =
                serde_json::from_str(&event.payload).unwrap_or_default();

            if let Some(job_id) = payload["job_id"].as_str() {
                if let Some(job) = jobs.get_mut(job_id) {
                    job.status = "Completed".to_string();

                    job.result = payload["site_id"].as_str().map(|s| s.to_string());

                    job.updated_tick = event.tick;
                }
            }
        }

        "JobFailed" => {
            let payload: serde_json::Value =
                serde_json::from_str(&event.payload).unwrap_or_default();

            if let Some(job_id) = payload["job_id"].as_str() {
                if let Some(job) = jobs.get_mut(job_id) {
                    job.status = "Failed".to_string();

                    job.result = payload["error"].as_str().map(|s| s.to_string());

                    job.updated_tick = event.tick;
                }
            }
        }

        _ => {}
    }
}
