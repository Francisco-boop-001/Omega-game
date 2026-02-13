# WS-H: Quality, Security, and Performance

## Scope Delivered

- Property testing via `proptest`:
  - `omega-core` invariant checks for time progression and bounded movement.
  - `omega-save` randomized save round-trip preservation.
- Fuzzing:
  - LibFuzzer targets under `fuzz/` for legacy map parsing and save decode paths.
  - Seed corpus smoke runner: `cargo run -p omega-tools --bin fuzz_smoke`.
- Performance:
  - Baseline runner: `cargo run -p omega-tools --bin perf_baseline`.
  - Budget enforcement: `cargo run -p omega-tools --bin perf_baseline -- --check`.
  - Budget config: `docs/quality/perf-budgets.json`.
- Security:
  - Checklist added at `docs/quality/SECURITY_CHECKLIST.md`.
- Milestone 4 governance:
  - flake policy source: `docs/quality/flake_exclusions.json`
  - parity defect board source: `docs/migration/PARITY_DEFECT_BOARD.json`
  - consolidated gate runner: `scripts/run-m4-gate.ps1`
  - remaining-criteria finalizer: `scripts/finalize-m4-remaining.ps1`

## Suggested Quality Gates

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo nextest run --workspace`
- `cargo run -p omega-tools --bin replay_tool -- --min-scenarios 500`
- `cargo run -p omega-tools --bin determinism_check -- --runs-per-fixture 20`
- `cargo run -p omega-tools --bin frontend_parity`
- `cargo run -p omega-tools --bin save_compat_report`
- `cargo run -p omega-tools --bin fuzz_smoke`
- `cargo run -p omega-tools --bin perf_baseline -- --check`
- `scripts/run-m4-gate.ps1`
