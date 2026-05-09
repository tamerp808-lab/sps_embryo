use crate::state::AppState;
use std::sync::{Arc, Mutex};
use teloxide::{prelude::*, utils::command::BotCommands};

pub type SharedState = Arc<Mutex<AppState>>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "SPS Commands")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Get system status")]
    Status,
}

pub async fn start_bot(token: String, state: SharedState) {
    let bot = Bot::new(token);
    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let state = Arc::clone(&state);
        async move {
            match cmd {
                Command::Start => {
                    bot.send_message(
                        msg.chat.id,
                        "🤖 SPS Embryo ready. Use /status to see system state.",
                    )
                    .await?;
                }
                Command::Status => {
                    let status = {
                        let guard = state.lock().unwrap();
                        format!(
                            "Counter: {}\nEvents: {}",
                            guard.counter,
                            guard.history.len()
                        )
                    };
                    bot.send_message(msg.chat.id, status).await?;
                }
            }
            Ok::<_, teloxide::RequestError>(())
        }
    })
    .await;
}
