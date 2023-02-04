pub(crate) mod config;

use crate::state::config::BotConfig;
use serenity::prelude::TypeMapKey;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Default)]
pub(crate) struct BotState {
    pub(crate) config: BotConfig,
}

impl TypeMapKey for BotState {
    type Value = Self;
}
