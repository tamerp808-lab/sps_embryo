use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub agent: String,
    pub status: TaskStatus,
    pub depends_on: Vec<u64>,
    pub result: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Repaired,
}

pub struct TaskGraph {
    tasks: BTreeMap<u64, Task>,
    next_id: u64,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self { tasks: BTreeMap::new(), next_id: 1 }
    }

    pub fn add_task(&mut self, description: String, agent: String, depends_on: Vec<u64>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.insert(id, Task {
            id, description, agent, status: TaskStatus::Pending, depends_on, result: None,
        });
        id
    }

    pub fn mark_running(&mut self, id: u64) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Running;
        }
    }

    pub fn mark_completed(&mut self, id: u64, result: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Completed;
            task.result = Some(result);
        }
    }

    pub fn mark_failed(&mut self, id: u64) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Failed;
        }
    }

    pub fn mark_repaired(&mut self, id: u64) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.status = TaskStatus::Repaired;
        }
    }

    pub fn get_next_ready(&self) -> Option<u64> {
        self.tasks.values()
            .filter(|t| t.status == TaskStatus::Pending && t.depends_on.iter().all(|d| {
                self.tasks.get(d).map_or(false, |dep| dep.status == TaskStatus::Completed)
            }))
            .min_by_key(|t| t.id)
            .map(|t| t.id)
    }

    pub fn all_completed(&self) -> bool {
        self.tasks.values().all(|t| t.status == TaskStatus::Completed)
    }
}
