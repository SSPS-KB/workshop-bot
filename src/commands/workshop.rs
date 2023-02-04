use crate::state::get_state;
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
        .description("Start/Stop a workshop (automove)")
        .description_localized(
            "cs",
            "Zapni/Vypni workshop (automatické přesouvání uživatelů)",
        )
        .create_option(|option| {
            option
                .name("on")
                .description("Whether the workshop should be on after executed")
                .name_localized("cs", "zapnut")
                .description_localized("cs", "Zda workshop má být zapnut po spuštění příkazu")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
}

fn get_message(locale: &str, on: bool) -> &'static str {
    match locale {
        "cs" => match on {
            true => "Workshop byl spuštěn!",
            false => "Workshop byl zastaven!",
        },
        _ => match on {
            true => "Workshop was started!",
            false => "Workshop was stopped!",
        },
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
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

        *lock = on;
    }

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(get_message(command.locale.as_str(), on))
                })
        })
        .await
    {
        error!("There was an error while running workshop command: {}", e)
    };
}
