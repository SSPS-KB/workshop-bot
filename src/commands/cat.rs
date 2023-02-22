use anyhow::{anyhow, Result};
use i18n::t;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;

use crate::state::get_state;

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cat")
        .description(t!("commands.cat.description").to_string())
        .description_localized("cs", t!("commands.cat.description", "cs"))
        .dm_permission(true)
}

//If there's a more compact way to do this please let me know
#[derive(Deserialize, Debug)]
struct TenorMedia {
    url: String,
}

#[derive(Deserialize, Debug)]
struct TenorMediaFormat {
    gif: TenorMedia,
}

#[derive(Deserialize, Debug)]
struct TenorResult {
    media_formats: TenorMediaFormat,
}

#[derive(Deserialize, Debug)]
struct TenorAPIResponse {
    results: Vec<TenorResult>,
}

async fn get_link(ctx: &Context) -> Result<String> {
    let api_key = match get_state(ctx).await.config.tenor_api_key {
        Some(api_key) => api_key,
        None => return Err(anyhow!("Missing Tenor API key"))
    };
    let term = "kitty%20review";
    let url = format!("https://tenor.googleapis.com/v2/search?q={term}&key={api_key}&limit=100");

    //Code from PR #13
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "SSPS-KB/1.0 workshop bot")
        .send()
        .await;

    let data = response?;
    let json = data.json::<TenorAPIResponse>().await?;

    match json.results.choose(&mut rand::thread_rng()) {
        Some(result) => Ok(result.media_formats.gif.url.to_owned()),
        None => Err(anyhow!("No GIFs found"))
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let result = match get_link(ctx).await {
        Ok(url) => url,
        Err(e) => {
            error!("There was an error while getting kitty review link: {e}");
            return;
        }
    };

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| embed
                        .image(result)
                        .color(Color::from_rgb(227, 0, 0))
                        .footer(|f| f.text("Via Tenor"))
                    )
                })
        })
        .await
    {
        error!("There was an error while responding to cat command: {e}")
    };
}
