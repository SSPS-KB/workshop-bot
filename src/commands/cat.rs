use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;
use rand::seq::SliceRandom;
use i18n::t;

pub(crate) fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("cat")
        .description(t!("commands.cat.description").to_string())
        .description_localized("cs", t!("commands.cat.description", "cs"))
        .dm_permission(true)
}

//TODO: no hardcoding keys
async fn get_link() -> anyhow::Result<String> {
    let api_key = "AIzaSyBYTniO_HQAop9AAAE5fo7KAPcqq_wIlv4";
    let term = "kitty%20review";
    let url = format!("https://tenor.googleapis.com/v2/search?q={}&key={}&limit=50", term, api_key);

    let response = reqwest::get(url).await;
    if let Ok(response) = response {
        let json = response.json::<serde_json::Value>().await;
        if let Ok(json) = json {
            if let Some(results) = json.get("results").and_then(|results| results.as_array()) {
                if let Some(random_result) = results.choose(&mut rand::thread_rng()) {
                    if let Some(gif) = random_result["media_formats"]["gif"]["url"].as_str() {
                        return Ok(gif.to_string());
                    }
                }
            }
        }
    }
    //Why can't I just return a string literal in Ok()?
    let error = "";
    Ok(error.to_string())
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
                    message
                        .embed(|embed| embed
                            .image(result)
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to cat command: {}",
            e
        )
    };
}
