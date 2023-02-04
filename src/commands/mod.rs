use serenity::client::Context;
use serenity::model::application::command::Command;
use tracing::{error, info};
use crate::commands;

pub(crate) mod workshop;

pub(crate) async fn register_commands(ctx: &Context) {
    /*let guild_id = GuildId::from();

    info!(
        "{:?}",
        guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| {
                    commands::workshop::register(command)
                });
                commands
            })
            .await
    );*/

    match Command::create_global_application_command(&ctx.http, |command| {
        commands::workshop::register(command)
    })
        .await
    {
        Ok(_) => info!("Created global application commands"),
        Err(e) => error!("Could not create global application command: {}", e),
    };
}