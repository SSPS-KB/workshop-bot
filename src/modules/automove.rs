use crate::state::get_state;
use serenity::client::Context;
use serenity::model::voice::VoiceState;
use tracing::{error, info};

pub(crate) async fn run_automove(ctx: &Context, state: VoiceState) {
    let bot_state = get_state(ctx).await;
    let automove_from = bot_state.config.automove_from;
    let automove_to = bot_state.config.automove_to;

    let workshop = { *bot_state.workshop.read().await };

    if automove_from.is_none() || automove_to.is_none() || !workshop {
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
