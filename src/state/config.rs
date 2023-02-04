use shuttle_secrets::SecretStore;
use std::str::FromStr;

fn parse_option<T: FromStr>(store: &SecretStore, key: &str) -> Option<T> {
    match store.get(key) {
        Some(v) => match v.parse::<T>() {
            Ok(val) => Some(val),
            _ => None,
        },
        None => None,
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Default)]
pub(crate) struct BotConfig {
    pub(crate) invite_channel: Option<u64>,
    pub(crate) automove_from: Option<u64>,
    pub(crate) automove_to: Option<u64>,
}

impl From<&SecretStore> for BotConfig {
    fn from(value: &SecretStore) -> Self {
        let invite_channel: Option<u64> = parse_option(value, "WORKSHOP_INVITE_CHANNEL");
        let automove_from: Option<u64> = parse_option(value, "WORKSHOP_AUTOMOVE_FROM");
        let automove_to: Option<u64> = parse_option(value, "WORKSHOP_AUTOMOVE_TO");

        BotConfig {
            invite_channel,
            automove_from,
            automove_to,
        }
    }
}
