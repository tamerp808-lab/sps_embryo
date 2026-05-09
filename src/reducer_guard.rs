use crate::event::Event;
use crate::state::AppState;

// =====================================================
// REDUCER GUARD
// =====================================================

/// reducer invariants.
///
/// أي reducer يكسر هذه القوانين:
/// runtime violation.
///
/// هذا بداية:
/// runtime constitution enforcement.
pub fn validate_transition(
    previous: &AppState,
    next: &AppState,
    event: &Event,
) -> Result<(), String> {

    // =============================================
    // COUNTER MUST INCREASE
    // =============================================

    if next.counter
        != previous.counter + 1
    {
        return Err(
            format!(
                "invalid counter transition at event {}",
                event.id
            )
        );
    }

    // =============================================
    // LOGICAL CLOCK MUST BE MONOTONIC
    // =============================================

    if next.last_tick
        < previous.last_tick
    {
        return Err(
            format!(
                "logical clock regression at event {}",
                event.id
            )
        );
    }

    // =============================================
    // EVENT TICK MUST MATCH STATE
    // =============================================

    if next.last_tick
        != event.tick
    {
        return Err(
            format!(
                "state tick mismatch at event {}",
                event.id
            )
        );
    }

    // =============================================
    // HISTORY MUST GROW
    // =============================================

    if next.history.len()
        != previous.history.len() + 1
    {
        return Err(
            format!(
                "history corruption at event {}",
                event.id
            )
        );
    }

    Ok(())
}
