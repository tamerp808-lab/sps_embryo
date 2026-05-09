use crate::state::AppState;
use std::sync::{Arc, Mutex};

pub struct TaskPipeline;

impl TaskPipeline {
    pub fn run_sequential(_goals: Vec<u64>, _state: Arc<Mutex<AppState>>) {
        // TODO: تنفيذ الأهداف عبر CognitiveLoop
    }
}
