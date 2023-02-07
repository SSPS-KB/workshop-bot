use i18n::t;
use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::utils::Color;
use tracing::error;

pub(crate) fn register_kiss(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("kiss")
        .description(t!("commands.kiss.description").to_string())
        .description_localized("cs", t!("commands.kiss.description", "cs"))
        .dm_permission(true)
}

pub(crate) fn register_hug(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("hug")
        .description(t!("commands.hug.description").to_string())
        .description_localized("cs", t!("commands.hug.description", "cs"))
        .dm_permission(true)
}

#[derive(Deserialize)]
struct KissResponse {
    url: String,
}

async fn get_link(reaction: &str) -> anyhow::Result<String> {
    let response =
        reqwest::get(format!("https://api.otakugifs.xyz/gif?reaction={reaction}")).await?;
    let body: KissResponse = response.json().await?;

    Ok(body.url)
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction, reaction: &str) {
    let result = match get_link(reaction).await {
        Ok(url) => url,
        Err(e) => {
            error!("Could not get {reaction} link: {e}");
            return;
        }
    };

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(":kissing_heart:")
                        .embed(|embed| embed.image(result).color(Color::from_rgb(255, 45, 53)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to {reaction} command: {}",
            e
        )
    };
}
