use serenity::model::gateway::Activity;
use serenity::model::user::OnlineStatus;
use serenity::prelude::Context;
use tracing::info;

pub(crate) async fn set_status(ctx: &Context) {
    let activity = Activity::watching("New KBB students");
    ctx.set_presence(Some(activity), OnlineStatus::Online).await;

    info!("Set a watching status");
}
