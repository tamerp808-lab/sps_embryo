use crate::auto_capability::AutoCapability;
use crate::capabilities::CapabilityToken;
use crate::command::ApiCommand;
use crate::event::Event;
use crate::event_log;
use crate::logical_clock::LogicalClock;
use crate::long_term_planner::LongTermPlanner;
use crate::state::{AppState, Goal, GoalStatus};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum GoalCommandResult {
    Created {
        goal_id: u64,
    },
    Decomposed {
        goal_id: u64,
        steps: Vec<String>,
    },
    List(Vec<Goal>),
    Progress {
        goal_id: u64,
        status: GoalStatus,
        children_progress: Vec<(u64, GoalStatus)>,
    },
    Rejected {
        reason: String,
    },
}

pub fn handle_goal_command(
    cmd: ApiCommand,
    token: &Arc<CapabilityToken>,
    clock: &LogicalClock,
    state: &Arc<Mutex<AppState>>,
) -> GoalCommandResult {
    if let Err(e) =
        AutoCapability::validate_request(token, crate::capabilities::Capability::WorkflowDispatch)
    {
        return GoalCommandResult::Rejected {
            reason: format!("Capability denied: {}", e),
        };
    }

    match cmd {
        ApiCommand::SetGoal {
            description,
            priority,
        } => {
            let id = clock.next_id();
            let tick = clock.current_tick();
            let vc = clock.vector_clock_string();
            let event = Event {

                id,
                tick,
                event_type: "GoalCreated".to_string(),
                payload: serde_json::json!({ "description": &description, "priority": priority })
                    .to_string(),
                parent_id: clock.last_id(),
                vector_clock: vc,
            };
            event_log::append_event(&event).expect("Failed to write event");

            let mut s = state.lock().unwrap();
            let goal = Goal {
                id,
                description,
                status: GoalStatus::Pending,
                priority,
                parent_id: None,
                children: vec![],
                depends_on: vec![],
                created_tick: tick,
                updated_tick: tick,
            };
            s.goals.insert(id, goal);
            GoalCommandResult::Created { goal_id: id }
        }
        ApiCommand::ListGoals => {
            let s = state.lock().unwrap();
            GoalCommandResult::List(s.goals.values().cloned().collect())
        }
        ApiCommand::DecomposeGoal { goal_id } => match LongTermPlanner::decompose(goal_id, state) {
            Ok(steps) => GoalCommandResult::Decomposed { goal_id, steps },
            Err(e) => GoalCommandResult::Rejected { reason: e },
        },
        ApiCommand::GoalProgress { goal_id } => {
            let s = state.lock().unwrap();
            if let Some(goal) = s.goals.get(&goal_id) {
                let children_progress: Vec<(u64, GoalStatus)> = goal
                    .children
                    .iter()
                    .filter_map(|child_id| {
                        s.goals.get(child_id).map(|g| (*child_id, g.status.clone()))
                    })
                    .collect();
                GoalCommandResult::Progress {
                    goal_id,
                    status: goal.status.clone(),
                    children_progress,
                }
            } else {
                GoalCommandResult::Rejected {
                    reason: "Goal not found".to_string(),
                }
            }
        }
        _ => GoalCommandResult::Rejected {
            reason: "Invalid goal command".to_string(),
        },
    }
}
