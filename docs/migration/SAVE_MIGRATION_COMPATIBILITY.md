# Save Migration Compatibility Matrix

Updated: 2026-02-06  
Owner: WS-E

## In-Scope Save Versions

- `v0` legacy raw/wrapped states
- `v1` current envelope (`SaveEnvelope`)

## Verification Artifacts

- Fixture source: `crates/omega-tools/fixtures/save-compat/`
- Generated report: `target/save-compat-report.json`
- Command: `cargo run -p omega-tools --bin save_compat_report`

## Matrix

| Source shape | Decoder path | Expected migrated version | Status |
|---|---|---:|---|
| `v0` raw `GameState` JSON | `omega_save::decode_json` | 1 | PASS |
| `v0` envelope wrapper (`payload.game_state`) | `omega_save::decode_json` | 1 | PASS |
| `v1` envelope (`payload.state`) | `omega_save::decode_json` | 1 | PASS |

## Rollback Expectations

- A release rollback must keep `omega-save` support for `v0` and `v1`.
- Rolling back runtime binaries must not alter the persisted envelope version written by current release candidate without an explicit compatibility plan.
- Unknown future versions remain hard errors by design.
