pub mod config;

use std::collections::HashMap;
use crate::state::config::BotConfig;
use serenity::prelude::{Context, RwLock, TypeMapKey};
use std::sync::Arc;

pub(crate) async fn get_state(ctx: &Context) -> BotState {
    ctx.data
        .read()
        .await
        .get::<BotState>()
        .expect("Could not get BotState")
        .clone()
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BotState {
    pub(crate) config: BotConfig,
    pub(crate) workshop: Arc<RwLock<HashMap<String, bool>>>,
}

impl TypeMapKey for BotState {
    type Value = Self;
}
