# Classic Freeze Contract

## Contract
`Classic` mode is treated as parity-frozen. Any drift is a defect unless explicitly approved as a parity bug fix.

## Guard Conditions
1. `GameMode::Classic` must be preserved in bootstrap, runtime, and save/load.
2. Classic content fingerprint must remain stable unless a parity-fix ADR updates baseline.
3. Deterministic classic input traces must produce stable state vectors.
4. Dual-mode additions must not alter classic state vectors for frozen traces.

## Enforcement
Hard checks:

1. `cargo run -p omega-tools --bin classic_mode_drift_guard`
2. `cargo run -p omega-tools --bin parity_certify`
3. `cargo run -p omega-tools --bin true_parity_refresh`

## Update Policy
If a classic baseline change is intentional:

1. Fix parity defect.
2. Regenerate classic drift artifact.
3. Update this document with rationale and linked evidence artifact.
4. Keep `true_parity_refresh` green in the same commit.

