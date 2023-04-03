# Changelog

## \[0.4.0]

- Add /amimir command
  - [516f189](https://github.com/SSPS-KB/workshop-bot/commit/516f1895931a105a31a3f2b0927591a0e6a8051b) amimir command ([#20](https://github.com/SSPS-KB/workshop-bot/pull/20)) on 2023-03-30
  - [2ac93f4](https://github.com/SSPS-KB/workshop-bot/commit/2ac93f448cf66bc774e4b442f34c852953401bbd) Update TRNKA.md ([#21](https://github.com/SSPS-KB/workshop-bot/pull/21)) on 2023-03-30
  - [d32ca1b](https://github.com/SSPS-KB/workshop-bot/commit/d32ca1b21820f432cede0cf3a3a99a9913bffc7e) Windownt command ([#24](https://github.com/SSPS-KB/workshop-bot/pull/24)) on 2023-04-01
  - [cf9527e](https://github.com/SSPS-KB/workshop-bot/commit/cf9527ec8d2adcf115284a2b992389bcf084ad74) fix: changes files on 2023-04-03
- Add /fr command
  - [6cefde1](https://github.com/SSPS-KB/workshop-bot/commit/6cefde1815a88d81eeb68cd4354f24314e3463e1) added /fr comand ([#23](https://github.com/SSPS-KB/workshop-bot/pull/23)) on 2023-03-31
  - [d32ca1b](https://github.com/SSPS-KB/workshop-bot/commit/d32ca1b21820f432cede0cf3a3a99a9913bffc7e) Windownt command ([#24](https://github.com/SSPS-KB/workshop-bot/pull/24)) on 2023-04-01
  - [cf9527e](https://github.com/SSPS-KB/workshop-bot/commit/cf9527ec8d2adcf115284a2b992389bcf084ad74) fix: changes files on 2023-04-03
- Add a /windont command
  - [d32ca1b](https://github.com/SSPS-KB/workshop-bot/commit/d32ca1b21820f432cede0cf3a3a99a9913bffc7e) Windownt command ([#24](https://github.com/SSPS-KB/workshop-bot/pull/24)) on 2023-04-01
  - [cf9527e](https://github.com/SSPS-KB/workshop-bot/commit/cf9527ec8d2adcf115284a2b992389bcf084ad74) fix: changes files on 2023-04-03

## \[0.3.3]

- Add a /cat command that sends a random kitty review gif
  - [8c2b9ea](https://github.com/SSPS-KB/workshop-bot/commit/8c2b9eaffe837ce5278c9101f4eb7692781e78d6) feat: add /cat command ([#16](https://github.com/SSPS-KB/workshop-bot/pull/16)) on 2023-02-21
- Add a /chad command
  - [7e60938](https://github.com/SSPS-KB/workshop-bot/commit/7e6093854f014dde381edc4bee0da84b939e71cc) feat: add chad command on 2023-02-12
- Add a /skull command
  - [f094264](https://github.com/SSPS-KB/workshop-bot/commit/f0942644c27e53a19085c2e65b2a34bb927a5f4b) feat: add /skull command ([#11](https://github.com/SSPS-KB/workshop-bot/pull/11)) on 2023-02-17

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
