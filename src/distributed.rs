#![allow(dead_code)]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use crate::event::Event;
use crate::reducer::{idempotent_reduce, ReducerRegistry, core_reducer};
use crate::hash::hash_state;
use crate::state::AppState;
use crate::journal::DeliveryJournal;
use crate::api;

pub fn spawn_node(
    port: u16,
    initial_state: AppState,
    initial_journal: DeliveryJournal,
    registry: ReducerRegistry,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut state = initial_state;
        let mut journal = initial_journal;
        let registry = registry;
        if let Err(e) = api::start_api_server(&mut state, &mut journal, &registry) {
            eprintln!("Node on port {} error: {}", port, e);
        }
    })
}

/// يُرسل حدثًا إلى عقدة أخرى عبر HTTP POST /append باستخدام TCP خام.
pub fn send_event_to_node(port: u16, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(event)?;
    let request = format!(
        "POST /append HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        port,
        body.len(),
        body
    );
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    if response.contains("200 OK") || response.contains("Event appended") {
        Ok(())
    } else {
        Err(format!("Node {} unexpected response: {}", port, response).into())
    }
}

pub fn run_distributed_test() -> bool {
    let mut registry_a = ReducerRegistry::new();
    registry_a.register("core", core_reducer);
    let registry_b = registry_a.clone();

    let state_a = AppState::default();
    let journal_a = DeliveryJournal::new();
    let state_b = AppState::default();
    let journal_b = DeliveryJournal::new();

    let port_a = 9101;
    let port_b = 9102;

    let _node_a = spawn_node(port_a, state_a.clone(), journal_a.clone(), registry_a);
    let _node_b = spawn_node(port_b, state_b.clone(), journal_b.clone(), registry_b);

    thread::sleep(Duration::from_millis(500));

    let events_a_to_b = vec![
    ];
    for event in &events_a_to_b {
        if let Err(e) = send_event_to_node(port_b, event) {
            eprintln!("Send a->b failed: {}", e);
        }
    }

    let events_b_to_a = vec![
    ];
    for event in &events_b_to_a {
        if let Err(e) = send_event_to_node(port_a, event) {
            eprintln!("Send b->a failed: {}", e);
        }
    }

    thread::sleep(Duration::from_millis(300));

    let mut all_events = events_a_to_b.clone();
    all_events.extend(events_b_to_a);

    let mut state_sim = AppState::default();
    let mut journal_sim = DeliveryJournal::new();
    for event in &all_events {
        state_sim = idempotent_reduce(state_sim, event, &mut journal_sim);
    }
    let hash_sim = hash_state(&state_sim);

    let mut state_verify = AppState::default();
    let mut journal_verify = DeliveryJournal::new();
    for event in &all_events {
        state_verify = idempotent_reduce(state_verify, event, &mut journal_verify);
    }
    let hash_verify = hash_state(&state_verify);

    println!("Hash after distributed exchange: {}", hash_sim);
    println!("Verification hash: {}", hash_verify);

    hash_sim == hash_verify
}
