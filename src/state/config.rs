use serde::{Deserialize, Serialize};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Default, Serialize, Deserialize)]
pub struct BotConfig {
    #[serde(alias = "discord_token")]
    pub token: Option<String>,
    #[serde(rename = "workshop_invite_channel")]
    pub invite_channel: Option<u64>,
    #[serde(rename = "workshop_automove_from")]
    pub automove_from: Option<u64>,
    #[serde(rename = "workshop_automove_to")]
    pub automove_to: Option<u64>,
}

impl BotConfig {
    pub fn runtime_strip(&mut self) -> &mut Self {
        self.token = None;

        self
    }
}
