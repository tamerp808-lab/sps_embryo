#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tiny_http::{Response, Server};

pub struct Metrics {
    pub events_processed: AtomicU64,
    pub api_requests: AtomicU64,
    pub replays_count: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            events_processed: AtomicU64::new(0),
            api_requests: AtomicU64::new(0),
            replays_count: AtomicU64::new(0),
        }
    }
}

pub fn start_metrics_server(
    metrics: Arc<Metrics>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)?;
    println!("Metrics server on http://{}", addr);
    for request in server.incoming_requests() {
        let events = metrics.events_processed.load(Ordering::Relaxed);
        let apis = metrics.api_requests.load(Ordering::Relaxed);
        let replays = metrics.replays_count.load(Ordering::Relaxed);
        let json = serde_json::json!({
            "events_processed": events,
            "api_requests": apis,
            "replays_count": replays
        });
        request.respond(Response::from_string(serde_json::to_string_pretty(&json)?))?;
    }
    Ok(())
}
