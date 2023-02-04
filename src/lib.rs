use crate::state::config::{get_state, BotConfig};
use crate::state::BotState;
use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::invite::Invite;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod state;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // Create an invite
        let invite_channel = get_state(&ctx).await.config.invite_channel;

        if let Some(invite_channel) = invite_channel {
            match Invite::create(&ctx.http, invite_channel, |mut create| {
                create.max_age(0);
                create
            })
            .await
            {
                Ok(invite) => info!("Created invite {}", invite.url()),
                Err(e) => error!("Could not create invite: {:?}", e),
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, state: VoiceState) {
        let automove_from = get_state(&ctx).await.config.automove_from;
        let automove_to = get_state(&ctx).await.config.automove_to;
        if automove_from.is_none() || automove_to.is_none() {
            return;
        }
        let automove_to = automove_to.unwrap();
        let automove_from = automove_from.unwrap();

        if state.channel_id != Some(automove_from.into()) {
            return;
        }

        if let Some(member) = state.member {
            match member.move_to_voice_channel(&ctx.http, automove_to).await {
                Ok(_) => info!("Moved {} into #workshopy.", member.user.name),
                Err(e) => error!("{}", e),
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
    };

    {
        let mut data = client.data.write().await;
        data.insert::<BotState>(state);
    }

    Ok(client)
}
