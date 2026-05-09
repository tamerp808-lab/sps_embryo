use std::collections::BTreeMap;

// =====================================================
// INTENT STATE
// =====================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum IntentState {

    Pending,

    Resolved,

    Planned,

    Rejected,
}

// =====================================================
// INTENT
// =====================================================

/// deterministic cognitive intent.
///
/// transforms high-level goals into:
/// replay-safe execution semantics.
#[derive(Debug, Clone)]
pub struct Intent {

    /// deterministic intent id
    pub intent_id: u64,

    /// owning workflow

    /// high-level objective
    pub goal: String,

    /// resolved meaning
    pub resolved_action: String,

    /// intent lifecycle
    pub state: IntentState,

    /// logical creation tick
    pub created_tick: u64,
}

// =====================================================
// INTENT ENGINE
// =====================================================

#[derive(Debug)]
pub struct IntentEngine {

    intents:
        BTreeMap<u64, Intent>,
}

// =====================================================
// IMPLEMENTATION
// =====================================================

impl IntentEngine {

    // =============================================
    // CONSTRUCTOR
    // =============================================

    pub fn new() -> Self {

        Self {

            intents:
                BTreeMap::new(),
        }
    }

    // =============================================
    // REGISTER INTENT
    // =============================================

    pub fn register_intent(

        &mut self,

        intent: Intent,
    ) {

        self.intents.insert(

            intent.intent_id,

            intent,
        );
    }

    // =============================================
    // GET INTENT
    // =============================================

    pub fn intent(

        &self,

        intent_id: u64,
    ) -> Intent {

        self.intents
            .get(&intent_id)
            .expect(
                "intent missing"
            )
            .clone()
    }

    // =============================================
    // TRANSITION
    // =============================================

    pub fn transition(

        &mut self,

        intent_id: u64,

        next: IntentState,
    ) {

        let intent =
            self.intents
                .get_mut(&intent_id)
                .expect(
                    "intent missing"
                );

        intent.state = next;
    }

    // =============================================
    // TOTAL INTENTS
    // =============================================

    pub fn total_intents(
        &self,
    ) -> usize {

        self.intents.len()
    }
}
