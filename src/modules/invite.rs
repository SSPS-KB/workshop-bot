use crate::state::get_state;
use serenity::model::prelude::Invite;
use serenity::prelude::Context;
use tracing::{error, info};

pub(crate) async fn generate_invite(ctx: &Context) {
    // Create an invite
    let invite_channel = get_state(ctx).await.config.invite_channel;

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
