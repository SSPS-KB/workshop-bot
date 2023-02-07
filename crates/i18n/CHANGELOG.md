# Changelog

## \[0.1.1]

- Instead of using the `env!` macro with `CARGO_MANIFEST_DIR`, which returned the static string of `/path/to/crates/i18n`,
  read this variable at macro runtime to match crate dir.
  - [8bc018b](https://github.com/SSPS-KB/workshop-bot/commit/8bc018bd7a51dab823f3281d56e2cde6f58f84ed) fix: read env at macro runtime on 2023-02-06

## \[0.1.0]

- Add initial changelog
  - [24ac822](https://github.com/SSPS-KB/workshop-bot/commit/24ac82277f37d9e77cedfb5efe95b4444913d000) ci: add covector ([#1](https://github.com/SSPS-KB/workshop-bot/pull/1)) on 2023-02-05
