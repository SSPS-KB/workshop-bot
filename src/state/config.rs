use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct BotConfig {
    #[serde(alias = "discord_token")]
    pub token: Option<String>,
    #[serde(rename = "workshop_invite_channel")]
    pub invite_channel: Option<u64>,
    pub guilds: HashMap<String, GuildConfig>
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Default, Serialize, Deserialize)]
pub struct GuildConfig {
    pub automove_from: Option<u64>,
    pub automove_to: Option<u64>,
}

impl BotConfig {
    pub fn runtime_strip(&mut self) -> &mut Self {
        self.token = None;

        self
    }
}
