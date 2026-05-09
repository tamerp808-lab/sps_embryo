use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

// =====================================================
// RUNTIME MODE
// =====================================================

/// runtime execution mode.
///
/// replay MUST NEVER:
/// - execute effects
/// - invoke LLMs
/// - call APIs
/// - touch external systems
///
/// replay reconstructs truth only.
static REPLAY_MODE:
    AtomicBool =
        AtomicBool::new(false);

// =====================================================
// ENABLE REPLAY MODE
// =====================================================

pub fn enable_replay_mode() {

    REPLAY_MODE.store(
        true,
        Ordering::SeqCst,
    );
}

// =====================================================
// DISABLE REPLAY MODE
// =====================================================

pub fn disable_replay_mode() {

    REPLAY_MODE.store(
        false,
        Ordering::SeqCst,
    );
}

// =====================================================
// CHECK MODE
// =====================================================

pub fn is_replay_mode()
    -> bool
{

    REPLAY_MODE.load(
        Ordering::SeqCst
    )
}
