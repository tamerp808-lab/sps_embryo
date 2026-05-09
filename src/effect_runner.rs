use crate::outbox::EffectOutbox;
use crate::sandbox;

/// يُنفذ جميع الآثار المعلقة في الـ Outbox (فقط في وضع Live)
pub struct EffectRunner;

impl EffectRunner {
    pub fn run_all(outbox: &mut EffectOutbox) -> Vec<String> {
        let mut results = Vec::new();
        let pending_ids = outbox.pending_ids();
        for id in pending_ids {
            if let Some(effect) = outbox.complete(id) {
                let result = sandbox::execute_effect(&effect);
                results.push(result);
            }
        }
        results
    }
}
