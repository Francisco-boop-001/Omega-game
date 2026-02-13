# WS-D Parity Harness

Status: Completed (expanded matrix + governance gates)
Date: 2026-02-06

## Deliverables
- Golden scenarios for movement, combat, inventory, and save-compat tagged paths.
- Replay fixture format + runner in `omega-tools`.
- Expanded fixture matrix (`>=500` scenarios/day denominator).
- Regression dashboard generation with tag/family rollups.
- Determinism report (`N=20` repeated seeded runs per family).

## Files
- `crates/omega-tools/src/replay.rs`
- `crates/omega-tools/src/bin/replay_tool.rs`
- `crates/omega-tools/src/bin/replay_matrix_gen.rs`
- `crates/omega-tools/src/bin/determinism_check.rs`
- `crates/omega-tools/fixtures/replay/movement_blocked_wall.json`
- `crates/omega-tools/fixtures/replay/combat_defeat_monster.json`
- `crates/omega-tools/fixtures/replay/pickup_drop_cycle.json`
- `crates/omega-tools/fixtures/replay/matrix/*.json`

## Run
```powershell
cargo run -p omega-tools --bin replay_matrix_gen
cargo run -p omega-tools --bin replay_tool
cargo run -p omega-tools --bin determinism_check -- --runs-per-fixture 20
```

## Outputs
- `target/ws-d-regression-dashboard.json`
- `target/ws-d-regression-dashboard.md`
- `target/ws-d-determinism-report.json`
- `target/ws-d-determinism-report.md`
