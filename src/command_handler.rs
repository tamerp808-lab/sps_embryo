use crate::auto_capability::AutoCapability;
use crate::capabilities::CapabilityToken;
use crate::cognitive_loop::CognitiveLoop;
use crate::command::ApiCommand;
use crate::conversation_memory::ConversationMemory;
use crate::event::Event;
use crate::event_log;
use crate::goal_handler;
use crate::logical_clock::LogicalClock;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};
use crate::outbox::EffectOutbox;
use crate::state::AppState;
use std::sync::{Arc, Mutex};

pub enum CommandResult {
    Accepted { task_id: u64, reply: String },
    Rejected { reason: String },
    GoalResult(goal_handler::GoalCommandResult),
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle(
        command: ApiCommand,
        token: &Arc<CapabilityToken>,
        clock: &Arc<LogicalClock>,
        short_mem: &Arc<Mutex<ShortTermMemory>>,
        long_mem: &Arc<LongTermMemory>,
        vec_mem: &Arc<Mutex<VectorMemory>>,
        conv_mem: &Arc<ConversationMemory>,
        outbox: &Arc<Mutex<EffectOutbox>>,
        state: &Arc<Mutex<AppState>>,
    ) -> CommandResult {
        let required = crate::capabilities::Capability::WorkflowDispatch;
        if let Err(e) = AutoCapability::validate_request(token, required) {
            return CommandResult::Rejected {
                reason: format!("Capability denied: {}", e),
            };
        }

        match command {
            ApiCommand::ExecuteTask { goal } => {
                let task_id = clock.next_id();
                let tick = clock.current_tick();
                let vc = clock.vector_clock_string();
                let event = Event {

                    id: task_id,
                    tick,
                    event_type: "TaskRequested".to_string(),
                    payload: goal.clone(),
                    parent_id: clock.last_id(),
                    vector_clock: vc,
                };
                event_log::append_event(&event).expect("Failed to write event");
                let reply = CognitiveLoop::run(
                    &goal, short_mem, long_mem, vec_mem, conv_mem, outbox, clock,
                );

                crate::auto_audit::AutoAudit::log(
                    "TASK_ACCEPTED",
                    &format!("Task {} : {}", task_id, goal),
                );

                CommandResult::Accepted { task_id, reply }
            }
            other => {
                let result = goal_handler::handle_goal_command(other, token, clock, state);
                CommandResult::GoalResult(result)
            }
        }
    }
}
