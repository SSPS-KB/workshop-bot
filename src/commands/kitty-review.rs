use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Color;
use tracing::error;
use i18n::t;
use reqwest::header;

pub(crate) fn register(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("kitty-review")
        .description(t!("commands.kitty-review.description").to_string())
        .description_localized("cs", t!("commands.kitty-review.description", "cs"))
        .dm_permission(true)
}

fn get_message(locale: String) -> &'static str {
    match locale.as_str() {
        "cs" => t!("commands.kitty-review.message", "cs"),
        _ => t!("commands.kitty-review.message"),
    }
}

pub(crate) async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let locale = command.clone().locale;

    let tenor_api_key = "API KEY ZDE";
    let search_url = "https://tenor.googleapis.com/v2/search?q=kitty-review&key=".to_owned() + tenor_api_key;
    let response = reqwest::Client::new()
        .get(&search_url)
        .header(header::USER_AGENT, "SSPS-KB BOT")
        .send()
        .await;

    if let Ok(response) = response {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            if let Some(results) = json.get("results").and_then(|results| results.as_array()) {
                if let Some(random_result) = results.choose(&mut rand::thread_rng()) {
                    if let Some(media) = random_result.get("media").and_then(|media| media.as_array()) {
                        if let Some(first_media) = media.get(0) {
                            if let Some(gif) = first_media.get("gif").and_then(|gif| gif.get("url").and_then(|url| url.as_str())) {
                                // Respond with the gif URL in an embed
                                if let Err(e) = command
                                    .create_interaction_response(&ctx.http, |response| {
                                        response
                                            .kind(InteractionResponseType::ChannelMessageWithSource)
                                            .interaction_response_data(|message| {
                                                message
                                                    .embed(|embed| embed
                                                        .image(gif)
                                                        .color(Color::from_rgb(110, 110, 110)))
                                            })
                                    })
                                    .await
                                {
                                    error!("There was an error while responding to kitty-review command: {}", e)
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    // Respond with an error message if there was a problem fetching the gif
    if let Err(e) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(get_message(locale))
                        .embed(|embed| embed
                            .description("Sorry, I couldn't find a kitty review gif at the moment :(")
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!("There was an error while responding to kitty-review command: {}", e)
    };
}
