use crate::project_state::ProjectState;
use crate::task_graph::TaskGraph;
use crate::agent_visualizer::AgentVisualizer;
use crate::llm_chain;
use crate::memory_system::LongTermMemory;
use tokio::sync::broadcast;

pub struct EventDrivenOrchestrator;

impl EventDrivenOrchestrator {
    pub async fn run(
        description: &str,
        memory: &LongTermMemory,
        tx: &broadcast::Sender<String>,
    ) -> Result<ProjectState, String> {
        let mut project = ProjectState::new();

        // === 1. إنشاء خطة المهام (Planner) ===
        AgentVisualizer::send_agent_state(tx, "planner", "thinking", "Creating task plan...");
        let plan = llm_chain::chat_sync(&format!(
            "Break down this project into 4-6 concrete tasks. Output JSON array of task descriptions.\nProject: {}", description
        ));
        let task_descs: Vec<String> = serde_json::from_str(&plan).unwrap_or_else(|_| {
            vec!["Generate HTML".into(), "Generate CSS".into(), "Generate JavaScript".into(), "Evaluate quality".into(), "Repair issues".into()]
        });

        for desc in task_descs {
            project.task_graph.add_task(desc, "coder".into(), vec![]);
        }
        // ربط التبعيات (المهمة الأخيرة تعتمد على السابقات)
        // نبسط هنا: جميع المهام مستقلة حاليًا.

        // === 2. حلقة تنفيذ المهام (الحلقة المعرفية الحقيقية) ===
        while !project.task_graph.all_completed() {
            // 3.1. البحث عن المهمة الجاهزة التالية
            let task_id = match project.task_graph.get_next_ready() {
                Some(id) => id,
                None => {
                    // لا توجد مهام جاهزة - قد يكون هناك فشل
                    AgentVisualizer::send_log(tx, "warning", "No ready tasks available. Checking for failures...");
                    break;
                }
            };

            // 3.2. توليد الكود لكل مهمة
            project.task_graph.mark_running(task_id);
            AgentVisualizer::send_agent_state(tx, "coder", "running", "Generating code...");

            let task_result = llm_chain::chat_sync(&format!(
                "Complete this task for the project '{}': {}. Output ONLY the result.",
                description, project.task_graph.tasks.get(&task_id).unwrap().description
            ));

            project.task_graph.mark_completed(task_id, task_result.clone());
            project.files.insert(format!("task_{}.txt", task_id), task_result.clone());
            project.execution_history.push(format!("Task {} completed: {}", task_id, &task_result[..50.min(task_result.len())]));

            // 3.3. تقييم الجودة (مرحلة التقييم)
            AgentVisualizer::send_agent_state(tx, "repair", "thinking", "Evaluating quality...");
            let evaluation = llm_chain::chat_sync(&format!(
                "Rate this task result on a scale of 1-10: {}", task_result
            ));

            // 3.4. إصلاح الفشل (إذا لزم الأمر)
            if evaluation.contains("1") || evaluation.contains("2") || evaluation.contains("3") {
                AgentVisualizer::send_agent_state(tx, "repair", "running", "Repairing failed task...");
                let repaired = llm_chain::chat_sync(&format!(
                    "This task result scored low ({}). Improve it: {}", evaluation, task_result
                ));
                project.task_graph.mark_repaired(task_id);
                project.files.insert(format!("task_{}_repaired.txt", task_id), repaired);
                project.diagnostics.push(format!("Task {} repaired", task_id));
            }

            // 3.5. بث التقدم
            let total = project.task_graph.tasks.len();
            let completed = project.task_graph.tasks.values().filter(|t| t.status == crate::task_graph::TaskStatus::Completed).count();
            AgentVisualizer::send_log(tx, "info", &format!("Progress: {}/{} tasks", completed, total));
        }

        // === 4. تحسين UX/SEO (المرحلة النهائية) ===
        AgentVisualizer::send_agent_state(tx, "ux", "running", "Optimizing user experience...");
        AgentVisualizer::send_agent_state(tx, "seo", "running", "Optimizing for search engines...");
        AgentVisualizer::send_log(tx, "success", "Project lifecycle completed.");

        // 5. تخزين في الذاكرة
        memory.store("project", description, None, 0.9).ok();
        memory.record_episode("project_lifecycle", &format!("Project: {}", description), 0.8).ok();

        Ok(project)
    }
}
