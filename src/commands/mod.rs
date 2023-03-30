use serenity::client::Context;
use serenity::model::application::command::Command;
use serenity::model::prelude::GuildId;
use tracing::{error, info, warn};

use crate::state::get_state;

pub(crate) mod chad;
pub(crate) mod otakugif;
pub(crate) mod reaction_role;
pub(crate) mod skull;
pub(crate) mod workshop;
pub(crate) mod fr;
pub(crate) mod cat;

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
        Command::create_global_application_command(&ctx.http, |command| {chad::register(command)})
        .await,
    );

    results.push(
        Command::create_global_application_command(&ctx.http, |command| {skull::register(command)})
            .await,
    );

    if get_state(ctx).await.config.tenor_api_key.clone().is_some() {
        //Register commands that use the Tenor API here.
        results.push(
            Command::create_global_application_command(&ctx.http, |command| {cat::register(command)})
                .await,
        );
    } else {
        warn!("Missing tenor_api_key, commands like cat that use the Tenor API won't work!");
    }

    match results
        .into_iter()
        .collect::<Result<Vec<Command>, serenity::Error>>()
    {
        Ok(_) => info!("Created global application commands"),
        Err(e) => error!("Could not create global application command: {}", e),
    };
}
