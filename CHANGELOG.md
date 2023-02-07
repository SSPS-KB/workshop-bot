# Changelog

## \[0.2.0]

- Added reaction roles interaction handler.
  - [32be324](https://github.com/SSPS-KB/workshop-bot/commit/32be324861f6380497eea0cae3371084f5ab55f8) feat: add reaction roles, make some responses ephemeral on 2023-02-07
- Made the `/workshop` command's responses ephemeral, so if used in global channels users can't see it.
  - [32be324](https://github.com/SSPS-KB/workshop-bot/commit/32be324861f6380497eea0cae3371084f5ab55f8) feat: add reaction roles, make some responses ephemeral on 2023-02-07
- Added per-guild configuration for automove. See Config.example.toml
  - [011a2ad](https://github.com/SSPS-KB/workshop-bot/commit/011a2ade3ca5b4547dda995fe0b0befe82c3568a) feat: move guild config into its own section on 2023-02-06
- Use only `GatewayIntents::GUILD_VOICE_STATES`, because message intents are not used by this bot.
  - [3450bf7](https://github.com/SSPS-KB/workshop-bot/commit/3450bf71b51d96018f1419b119cad1f6a0e322b6) fix: remove privileded intents on 2023-02-06

## \[0.1.0]

- Add initial changelog
  - [24ac822](https://github.com/SSPS-KB/workshop-bot/commit/24ac82277f37d9e77cedfb5efe95b4444913d000) ci: add covector ([#1](https://github.com/SSPS-KB/workshop-bot/pull/1)) on 2023-02-05
