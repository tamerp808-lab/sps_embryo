#![allow(dead_code)]

use crate::event::Event;
use std::io::Write;
use std::net::TcpStream;

/// يُرسل حدثًا إلى عقدة أخرى (إلى قسم آخر) عبر HTTP.
pub fn send_event_cross_partition(
    target_node: &str, // e.g. "127.0.0.1:9102"
    event: &Event,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(event)?;
    let request = format!(
        "POST /cross_partition HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        target_node,
        body.len(),
        body
    );
    let mut stream = TcpStream::connect(target_node)?;
    stream.write_all(request.as_bytes())?;
    stream.flush()?;
    Ok(())
}
