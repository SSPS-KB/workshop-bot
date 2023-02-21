use crate::commands::command::SlashCommand;
use crate::state::get_state;
use i18n::t;
use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::Permissions;
use serenity::prelude::Context;
use tracing::error;

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("workshop")
        .description(t!("commands.workshop.description"))
        .description_localized("cs", t!("commands.workshop.description", "cs"))
        .create_option(|option| {
            option
                .name(t!("commands.workshop.options.on.name"))
                .description(t!("commands.workshop.options.on.description"))
                .name_localized("cs", t!("commands.workshop.options.on.name", "cs"))
                .description_localized("cs", t!("commands.workshop.options.on.description", "cs"))
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

fn get_message(locale: &str, on: bool) -> &'static str {
    match locale {
        "cs" => match on {
            true => t!("commands.workshop.started", "cs"),
            false => t!("commands.workshop.stopped", "cs"),
        },
        _ => match on {
            true => t!("commands.workshop.started"),
            false => t!("commands.workshop.stopped"),
        },
    }
}

pub(crate) struct WorkshopCommand;

#[async_trait]
impl SlashCommand for WorkshopCommand {
    fn name() -> String {
        "workshop".into()
    }

    fn description() -> String {
        t!("commands.workshop.description").into()
    }

    async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
        let guild_id = match command.guild_id {
            None => return,
            Some(id) => id,
        };

        let on = match command.data.options.get(0) {
            Some(options) => match options.value {
                Some(serde_json::Value::Bool(b)) => b,
                _ => false,
            },
            _ => false,
        };

        let workshop = { get_state(ctx).await };

        {
            let mut lock = workshop.workshop.write().await;

            lock.insert(guild_id.to_string(), on);
        }

        if let Err(e) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content(get_message(command.locale.as_str(), on))
                            .ephemeral(true)
                    })
            })
            .await
        {
            error!("There was an error while running workshop command: {e}")
        };
    }
}
