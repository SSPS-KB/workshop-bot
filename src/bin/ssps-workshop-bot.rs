use anyhow::anyhow;
use std::fs::read_to_string;
use std::path::PathBuf;
use workshop_bot::create;
use workshop_bot::state::config::BotConfig;

fn find_config() -> anyhow::Result<PathBuf> {
    let cwd = PathBuf::from("Config.toml");

    if cwd.exists() {
        return Ok(cwd);
    }

    let etc = PathBuf::from("/etc/workshop-bot/Config.toml");
    if etc.exists() {
        return Ok(etc);
    }

    Err(anyhow!("There is no Config.toml in current directory nor in /etc/workshop-bot"))
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Init tracing_subscriber
    tracing_subscriber::fmt::init();

    // Load config
    let config = read_to_string(find_config()?)?;
    let config: BotConfig = basic_toml::from_str(&config)?;

    let token = match config.token.clone() {
        Some(c) => c,
        None => return Err(anyhow!("Missing \"discord_token\"")),
    };

    // Spawn bot on another thread
    match create(token, config).await {
        Ok(mut client) => {
            if let Err(e) = client.start().await {
                return Err(anyhow!("There was an error: {e}"));
            }
        }
        Err(e) => return Err(anyhow!("There was an error: {e}")),
    };

    Ok(())
}
