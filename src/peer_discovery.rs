use std::collections::BTreeMap;
use std::net::UdpSocket;
use std::sync::Mutex;

pub struct PeerDiscovery {
    peers: Mutex<BTreeMap<u64, String>>, // node_id -> address
    local_node_id: u64,
    local_address: String,
}

impl PeerDiscovery {
    pub fn new(node_id: u64, address: &str) -> Self {
        Self {
            peers: Mutex::new(BTreeMap::new()),
            local_node_id: node_id,
            local_address: address.to_string(),
        }
    }

    /// أضف نظيرًا
    pub fn add_peer(&self, node_id: u64, address: &str) {
        let mut peers = self.peers.lock().unwrap();
        peers.insert(node_id, address.to_string());
    }

    /// أزل نظيرًا
    pub fn remove_peer(&self, node_id: u64) {
        let mut peers = self.peers.lock().unwrap();
        peers.remove(&node_id);
    }

    /// ابدأ بث رسائل الاكتشاف بشكل دوري
    pub fn start_broadcast(&self, port: u16) {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind discovery socket");
        socket.set_broadcast(true).ok();
        let local = self.local_address.clone();
        let node_id = self.local_node_id;

        std::thread::spawn(move || loop {
            let message = format!("SPS_DISCOVER:{}:{}", node_id, local);
            if let Err(e) = socket.send_to(message.as_bytes(), format!("255.255.255.255:{}", port))
            {
                eprintln!("Discovery broadcast error: {}", e);
            }
            std::thread::sleep(std::time::Duration::from_secs(5));
        });
    }

    /// استمع للإعلانات من الأقران الآخرين
    pub fn start_listener(&self, port: u16) {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
            .expect("Failed to bind discovery listener");

        std::thread::spawn(move || loop {
            let mut buf = [0u8; 1024];
            if let Ok((size, src)) = socket.recv_from(&mut buf) {
                let msg = String::from_utf8_lossy(&buf[..size]);
                if msg.starts_with("SPS_DISCOVER:") {
                    let parts: Vec<&str> = msg.split(':').collect();
                    if parts.len() >= 3 {
                        if let Ok(node_id) = parts[1].parse::<u64>() {
                            let address = parts[2].to_string();
                            println!(
                                "🔍 Discovered peer: Node {} at {} (from {})",
                                node_id, address, src
                            );
                            // هنا يمكن إضافة المنطق الخاص بتحديث قائمة الأقران
                        }
                    }
                }
            }
        });
    }

    /// قائمة الأقران الحالية
    pub fn list_peers(&self) -> Vec<(u64, String)> {
        self.peers
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}
