use anyhow::anyhow;
use serenity::{async_trait};
use serenity::model::gateway::Ready;
use serenity::model::invite::Invite;
use serenity::model::voice::VoiceState;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        match Invite::create(&ctx.http, 1069606132257132608, |mut create| {
            create.max_age(0);
            create
        }).await {
            Ok(invite) => info!("Created invite {}", invite.url()),
            Err(e) => error!("Could not create invite: {:?}", e),
        }
    }

    async fn voice_state_update(&self, ctx: Context, state: VoiceState) {
        if state.channel_id != Some(1069606132257132609.into()) {
            return
        }

        match state.member {
            Some(member) => {
                match member.move_to_voice_channel(&ctx.http, 1069607106908532850).await {
                    Ok(_) => info!("Moved {} into #workshopy.", member.user.name),
                    Err(e) => error!("{}", e),
                }

            }
            _ => {},
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
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}