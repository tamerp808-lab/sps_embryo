use crate::task_graph::TaskGraph;

pub struct ProjectState {
    pub files: std::collections::BTreeMap<String, String>,
    pub task_graph: TaskGraph,
    pub execution_history: Vec<String>,
    pub diagnostics: Vec<String>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            files: std::collections::BTreeMap::new(),
            task_graph: TaskGraph::new(),
            execution_history: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}
