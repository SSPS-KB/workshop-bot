use std::collections::HashMap;
use crate::state::get_state;
use serenity::client::Context;
use serenity::model::voice::VoiceState;
use tracing::{error, info};

pub(crate) async fn run_automove(ctx: &Context, state: VoiceState) {
    let guild_id = match state.guild_id {
        Some(id) => id,
        None => return,
    };

    let bot_state = get_state(ctx).await;
    let guild_config = bot_state.config.guilds.get(&guild_id.to_string());
    if guild_config.is_none() {
        return
    }

    let guild_config = guild_config.unwrap();
    let automove_from = guild_config.automove_from;
    let automove_to = guild_config.automove_to;

    let workshop = {
        let lock = bot_state.workshop.read().await;
        let guild_id = guild_id.to_string();
        match lock.get(&guild_id) {
            None => None,
            Some(v) => Some(v.to_owned()),
        }
    };

    if automove_from.is_none() || automove_to.is_none() || workshop.is_none() || !workshop.unwrap() {
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
