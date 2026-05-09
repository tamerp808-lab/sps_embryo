#![allow(dead_code)]

use std::io::{Read, Write};
use std::net::TcpStream;

/// يُرسل طلب اكتشاف إلى نظير ويُعيد رد الخادم.
pub fn discover_peer(peer_addr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!(
        "GET /peers HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        peer_addr
    );
    let mut stream = TcpStream::connect(peer_addr)?;
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

/// يُخبر نظيرًا بأن هذه العقدة حية وتملك أقسامًا معينة.
pub fn announce_to_peer(
    peer_addr: &str,
    node_id: u64,
    partitions: &[u64],
) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::json!({
        "node_id": node_id,
        "partitions": partitions,
        "action": "announce"
    })
    .to_string();

    let request = format!(
        "POST /peers HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        peer_addr,
        body.len(),
        body
    );
    let mut stream = TcpStream::connect(peer_addr)?;
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    Ok(())
}
