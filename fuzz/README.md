# Omega Fuzz Targets

This directory contains libFuzzer targets for parser/save hardening.

## Targets

- `fuzz_content_map_parser`: exercises `omega-content` legacy map parsing.
- `fuzz_save_decode`: exercises `omega-save` decode/migration paths.

## Run locally

1. Install cargo-fuzz:
   - `cargo install cargo-fuzz`
2. Run a target:
   - `cargo fuzz run fuzz_content_map_parser`
   - `cargo fuzz run fuzz_save_decode`

## Seed corpus smoke (CI-friendly)

Use deterministic corpus smoke checks without requiring cargo-fuzz:

- `cargo run -p omega-tools --bin fuzz_smoke`
