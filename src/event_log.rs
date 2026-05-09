pub use crate::event::Event;
use crate::signer::{EventSigner, SignedEvent};
use once_cell::sync::Lazy;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventLogError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Signature verification failed")]
    Signature,
}

const EVENT_LOG_FILE: &str = "event_log.jsonl";

static SIGNER: Lazy<Mutex<EventSigner>> = Lazy::new(|| Mutex::new(EventSigner::new()));

pub fn append_event(event: &Event) -> Result<(), EventLogError> {
    let signer = SIGNER.lock().unwrap();
    let signed = signer.sign(event).map_err(|_| EventLogError::Signature)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(EVENT_LOG_FILE)?;
    let line = serde_json::to_string(&signed)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

pub fn read_all_events() -> Result<Vec<Event>, EventLogError> {
    if !Path::new(EVENT_LOG_FILE).exists() {
        return Ok(vec![]);
    }
    let file = fs::File::open(EVENT_LOG_FILE)?;
    let reader = io::BufReader::new(file);
    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let signed: SignedEvent = serde_json::from_str(&line)?;
        if !EventSigner::verify(&signed) {
            return Err(EventLogError::Signature);
        }
        let event: Event = serde_json::from_str(&signed.event_json)?;
        events.push(event);
    }
    events.sort_by(|a, b| a.tick.cmp(&b.tick).then(a.id.cmp(&b.id)));
    Ok(events)
}

pub fn clear_log() -> Result<(), EventLogError> {
    if Path::new(EVENT_LOG_FILE).exists() {
        fs::remove_file(EVENT_LOG_FILE)?;
    }
    Ok(())
}
