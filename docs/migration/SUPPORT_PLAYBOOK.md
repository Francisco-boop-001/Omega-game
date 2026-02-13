# Support Playbook (Rust Runtime Paths)

Updated: 2026-02-06  
Scope: active releases

## First Response Checklist

1. Confirm runtime artifact source is Rust release workflow (`.github/workflows/release.yml`).
2. Collect gate artifacts from the failing build:
   - `target/m4-gate-check-summary.md`
   - `target/ws-d-regression-dashboard.md`
   - `target/ws-d-determinism-report.md`
   - `target/save-compat-report.md`
3. Classify issue severity (`P0`, `P1`, `P2`) and update `docs/migration/PARITY_DEFECT_BOARD.json`.

## Save/Load Incident Handling

- Reproduce with `crates/omega-tools/fixtures/save-compat/` patterns.
- Run:
  - `cargo run -p omega-tools --bin save_compat_report`
- If decode fails on in-scope versions (`v0`, `v1`), treat as release-blocking.

## Parity Incident Handling

- Reproduce via:
  - `cargo run -p omega-tools --bin replay_tool -- --min-scenarios 500`
  - `cargo run -p omega-tools --bin determinism_check -- --runs-per-fixture 20`
- Validate frontend mapping:
  - `cargo run -p omega-tools --bin frontend_parity`

## Escalation

- P0: immediate rollback decision path (see `docs/migration/ROLLBACK_RUNBOOK.md`).
- P1: mitigation plus hotfix plan in next RC window.
