use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;
use i18n::t;
use rand::prelude::*;

pub(crate) fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("kitty")
        .description(t!("commands.kitty.description").to_string())
        .description_localized("cs", t!("commands.kitty.description", "cs"))
        .dm_permission(true)
}

fn get_message(locale: String) -> &'static str {
    match locale.as_str() {
        "cs" => t!("commands.kitty.message", "cs"),
        _ => t!("commands.kitty.message"),
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let locale = command.clone().locale;

    let mut rng = rand::thread_rng();
    let my_strings : Vec<&str> = vec![];
    let random_string_index: usize = rng.gen_range(0..my_strings.len());
    let string = my_strings[random_string_index];

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(get_message(locale))
                        .embed(|embed| embed
                            .image(string)
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to skull command: {}",
            e
        )
    };
}