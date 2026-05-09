#![allow(dead_code)]

use crate::event_log::{read_all_events, Event};
use crate::journal::DeliveryJournal;
use crate::reducer::idempotent_reduce;
use crate::state::AppState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditEntry {
    pub event: Event,
    pub prev_hash: Option<String>,
    pub current_state_hash: String,
}

#[derive(Debug, Serialize)]
pub struct ChainVerificationResult {
    pub total_events: usize,
    pub chain_valid: bool,
    pub final_state_hash: String,
    pub entries: Vec<AuditEntry>,
}

pub fn build_audit_chain() -> Result<ChainVerificationResult, Box<dyn std::error::Error>> {
    let events = read_all_events()?;
    let mut state = AppState::default();
    let mut journal = DeliveryJournal::new();
    let mut entries = Vec::new();
    let mut prev_hash: Option<String> = None;

    for event in &events {
        state = idempotent_reduce(state, event, &mut journal);
        let current_hash = blake3::hash(serde_json::to_string(&state)?.as_bytes())
            .to_hex()
            .to_string();
        entries.push(AuditEntry {
            event: event.clone(),
            prev_hash: prev_hash.clone(),
            current_state_hash: current_hash.clone(),
        });
        prev_hash = Some(current_hash);
    }

    let valid = verify_chain_internal(&entries);
    Ok(ChainVerificationResult {
        total_events: events.len(),
        chain_valid: valid,
        final_state_hash: entries
            .last()
            .map(|e| e.current_state_hash.clone())
            .unwrap_or_default(),
        entries,
    })
}

fn verify_chain_internal(entries: &[AuditEntry]) -> bool {
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            let expected_prev = &entries[i - 1].current_state_hash;
            if entry.prev_hash.as_ref() != Some(expected_prev) {
                return false;
            }
        }
    }
    true
}

pub fn forensic_trace(
    event_id: u64,
) -> Result<Option<Vec<AuditEntry>>, Box<dyn std::error::Error>> {
    let events = read_all_events()?;
    let target_idx = events.iter().position(|e| e.id == event_id);
    if target_idx.is_none() {
        return Ok(None);
    }
    let idx = target_idx.unwrap();

    let mut state = AppState::default();
    let mut journal = DeliveryJournal::new();
    let mut entries: Vec<AuditEntry> = Vec::new();

    for (i, event) in events.iter().enumerate() {
        state = idempotent_reduce(state, event, &mut journal);
        if i <= idx {
            let current_hash = blake3::hash(serde_json::to_string(&state)?.as_bytes())
                .to_hex()
                .to_string();
            entries.push(AuditEntry {
                event: event.clone(),
                prev_hash: if i > 0 {
                    Some(entries[i - 1].current_state_hash.clone())
                } else {
                    None
                },
                current_state_hash: current_hash,
            });
        } else {
            break;
        }
    }
    Ok(Some(entries))
}
