# Dual-Mode Parity Certification

## Required Artifacts
1. `target/classic/classic-mode-drift-guard.json`
2. `target/modern/modern-mode-smoke.json`
3. `target/dual/dual-mode-blackbox-suite.json`
4. `target/mode-artifact-integrity-guard.json`
5. `target/true-parity-regression-dashboard.json`

## Certification Command Set
1. `cargo run -p omega-tools --bin classic_mode_drift_guard`
2. `cargo run -p omega-tools --bin modern_mode_smoke`
3. `cargo run -p omega-tools --bin dual_mode_blackbox_suite`
4. `cargo run -p omega-tools --bin mode_artifact_integrity_guard`
5. `cargo run -p omega-tools --bin true_parity_refresh`
6. `cargo test --workspace`

## Pass Criteria
1. Classic drift guard: `pass=true`
2. Modern smoke: `pass=true`
3. Dual black-box suite: `pass=true`
4. Artifact integrity guard: `pass=true`
5. True parity dashboard: `status=PASS`
6. Workspace tests: all green

## Failure Policy
Any failing gate is blocking:

1. classify defect
2. patch minimal coherent fix
3. rerun failed gate + dependent gates
4. rerun full certification set

