use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::prelude::GuildId;
use tracing::{error, info};

pub(crate) mod otakugif;
pub(crate) mod reaction_role;
pub(crate) mod workshop;
pub(crate) mod chad;
pub(crate) mod skull;

pub(crate) async fn register_commands(ctx: &Context) {
    let guild_id = GuildId::from(1069606131510562889);

    info!(
        "{:?}",
        guild_id
            .set_application_commands(&ctx.http, |commands| {
                /*
                commands.create_application_command(|command| otakugif::register_kiss(command));
                commands.create_application_command(|command| otakugif::register_hug(command));
                */
                commands
            })
            .await
    );

    let mut results = Vec::new();

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            workshop::register(command)
        })
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            otakugif::register_kiss(command)
        })
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            otakugif::register_hug(command)
        })
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            otakugif::register_slap(command)
        })
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            otakugif::register_punch(command)
        })
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {
            chad::register(command)
        })
            .await,
    );

    results.push(
            Command::create_global_application_command(&ctx.http, |command| {
                skull::register(command)
            })
                .await,
        );

    match results
        .into_iter()
        .collect::<Result<Vec<Command>, serenity::Error>>()
    {
        Ok(_) => info!("Created global application commands"),
        Err(e) => error!("Could not create global application command: {}", e),
    };
}
