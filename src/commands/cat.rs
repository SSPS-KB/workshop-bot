use i18n::t;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;

pub(crate) fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("cat")
        .description(t!("commands.cat.description").to_string())
        .description_localized("cs", t!("commands.cat.description", "cs"))
        .dm_permission(true)
}

//If there's a more compact way to do this, please let me know
#[derive(Deserialize, Debug)]
struct Media {
    url: String,
}

#[derive(Deserialize, Debug)]
struct MediaFormat {
    gif: Media,
}

#[derive(Deserialize, Debug)]
struct TenorResults {
    media_formats: MediaFormat,
}

#[derive(Deserialize, Debug)]
struct TenorAPIResponse {
    results: Vec<TenorResults>,
}

async fn get_link() -> anyhow::Result<String> {
    let api_key = std::env::var("TENOR_API_KEY").unwrap();
    let term = "kitty%20review";
    let url = format!(
        "https://tenor.googleapis.com/v2/search?q={}&key={}&limit=100",
        term, api_key
    );

    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "SSPS-KB/1.0 workshop bot")
        .send()
        .await;

    if let Ok(response) = response {
        let json = response.json::<TenorAPIResponse>().await;
        match json {
            Ok(data) => {
                if let Some(random_gif) = data.results.choose(&mut rand::thread_rng()) {
                    return Ok(random_gif.media_formats.gif.url.to_string());
                }
            }
            Err(e) => {
                error!("Error while parsing data from Tenor API: {}", e);
            }
        };
    }
    return Err(anyhow::anyhow!("Error while connecting to Tenor API"));
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let result = match get_link().await {
        Ok(url) => url,
        Err(e) => {
            error!("Error while running cat command: {}", e);
            return;
        }
    };

    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| embed.image(result).color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!("There was an error while responding to cat command: {}", e)
    };
}
