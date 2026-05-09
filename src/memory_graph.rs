use crate::conversation_memory::ConversationMemory;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

pub struct CognitiveMemoryGraph {
    pub short_term: Arc<Mutex<ShortTermMemory>>,
    pub long_term: Arc<LongTermMemory>,
    pub vector: Arc<Mutex<VectorMemory>>,
    pub conversation: Arc<ConversationMemory>,
    pub semantic: Mutex<BTreeMap<String, Vec<String>>>,
    pub episodic: Mutex<Vec<String>>,
}

impl CognitiveMemoryGraph {
    pub fn new(
        short_term: Arc<Mutex<ShortTermMemory>>,
        long_term: Arc<LongTermMemory>,
        vector: Arc<Mutex<VectorMemory>>,
        conversation: Arc<ConversationMemory>,
    ) -> Self {
        Self {
            short_term,
            long_term,
            vector,
            conversation,
            semantic: Mutex::new(BTreeMap::new()),
            episodic: Mutex::new(Vec::new()),
        }
    }

    pub fn add_semantic_relation(&self, subject: &str, predicate: &str, object: &str) {
        let mut sem = self.semantic.lock().unwrap();
        sem.entry(subject.to_string())
            .or_default()
            .push(format!("{} {}", predicate, object));
    }

    pub fn record_episode(&self, description: &str) {
        let mut epi = self.episodic.lock().unwrap();
        epi.push(description.to_string());
        if epi.len() > 1000 {
            epi.remove(0);
        }
    }

    pub fn recall_episodes(&self, limit: usize) -> Vec<String> {
        let epi = self.episodic.lock().unwrap();
        epi.iter().rev().take(limit).cloned().collect()
    }
}

/// رسم معرفي بسيط (عقد وعلاقات)
pub struct KnowledgeGraph {
    nodes: Mutex<BTreeMap<String, Vec<String>>>,
    edges: Mutex<Vec<(String, String, String)>>, // (from, to, relation)
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Mutex::new(BTreeMap::new()),
            edges: Mutex::new(Vec::new()),
        }
    }

    pub fn add_node(&self, id: &str, properties: Vec<String>) {
        let mut nodes = self.nodes.lock().unwrap();
        nodes.insert(id.to_string(), properties);
    }

    pub fn add_edge(&self, from: &str, to: &str, relation: &str) {
        let mut edges = self.edges.lock().unwrap();
        edges.push((from.to_string(), to.to_string(), relation.to_string()));
    }

    pub fn query(&self, node_id: &str) -> Vec<(String, String)> {
        let edges = self.edges.lock().unwrap();
        edges
            .iter()
            .filter(|(from, to, _)| from == node_id || to == node_id)
            .map(|(from, to, rel)| {
                if from == node_id {
                    (to.clone(), rel.clone())
                } else {
                    (from.clone(), rel.clone())
                }
            })
            .collect()
    }
}
