use futures_util::StreamExt;
use futures_util::SinkExt;
use tokio::sync::broadcast;
use std::time::Duration;
use serde_json::json;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static GLOBAL_TX: Lazy<Mutex<Option<broadcast::Sender<String>>>> = Lazy::new(|| Mutex::new(None));

pub fn set_global_tx(tx: broadcast::Sender<String>) {
    *GLOBAL_TX.lock().unwrap() = Some(tx);
}

pub fn get_broadcast_tx() -> broadcast::Sender<String> {
    GLOBAL_TX.lock().unwrap().as_ref().unwrap().clone()
}

pub async fn start_ws_server(port: u16, rx: broadcast::Receiver<String>) {
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("WebSocket server on ws://0.0.0.0:{}", port);
    while let Ok((stream, _)) = listener.accept().await {
        let ws = tokio_tungstenite::accept_async(stream).await.unwrap();
        let (mut sender, _) = ws.split();
        let mut local_rx = rx.resubscribe();
        tokio::spawn(async move {
            while let Ok(msg) = local_rx.recv().await {
                if sender.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await.is_err() { break; }
            }
        });
    }
}

/// بث الحالة الكاملة عبر std::thread (لا تحتاج إلى Tokio runtime)
pub fn spawn_full_state_broadcast(tx: broadcast::Sender<String>) {
    std::thread::spawn(move || loop {
        let state = json!({
            "type": "FullState",
            "agents": {"coder": "running", "planner": "thinking", "repair": "idle", "ux": "idle", "seo": "idle"},
            "metrics": {"cpu": "32%", "ram_free_mb": 450, "battery": 78, "events_per_sec": 14.2, "uptime_sec": 3600},
            "replay": {"hash_match": true, "last_events": 42, "snapshots": 3, "total_events": 1024},
            "consensus": {"node_id": 1, "leader": 1, "partitions": [1,2], "health": "healthy"},
            "tasks": {"queued": 2, "running": 1, "completed": 14, "failed": 0},
            "swarm": {"pending": 3, "routes": 5},
            "logs": ["[INFO] Runtime stable", "[INFO] WebSocket streaming active"]
        });
        let _ = tx.send(state.to_string());
        std::thread::sleep(Duration::from_secs(2));
    });
}
