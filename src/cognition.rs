use crate::memory::MemoryGraph;

use std::collections::BTreeMap;

// =====================================================
// THOUGHT STATE
// =====================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum ThoughtState {

    Pending,

    Active,

    Completed,

    Failed,
}

// =====================================================
// THOUGHT FRAME
// =====================================================

#[derive(Debug, Clone)]
pub struct ThoughtFrame {

    pub thought_id: u64,


    pub reasoning: String,

    pub state: ThoughtState,

    pub created_tick: u64,
}

// =====================================================
// COGNITIVE RUNTIME
// =====================================================

#[derive(Debug)]
pub struct CognitiveRuntime {

    thoughts:
        BTreeMap<u64, ThoughtFrame>,

    // =============================================
    // MEMORY GRAPH
    // =============================================

    pub memory_graph:
        MemoryGraph,
}

// =====================================================
// IMPLEMENTATION
// =====================================================

impl CognitiveRuntime {

    // =============================================
    // CONSTRUCTOR
    // =============================================

    pub fn new() -> Self {

        Self {

            thoughts:
                BTreeMap::new(),

            memory_graph:
                MemoryGraph::new(),
        }
    }

    // =============================================
    // REGISTER THOUGHT
    // =============================================

    pub fn register_thought(

        &mut self,

        thought: ThoughtFrame,
    ) {

        self.thoughts.insert(

            thought.thought_id,

            thought,
        );
    }

    // =============================================
    // TRANSITION
    // =============================================

    pub fn transition(

        &mut self,

        thought_id: u64,

        new_state:
            ThoughtState,
    ) {

        let thought =
            self.thoughts
                .get_mut(
                    &thought_id
                )
                .expect(
                    "thought missing"
                );

        thought.state =
            new_state;
    }

    // =============================================
    // GET THOUGHT
    // =============================================

    pub fn thought(

        &self,

        thought_id: u64,
    ) -> ThoughtFrame {

        self.thoughts
            .get(
                &thought_id
            )
            .expect(
                "thought missing"
            )
            .clone()
    }

    // =============================================
    // TOTAL THOUGHTS
    // =============================================

    pub fn total_thoughts(
        &self,
    ) -> usize {

        self.thoughts.len()
    }
}
