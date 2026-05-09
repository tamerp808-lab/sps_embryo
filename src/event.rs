use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: u64,
    pub tick: u64,
    pub event_type: String,
    pub payload: String,
    pub parent_id: Option<u64>,
    pub vector_clock: String,
}
