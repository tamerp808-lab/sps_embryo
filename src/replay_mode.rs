use std::sync::atomic::{AtomicBool, Ordering};

static REPLAY_MODE: AtomicBool = AtomicBool::new(false);

pub fn is_replay_mode() -> bool {
    REPLAY_MODE.load(Ordering::SeqCst)
}

pub fn enable_replay_mode() {
    REPLAY_MODE.store(true, Ordering::SeqCst);
}

pub fn disable_replay_mode() {
    REPLAY_MODE.store(false, Ordering::SeqCst);
}
