use crate::hash::hash_state;

use crate::replay::replay;

use crate::snapshot::{
    load_snapshot,
    validate_snapshot,
};

use crate::state::AppState;

// =====================================================
// RECOVERY ENGINE
// =====================================================

/// recovery reconstructs runtime truth
/// after crashes or restart.
///
/// recovery MUST:
/// - validate snapshot
/// - replay events
/// - verify hashes
/// - restore canonical state
///
/// if recovery diverges:
/// runtime MUST panic.
pub fn recover_runtime() -> AppState {

    // =============================================
    // LOAD SNAPSHOT
    // =============================================

    let snapshot =
        load_snapshot()
            .expect(
                "failed to load snapshot"
            )
            .expect(
                "snapshot missing"
            );

    // =============================================
    // VALIDATE SNAPSHOT
    // =============================================

    let valid =
        validate_snapshot(
            &snapshot
        );

    if !valid {

        panic!(
            "SNAPSHOT INTEGRITY FAILURE"
        );
    }

    // =============================================
    // REPLAY STATE
    // =============================================

    let replayed_state =
        replay();

    // =============================================
    // HASH VALIDATION
    // =============================================

    let replay_hash =
        hash_state(
            &replayed_state
        );

    if replay_hash
        != snapshot.state_hash
    {

        panic!(
            "RECOVERY DIVERGENCE DETECTED"
        );
    }

    println!(
        "Recovery validation successful."
    );

    replayed_state
}
