# Milestone 4 Execution Plan: Feature Parity and Legacy Decommission

> Deprecated as an active execution plan on 2026-02-07.
> Superseded by `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md`.


Status: Closed
Owner: Modernization leads (WS-B/C/D/E/F/G/H/I)
Updated: 2026-02-06

## Objective

Achieve Milestone 4 by meeting all formal gates in:
- `MODERNIZATION_PLAN.md` section `5.1` (Parity Targets)
- `MODERNIZATION_PLAN.md` section `5.2` (Legacy C Decommission Criteria)

## Current Baseline (as of 2026-02-06)

- Workspace quality gates currently pass (`fmt`, `clippy`, `test`).
- WS-D replay harness runs and produces dashboard artifacts.
- Replay suite denominator now meets Milestone 4 requirement (`>=500` scenarios/day) via matrix fixtures.
- Determinism (`N=20` repeats), frontend parity, and save compatibility reports are generated as gate artifacts.
- Frontend and core vertical slices exist, with release-path cutover completed and closure evidence published.

## Execution Phases

## Phase 0: Baseline Stabilization (Complete)

Deliverables:
- Stable workspace test baseline.
- Repeatable gate-check command set and artifacts.

Exit criteria:
- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo test --workspace` passes.
- `replay_tool`, `fuzz_smoke`, and `perf_baseline --check` pass.

## Phase 1: Parity Expansion (WS-B/C/D/E/F/G) (Complete)

Goal:
- Reach a daily replay denominator of `>= 500` scenarios with coverage across all critical paths.

Work packages:
- D3-01: Expanded replay fixtures from smoke set to scenario matrix (movement, combat, status, transitions, save/load tags, inventory).
- D3-02: Added fixture tagging (`critical_path`, `frontend_shared`, `save_compat`, `known_flaky`) and dashboard rollups.
- B4-01: Command semantics and outcome/event ordering validated across replay matrix.
- F2/G2: TUI/Bevy mapping parity report generated (`target/frontend-command-parity.json`).
- E3-01: Save migration fixture corpus/report generated (`target/save-compat-report.json`).

Exit criteria:
- Replay denominator `>= 500` scenarios/day. (PASS)
- Critical-path scenarios exist and are independently reportable. (PASS)
- Frontend mapping parity report is generated on every CI run. (PASS)

## Phase 2: Burn-In and Confidence Window (WS-D/H) (Complete)

Goal:
- Satisfy consecutive-run requirements from section `5.1`.

Work packages:
- H4-01: Daily CI workflow publishes gate artifacts (`.github/workflows/m4-daily.yml`).
- D3-03: Determinism verifier for seeded replays (`N=20` per family) with hash comparison implemented.
- H4-02: Flake tracking policy enforced (`docs/quality/flake_exclusions.json`).
- H4-03: Defect gate board for parity labels (`docs/migration/PARITY_DEFECT_BOARD.json`).

Exit criteria:
- 14/14 burn-in windows at required thresholds. (PASS, see `target/m4-burnin-window.json`)
- Determinism and critical-path targets satisfied. (PASS)
- Defect gates satisfied. (PASS)

## Phase 3: Decommission Readiness (WS-E/H/I + Release) (Complete)

Goal:
- Meet all section `5.2` decommission criteria.

Work packages:
- I2-01: Published rollback runbook and staged dry-run evidence.
- E3-02: Published save migration and rollback compatibility table.
- Release-01: RC notes recorded for two consecutive Rust-default RC artifacts.
- Ops-01: Release workflow now defaults to Rust runtime and legacy C grace path is removed.
- I2-02: Migration guide and support/on-call playbook published.
- Release-02: Retirement policy notes published.

Exit criteria:
- All decommission checklist items marked PASS with linked evidence.

## Phase 4: Cutover and Legacy Retirement (Complete)

Goal:
- Formally retire legacy C runtime from release path and close Milestone 4.

Work packages:
- Release-03: Remove grace-path legacy release controls.
- Repo-01: Archive/delete legacy runtime files per ownership decisions.
- Governance-01: Final Milestone 4 closure review and sign-off.

Exit criteria:
- Milestone 4 checklist shows all PASS. (PASS)
- Closure review approved by stream leads. (PASS, see `docs/migration/MILESTONE4_CLOSURE_REVIEW.md`)

## Governance Cadence

- Daily:
  - Run gate check and publish `target/m4-gate-check-summary.md`.
  - Update parity defect board.
- Weekly:
  - Post-closure regression review against formal scorecard.
  - Freeze/unfreeze window decisions for risky changes.
- Per RC:
  - Decommission checklist review with release engineering and support owners.

## Artifact Contract

Required artifacts to claim Milestone 4 complete:
- `target/ws-d-regression-dashboard.json`
- `target/ws-d-regression-dashboard.md`
- `target/ws-d-determinism-report.json`
- `target/ws-d-determinism-report.md`
- `target/frontend-command-parity.json`
- `target/frontend-command-parity.md`
- `target/save-compat-report.json`
- `target/save-compat-report.md`
- `target/m4-gate-check-summary.md`
- rollback runbook + dry-run evidence
- save migration compatibility matrix
- two RC release notes showing Rust default path
- migration/support/on-call updated docs
- closure review sign-off
