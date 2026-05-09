use crate::cognitive_loop::CognitiveLoop;
use crate::conversation_memory::ConversationMemory;
use crate::deterministic_scheduler::DeterministicScheduler;
use crate::logical_clock::LogicalClock;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};
use crate::outbox::EffectOutbox;
use crate::state::GoalStatus;
use crate::state_store::StateStore;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AutonomousWorker;

impl AutonomousWorker {
    pub fn start(
        state_store: Arc<StateStore>,
        short_mem: Arc<Mutex<ShortTermMemory>>,
        long_mem: Arc<LongTermMemory>,
        vec_mem: Arc<Mutex<VectorMemory>>,
        conv_mem: Arc<ConversationMemory>,
        outbox: Arc<Mutex<EffectOutbox>>,
        interval: Duration,
    ) {
        let clock = Arc::new(LogicalClock::new(1));
        std::thread::spawn(move || loop {
            let goal_to_process = {
                let s = state_store.read_state();
                DeterministicScheduler::next_task(&s, &clock)
            };

            if let Some(goal) = goal_to_process {
                println!(
                    "🤖 Worker: picked goal '{}' (ID: {}, priority: {})",
                    goal.description, goal.id, goal.priority
                );

                let result = CognitiveLoop::run(
                    &goal.description,
                    &short_mem,
                    &long_mem,
                    &vec_mem,
                    &conv_mem,
                    &outbox,
                    &clock,
                );

                let event = crate::event::Event {

                    id: clock.next_id(),
                    tick: clock.current_tick(),
                    event_type: "GoalStatusChanged".to_string(),
                    payload: format!(
                        "Goal {} : {:?}",
                        goal.id,
                        if result.contains("✅") {
                            GoalStatus::Completed
                        } else {
                            GoalStatus::Failed
                        }
                    ),
                    parent_id: clock.last_id(),
                    vector_clock: clock.vector_clock_string(),
                };
                state_store.apply_event(&event);
                println!("  ↳ Updated goal {} status.", goal.id);
            } else {
                std::thread::sleep(interval);
            }
        });
    }
}
