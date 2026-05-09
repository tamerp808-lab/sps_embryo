use std::collections::BTreeMap;

// =====================================================
// MEMORY TYPE
// =====================================================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Working,
}

// =====================================================
// MEMORY RECORD
// =====================================================

/// replay-safe cognitive memory.
///
/// MUST be:
/// - deterministic
/// - immutable once recorded
/// - auditable
#[derive(Debug, Clone)]
pub struct MemoryRecord {
    /// deterministic memory id
    pub memory_id: u64,

    /// memory category
    pub memory_type: MemoryType,

    /// memory content
    pub content: String,

    /// logical creation tick
    pub created_tick: u64,
}

// =====================================================
// MEMORY GRAPH
// =====================================================

/// deterministic cognitive memory graph.
#[derive(Debug)]
pub struct MemoryGraph {
    memories: BTreeMap<u64, MemoryRecord>,
}

// =====================================================
// IMPLEMENTATION
// =====================================================

impl MemoryGraph {
    // =============================================
    // CONSTRUCTOR
    // =============================================

    pub fn new() -> Self {
        Self {
            memories: BTreeMap::new(),
        }
    }

    // =============================================
    // STORE MEMORY
    // =============================================

    pub fn store(&mut self, memory: MemoryRecord) {
        self.memories.insert(memory.memory_id, memory);
    }

    // =============================================
    // GET MEMORY
    // =============================================

    pub fn get(&self, memory_id: u64) -> MemoryRecord {
        self.memories
            .get(&memory_id)
            .expect("memory missing")
            .clone()
    }

    // =============================================
    // TOTAL MEMORIES
    // =============================================

    pub fn total_memories(&self) -> usize {
        self.memories.len()
    }
}
