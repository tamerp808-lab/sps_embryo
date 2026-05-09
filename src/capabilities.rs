use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Capability {
    WorkflowDispatch,
    WorkflowExecute,
    MemoryRead,
    MemoryWrite,
    SnapshotCreate,
    ReplayRun,
    EffectInvoke,
    VoiceControl,
    MobileControl,
    BrowserControl,
    CoderControl,
    GmailControl,
    CalendarControl,
    AdminAccess,
}

#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub agent_id: u64,
    capabilities: BTreeSet<Capability>,
}

impl CapabilityToken {
    pub fn new(agent_id: u64) -> Self {
        Self {
            agent_id,
            capabilities: BTreeSet::new(),
        }
    }

    pub fn root() -> Self {
        let mut token = Self::new(0);
        token.grant(Capability::WorkflowDispatch);
        token.grant(Capability::WorkflowExecute);
        token.grant(Capability::MemoryRead);
        token.grant(Capability::MemoryWrite);
        token.grant(Capability::SnapshotCreate);
        token.grant(Capability::ReplayRun);
        token.grant(Capability::EffectInvoke);
        token.grant(Capability::VoiceControl);
        token.grant(Capability::MobileControl);
        token.grant(Capability::BrowserControl);
        token.grant(Capability::CoderControl);
        token.grant(Capability::GmailControl);
        token.grant(Capability::CalendarControl);
        token.grant(Capability::AdminAccess);
        token
    }

    pub fn grant(&mut self, cap: Capability) {
        self.capabilities.insert(cap);
    }

    pub fn has(&self, cap: &Capability) -> bool {
        self.capabilities.contains(cap)
    }
}
