# Modern Slice 01: Objective Journal + Compass

## Scope
- Modern-only, read-only guidance layer.
- No mechanical rule changes.
- Classic behavior remains frozen.

## Implemented
1. Core read-only adapters:
- `active_objective_snapshot(state: &GameState) -> Option<ObjectiveSnapshot>`
- `objective_journal(state: &GameState) -> Vec<ObjectiveSnapshot>`
- `objective_map_hints(state: &GameState) -> Vec<Position>`

2. Modern-only projection:
- Bevy renders objective HUD rows and objective map markers only when `mode == Modern`.
- TUI adds a compact objective summary line only in Modern mode.

3. New strict gates:
- `modern_objective_blackbox_smoke`
- `classic_objective_drift_guard`
- both required in `live_checks_all` and `true_parity_refresh`.

## Artifact Paths
- `target/modern/modern-objective-blackbox-smoke.json`
- `target/classic/classic-objective-drift-guard.json`
- `target/true-parity-regression-dashboard.json`

## Notes
- Objective data is derived from existing quest/progression state.
- Any classic drift in objective projections blocks closure.
