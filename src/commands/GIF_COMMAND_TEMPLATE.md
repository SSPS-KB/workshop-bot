# How to make gif commands

<hr>

* create a `.rs` file in `src/commands` with an appropriate name
* paste in this code:
```rust
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
        .name({command name})
        .description(t!("commands.{command name}.description").to_string())
        .description_localized("cs", t!("commands.{command name}.description", "cs"))
        .dm_permission(true)
}

fn get_message(locale: String) -> &'static str {
    match locale.as_str() {
        "cs" => t!("commands.{command name}.message", "cs"),
        _ => t!("commands.{command name}.message"),
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
                            .image({gif url})
                            .color(Color::from_rgb(110, 110, 110)))
                })
        })
        .await
    {
        error!(
            "There was an error while responding to {command name} command: {}",
            e
        )
    };
}
```
* replace `{command name}` with the name of your command
* replace `{gif url}` with the gif url
  * `WARNING` if you use a tenor gif, check if the url format is `media.tenor.com/...`, `tenor.com/...` gifs don't point directly to the file, so they don't work)
* in `src/commands/mod.rs` at the top write this code: `pub(crate) mod {command name};` and again change the `{command name}` to the name of your command
* then, scroll down and below the `result.push...` blocks add another one in this format:
```rust
results.push(
    Command::create_global_application_command(&ctx.http, |command| {
        {command name}::register(command)
    })
        .await,
);
```
* again, change the `{command name}` to your command name
* go to `src/lib.rs` and to the `Interaction::Application ... name.as_str() { ...` add another line saying:
```rust
"{command name}" => commands::{command name}::run(&ctx, &command).await,
```
* and change the `{command name}` to your command name
* next, in `locale/cs.json` and `locale/en.json` in `"commands": { ... ` add another block of code saying:
```json
"skull": {
  "description": "Sends a {gif description} gif",
  "message": "{message}"
}
```
* change `{gif description}` to usually the command name or some very short description
* change `{message}` to a message you want the bot to send with the gif
* do this for both the languages in the appropriate language
* in `.changes` add another `.md` file with the name `{command name}-command.md` and change {command name} to your command name
* in this file, paste this text:
```markdown
---
workshop-bot: patch
---
Add a /{command name} command
```
* and change the `{command name}` to your command name

### now you're ready to pull request!