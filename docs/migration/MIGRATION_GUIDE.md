# Runtime Migration Guide (Rust Runtime Default)

Version: 2  
Updated: 2026-02-07

## What Changed

- Runtime path is now defined by Rust workspace crates (`omega-core`, `omega-tui`, `omega-bevy`, `omega-save`, `omega-content`).
- Save/load uses versioned serde envelopes through `omega-save`.
- Replay and parity tooling is generated from `omega-tools`.

## Authoritative Parity Contract

- Active reality baseline: `docs/migration/FULL_OMEGA_PARITY_REALITY_AUDIT_2026-02-07.md`
- Active closure contract: `docs/migration/FULL_OMEGA_PARITY_RECOVERY_PLAN.md`
- Active scorecard: `docs/migration/FULL_OMEGA_PARITY_RECOVERY_SCORECARD.md`
- Active closure review: `docs/migration/FULL_OMEGA_PARITY_RECOVERY_CLOSURE_REVIEW.md`
- Revoked (non-authoritative): `docs/migration/FULL_OMEGA_TRUE_PARITY_PLAN.md`, `docs/migration/FULL_OMEGA_TRUE_PARITY_SCORECARD.md`, `docs/migration/FULL_OMEGA_TRUE_PARITY_CLOSURE_REVIEW.md`
- Historical only (superseded): `docs/migration/CLASSIC_OMEGA_PARITY_EXECUTION_PLAN.md`, `docs/migration/CLASSIC_OMEGA_PARITY_SCORECARD.md`, `docs/migration/CLASSIC_OMEGA_PARITY_CLOSURE_REVIEW.md`, `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md`

## Local Developer Upgrade Steps

1. Build workspace:
   - `cargo build --workspace`
2. Run quality and foundational gates:
   - `scripts/run-m4-gate.ps1`
3. Run parity recovery contract guard (must fail while placeholders/synthetic rendering remain):
   - `cargo run -p omega-tools --bin recovery_contract_guard`
4. Validate content and save compatibility:
   - `cargo run -p omega-tools --bin content_report`
   - `cargo run -p omega-tools --bin save_compat_report`

## Known Compatibility Guarantees

- `v0` and `v1` save inputs are accepted and migrated to `v1`.
- Shared command mapping remains aligned between TUI and Bevy for the `omega-core::Command` surface.
