use anyhow::anyhow;
use std::fs::read_to_string;
use tracing::error;
use workshop_bot::create;
use workshop_bot::state::config::BotConfig;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Init tracing_subscriber
    tracing_subscriber::fmt::init();

    // Load config
    let config = read_to_string("Config.toml")?;
    let config: BotConfig = toml::from_str(&config)?;

    println!("{:?}", config);

    let token = match config.token.clone() {
        Some(c) => c,
        None => return Err(anyhow!("Missing \"discord_token\"")),
    };

    // Spawn bot on another thread
    match create(token, config).await {
        Ok(mut client) => {
            if let Err(e) = client.start().await {
                error!("There was an error: {e}");
            }
        }
        Err(e) => error!("There was an error: {e}"),
    };

    Ok(())
}
