use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone)]
pub struct ConsensusEngine {
    pub node_id: u64,
    pub state: NodeState,
    pub current_term: u64,
    pub voted_for: Option<u64>,
    pub leader_id: Option<u64>,
    pub partition_owners: BTreeMap<u64, u64>, // partition_id -> node_id
}

impl ConsensusEngine {
    pub fn new(node_id: u64) -> Self {
        Self {
            node_id,
            state: NodeState::Follower,
            current_term: 0,
            voted_for: None,
            leader_id: None,
            partition_owners: BTreeMap::new(),
        }
    }

    /// بدء انتخاب قائد جديد
    pub fn start_election(&mut self, peers: &[u64]) {
        self.current_term += 1;
        self.state = NodeState::Candidate;
        self.voted_for = Some(self.node_id);

        let mut votes = 1u64; // صوت العقدة لنفسها
        for _peer_id in peers {
            // في النسخة الحقيقية، نرسل طلب تصويت وننتظر الرد
            // هنا نفترض استجابة جميع الأقران
        }

        votes += peers.len() as u64;
        if votes > (peers.len() + 1) as u64 / 2 {
            self.state = NodeState::Leader;
            self.leader_id = Some(self.node_id);
            println!(
                "🎯 Node {} elected as leader for term {}",
                self.node_id, self.current_term
            );
        }
    }

    /// هل هذه العقدة هي القائد؟
    pub fn is_leader(&self) -> bool {
        self.state == NodeState::Leader
    }
}
