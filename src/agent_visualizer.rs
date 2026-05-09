use serde_json::json;
use tokio::sync::broadcast;

pub struct AgentVisualizer;

impl AgentVisualizer {
    pub fn send_agent_state(tx: &broadcast::Sender<String>, agent: &str, state: &str, action: &str) {
        let event = json!({"type":"AgentStateChanged","agent":agent,"state":state,"action":action});
        let _ = tx.send(event.to_string());
    }
    pub fn send_log(tx: &broadcast::Sender<String>, level: &str, message: &str) {
        let event = json!({"type":"LogAdded","level":level,"message":message});
        let _ = tx.send(event.to_string());
    }
}
