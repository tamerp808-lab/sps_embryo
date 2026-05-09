use crate::outbox::{EffectOutbox, EffectRequest, EffectType};

pub struct IsolatedProxy;

impl IsolatedProxy {
    pub fn execute_agent_code(code: &str, outbox: &mut EffectOutbox) {
        let tmp = format!("/tmp/sps_agent_{}.py", uuid::Uuid::now_v7());
        let _ = std::fs::write(&tmp, code);
        let cmd = format!("python3 {} 2>&1; rm {}", tmp, tmp);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
}
