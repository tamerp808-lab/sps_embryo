use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static BUS: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static REPLIES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub struct SwarmBus;

impl SwarmBus {
    pub fn send(target: &str, message: &str) -> Result<String, String> {
        let mut bus = BUS.lock().map_err(|e| e.to_string())?;
        bus.entry(target.to_string())
            .or_default()
            .push(message.to_string());
        Ok(format!("Message sent to {}", target))
    }

    pub fn request(target: &str, payload: &str, timeout_ms: u64) -> Result<String, String> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let full_msg = format!("{}||{}", request_id, payload);
        let mut bus = BUS.lock().map_err(|e| e.to_string())?;
        bus.entry(target.to_string()).or_default().push(full_msg);
        drop(bus);

        let start = std::time::Instant::now();
        loop {
            if let Ok(replies) = REPLIES.lock() {
                if let Some(reply) = replies.get(&request_id) {
                    return Ok(reply.clone());
                }
            }
            if start.elapsed().as_millis() > timeout_ms as u128 {
                return Err("Request timed out".to_string());
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn reply(request_id: &str, message: &str) -> Result<(), String> {
        let mut replies = REPLIES.lock().map_err(|e| e.to_string())?;
        replies.insert(request_id.to_string(), message.to_string());
        Ok(())
    }

    pub fn list_agents() -> Vec<String> {
        BUS.lock()
            .map(|b| b.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn fetch_messages(agent: &str) -> Vec<String> {
        let mut bus = BUS.lock().unwrap();
        bus.remove(agent).unwrap_or_default()
    }
}
