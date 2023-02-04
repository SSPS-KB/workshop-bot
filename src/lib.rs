use crate::commands::register_commands;
use crate::modules::automove::run_automove;
use crate::modules::invite::generate_invite;
use crate::state::config::BotConfig;
use crate::state::BotState;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::sync::Arc;
use tracing::{info, warn};

mod commands;
mod modules;
mod state;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

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
                _ => warn!(
                    "Recieved a command which is not implemented: {}",
                    command.data.name
                ),
            }
        }
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    let state = BotState {
        config: BotConfig::from(&secret_store),
        workshop: Arc::new(RwLock::new(false)),
    };

    {
        let mut data = client.data.write().await;
        data.insert::<BotState>(state);
    }

    Ok(client)
}
