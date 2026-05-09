#[cfg(test)]
mod tests {
    use crate::reducer::{ReducerRegistry, core_reducer, idempotent_reduce};
    use crate::state::AppState;
    use crate::event::Event;
    use crate::journal::DeliveryJournal;

    #[test]
    fn test_determinism() {
        let mut registry = ReducerRegistry::new();
        registry.register("core", core_reducer);
        let state1 = AppState::default();
        let state2 = AppState::default();
        let mut journal = DeliveryJournal::new();
        let r1 = idempotent_reduce(state1, &event, &mut journal);
        let r2 = idempotent_reduce(state2, &event, &mut journal);
        assert_eq!(r1.counter, r2.counter);
    }
}
