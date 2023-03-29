use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;
use i18n::t;

pub(crate) fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("amimir")
        .description(t!("commands.amimir.description").to_string())
        .description_localized("cs", t!("commands.amimir.description", "cs"))
        .dm_permission(true)
}

fn get_message(locale: String) -> &'static str {
    match locale.asstr() {
        "cs" => t!("commands.amimir.message", "cs"),
         => t!("commands.amimir.message"),
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let locale = command.clone().locale;

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(get_message(locale))
                        .embed(|embed| embed
                            .image("https://cdn.discordapp.com/attachments/888165880553410610/1090714681062215811/trnka_1.png")
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to amimir (trnka) command: {}",
            e
        )
    };
}
