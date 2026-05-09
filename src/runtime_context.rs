use crate::capabilities::CapabilityToken;
use crate::workflow::WorkflowRuntime;
use crate::cognition::CognitiveRuntime;
use crate::memory::MemoryGraph;
use crate::planner::PlannerRuntime;
use crate::intent::IntentEngine;
use crate::outbox::EffectOutbox;
use crate::lease::LeaseManager;
use crate::scheduler::DeterministicScheduler;
use crate::journal::DeliveryJournal;

#[derive(Debug)]
pub struct RuntimeContext {
    pub capability_token: CapabilityToken,
    pub workflow_runtime: WorkflowRuntime,
    pub cognition_runtime: CognitiveRuntime,
    pub memory_graph: MemoryGraph,
    pub planner_runtime: PlannerRuntime,
    pub intent_runtime: IntentEngine,
    pub outbox: EffectOutbox,
    pub lease_manager: LeaseManager,
    pub scheduler: DeterministicScheduler,
    pub journal: DeliveryJournal,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            capability_token: CapabilityToken::root(),
            workflow_runtime: WorkflowRuntime::new(),
            cognition_runtime: CognitiveRuntime::new(),
            memory_graph: MemoryGraph::new(),
            planner_runtime: PlannerRuntime::new(),
            intent_runtime: IntentEngine::new(),
            outbox: EffectOutbox::new(),
            lease_manager: LeaseManager::new(),
            scheduler: DeterministicScheduler::new(),
            journal: DeliveryJournal::new(),
        }
    }
}
