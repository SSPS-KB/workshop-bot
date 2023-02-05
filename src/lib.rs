use crate::commands::register_commands;
use crate::modules::automove::run_automove;
use crate::modules::invite::generate_invite;
use crate::modules::status::set_status;
use crate::state::config::BotConfig;
use crate::state::BotState;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use std::sync::Arc;
use tracing::{info, warn};

pub(crate) mod commands;
pub(crate) mod modules;
pub mod state;

pub(crate) struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        set_status(&ctx).await;
        register_commands(&ctx).await;
        generate_invite(&ctx).await;
    }

    async fn voice_state_update(&self, ctx: Context, state: VoiceState) {
        run_automove(&ctx, state).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "workshop" => commands::workshop::run(&ctx, &command).await,
                "kiss" => commands::otakugif::run(&ctx, &command, "kiss").await,
                "hug" => commands::otakugif::run(&ctx, &command, "hug").await,
                _ => warn!(
                    "Received a command which is not implemented: {}",
                    command.data.name
                ),
            }
        }
    }
}

pub async fn create(token: String, settings: BotConfig) -> anyhow::Result<Client> {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    let state = BotState {
        config: settings,
        workshop: Arc::new(RwLock::new(false)),
    };

    {
        let mut data = client.data.write().await;
        data.insert::<BotState>(state);
    }

    Ok(client)
}
