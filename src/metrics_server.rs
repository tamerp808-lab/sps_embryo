use tiny_http::{Response, Server};

pub fn start_metrics_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr)?;
    println!("Metrics server on http://{}", addr);

    for request in server.incoming_requests() {
        let metrics = r#"# HELP sps_events_total Total events processed
# TYPE sps_events_total counter
sps_events_total 1024
# HELP sps_uptime_seconds System uptime
# TYPE sps_uptime_seconds gauge
sps_uptime_seconds 3600
# HELP sps_agent_count Agent count
# TYPE sps_agent_count gauge
sps_agent_count 6
"#;
        request.respond(Response::from_string(metrics.to_string()))?;
    }
    Ok(())
}
