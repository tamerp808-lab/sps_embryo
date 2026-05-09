use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkflowStatus {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowTask {
    pub workflow_id: u64,
    pub priority: u64,
    pub created_tick: u64,
    pub status: WorkflowStatus,
}

#[derive(Debug)]
pub struct DeterministicScheduler {
    tasks: BTreeMap<u64, WorkflowTask>,
}

impl DeterministicScheduler {
    pub fn new() -> Self {
        Self { tasks: BTreeMap::new() }
    }

    pub fn register_task(&mut self, workflow_id: u64, priority: u64, created_tick: u64) {
        let task = WorkflowTask { workflow_id, priority, created_tick, status: WorkflowStatus::Pending };
        self.tasks.insert(workflow_id, task);
    }

    pub fn mark_ready(&mut self, workflow_id: u64) {
        if let Some(task) = self.tasks.get_mut(&workflow_id) {
            task.status = WorkflowStatus::Ready;
        }
    }

    pub fn next_task(&self) -> Option<&WorkflowTask> {
        self.tasks
            .values()
            .filter(|t| t.status == WorkflowStatus::Ready)
            .min_by_key(|t| (t.priority, t.created_tick, t.workflow_id))
    }

    pub fn total_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn pending_ready_tasks(&self) -> Vec<&WorkflowTask> {
        let mut ready: Vec<&WorkflowTask> = self.tasks
            .values()
            .filter(|t| t.status == WorkflowStatus::Ready)
            .collect();
        ready.sort_by_key(|t| (t.priority, t.created_tick, t.workflow_id));
        ready
    }
}
