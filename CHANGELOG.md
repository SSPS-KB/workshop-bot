# Changelog

## \[0.3.2]

- Fixed deploy CI
  - [7db5400](https://github.com/SSPS-KB/workshop-bot/commit/7db54002bbd50dedbe7c8cc5b14021dc6f30b2b3) ci: fx deploy ci on 2023-02-09
  - [7bd2cad](https://github.com/SSPS-KB/workshop-bot/commit/7bd2cadaab1e3c62cdbf21ece767bf8e0e52d940) publish new versions ([#8](https://github.com/SSPS-KB/workshop-bot/pull/8)) on 2023-02-09
  - [7afb478](https://github.com/SSPS-KB/workshop-bot/commit/7afb4787249fbba9e731a9db9005b9d009fb517c) chore: rename change on 2023-02-09

## \[0.3.1]

- Fixed deploy CI
  - [7db5400](https://github.com/SSPS-KB/workshop-bot/commit/7db54002bbd50dedbe7c8cc5b14021dc6f30b2b3) ci: fx deploy ci on 2023-02-09

## \[0.3.0]

- Added logic to also check `/etc/workshop-bot` for configuration
  - [586e83b](https://github.com/SSPS-KB/workshop-bot/commit/586e83b87dbbff34da5b90852ab32935dd47e527) feat: add config checking on 2023-02-09

## \[0.2.1]

- Added /slap and /punch otakugif commands.
  - [21d5d3a](https://github.com/SSPS-KB/workshop-bot/commit/21d5d3abf76ed0730f2671f529c02d1bc0faa581) feat: add slap and punch commands on 2023-02-07

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
