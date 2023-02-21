use serenity::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::GuildId;
use serenity::prelude::Context;

#[async_trait]
pub trait SlashCommand {
    fn name() -> String;

    fn description() -> String;

    async fn run(ctx: &Context, command: &ApplicationCommandInteraction);
}

pub(crate) async fn register_guild_slash_command<T: SlashCommand>(
    ctx: &Context,
    guild_id: GuildId,
    _: T,
) {
    let name = T::name();
    guild_id
        .create_application_command(&ctx.http, |cmd| {
            cmd.name(name.clone()).description(T::description())
        })
        .await
        .unwrap_or_else(|e| panic!("Could not register command {name}: {e}"));
}
