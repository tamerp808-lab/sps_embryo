use crate::event::Event;
use crate::lease::LeaseManager;
use crate::capabilities::{CapabilityToken, Capability};

pub struct GovernanceAgent {
    pub agent_id: u64,
}

impl GovernanceAgent {
    pub fn new(agent_id: u64) -> Self {
        Self { agent_id }
    }

    pub fn review_event(
        &self,
        event: &Event,
        token: &CapabilityToken,
        lease_mgr: &mut LeaseManager,
        current_tick: u64,
    ) -> Result<(), String> {
        // التحقق من الصلاحيات
        if event.payload.starts_with("WF:") {
            if !token.has(&Capability::WorkflowDispatch) {
                return Err("Missing capability: WorkflowDispatch".to_string());
            }
        }
        if event.payload.starts_with("EXEC:") {
            if !token.has(&Capability::EffectInvoke) {
                return Err("Missing capability: EffectInvoke".to_string());
            }
        }

        // تنظيف العُقد المنتهية
        lease_mgr.revoke_expired(current_tick);

        Ok(())
    }
}
