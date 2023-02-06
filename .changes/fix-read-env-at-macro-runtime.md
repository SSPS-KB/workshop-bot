---
"i18n": patch
---
Instead of using the `env!` macro with `CARGO_MANIFEST_DIR`, which returned the static string of `/path/to/crates/i18n`,
read this variable at macro runtime to match crate dir.
