# Playable Omega Strict Plan (Replacement)

Status: Completed (all tracks closed)  
Effective date: 2026-02-07  
Closure date: 2026-02-07  
Purpose: replace milestone-driven planning with one fixed, code-verified path to a playable Omega game on the Rust stack.

Next phase:
- Full classic feature/content parity planning and execution now lives in
  `docs/migration/FULL_OMEGA_PARITY_RECOVERY_PLAN.md`.

## Plan Replacement and Deprecation

This document supersedes all prior execution plans.

Deprecated as active plans:
- `MODERNIZATION_PLAN.md`
- `docs/migration/MILESTONE4_EXECUTION_PLAN.md`
- `docs/migration/MILESTONE5_EXECUTION_PLAN.md`

Historical evidence docs (closure reviews, scorecards, ticket evidence) remain valid as records, but they are not execution drivers.

## Current Code Reality (Audited and Executed 2026-02-07)

Verified by code and strict gate artifacts:
- Full strict gate passes: `powershell -ExecutionPolicy Bypass -File ./scripts/run-m5-gate.ps1 -StrictArtifactMode`.
- M5 artifact coverage is complete (`15/15`): `target/m5-artifact-summary.md`.
- E2E journey uses natural game-over path (no simulation): `target/m5-e2e-journey-report.md`.
- Lifecycle parity reports natural game-over parity across TUI and Bevy: `target/m5-lifecycle-parity-report.md`.
- Both frontends expose executable launchers:
  - `cargo run -p omega-tui --bin omega-tui-app`
  - `cargo run -p omega-bevy --bin omega-bevy-app`
- Core now has enemy turns, natural defeat, and win-state transitions.
- Content bootstrap is integrated and diagnostics are surfaced in launcher startup.

## Playable Definition (Non-Negotiable Exit Criteria)

A playable Omega on the new stack is reached only when all criteria below are true.

- [x] `P-001` Launchers exist and run.
- [x] `P-002` Natural core lifecycle exists (new/save/load/game-over/restart without forced HP=0 hacks).
- [x] `P-003` Combat loop is bidirectional (enemy turns can damage/kill player).
- [x] `P-004` Map/content-backed play exists.
- [x] `P-005` Frontend parity for lifecycle is reported and passing.
- [x] `P-006` Save/load is user-accessible in both frontends in-session.
- [x] `P-007` E2E journey is real (`simulated_game_over=false`).
- [x] `P-008` Lifecycle parity report is real (no simulation requirement outstanding).
- [x] `P-009` Reliability and perf artifacts are implemented.
- [x] `P-010` Security/release artifacts are implemented.
- [x] `P-011` Strict gate enforcement enabled and passing.
- [x] `P-012` No open gameplay blockers in current gate board constraints.

## Track Status (Checkmarked)

Only tasks mapped to the 12 exit criteria were executed.

### Track A: Runtime Entry and Lifecycle

- [x] `A-001` Add TUI executable launcher (`omega-tui-app`) and startup menu flow.
- [x] `A-002` Add Bevy executable launcher (`omega-bevy-app`) and startup menu flow.
- [x] `A-003` Add lifecycle command surface (new/save/load/restart) and wire to frontends.
- [x] `A-004` Add persistent save-slot handling for local runs.

### Track B: Core Gameplay Completion

- [x] `B-001` Implement enemy turn scheduling and damage resolution in `omega-core`.
- [x] `B-002` Add natural player death transition events from core simulation.
- [x] `B-003` Add minimal win/goal condition and terminal outcome event.
- [x] `B-004` Expand tests for enemy turns, death path, and restart invariants.

### Track C: Content Integration

- [x] `C-001` Integrate `omega-content` maps into game bootstrap path.
- [x] `C-002` Add content-to-core spawn translation (player spawn, monster/item placement baseline).
- [x] `C-003` Add failure diagnostics for invalid/missing content in launcher startup.

### Track D: Gate Hardening and Artifact Completion

- [x] `D-001` Remove game-over simulation from `m5_e2e_journey`; require natural defeat.
- [x] `D-002` Implement boot reliability report (`target/m5-boot-reliability.*`).
- [x] `D-003` Implement perf budget delta report vs frozen baseline (`target/m5-perf-budget-report.*`).
- [x] `D-004` Implement Bevy frame-time report (`target/m5-frame-time-report.*`).
- [x] `D-005` Implement security audit output + weekly fuzz summary artifacts.
- [x] `D-006` Implement release operations checklist artifact generation.
- [x] `D-007` Enable strict mode in CI (`.github/workflows/m5-daily.yml`).

### Track E: Playability Closure

- [x] `E-001` Run full strict gate locally and require PASS.
- [x] `E-002` Update readiness scorecard to reflect strict playable closure.
- [x] `E-003` Publish closure review (`docs/migration/PLAYABLE_OMEGA_CLOSURE_REVIEW.md`).

## Anti-Scope-Creep Rules (Retained)

1. No new milestone labels or parallel plan docs may be created.
2. Any proposed task must cite exactly one exit criterion (`P-001`..`P-012`), or it is deferred.
3. Cosmetic refactors are deferred until after playable closure.
4. Net-new gameplay systems beyond minimum playable (e.g., redesign, content expansion) are deferred.

## Closure Evidence

- Strict gate: `target/m5-gate-check-summary.md`
- Artifact coverage: `target/m5-artifact-summary.md`
- E2E journey: `target/m5-e2e-journey-report.md`
- Lifecycle parity: `target/m5-lifecycle-parity-report.md`
- Boot reliability: `target/m5-boot-reliability.md`
- Perf budget: `target/m5-perf-budget-report.md`
- Frame-time: `target/m5-frame-time-report.md`
- Security audit: `target/m5-security-audit.json`
- Weekly fuzz report: `target/m5-fuzz-weekly-report.md`
- Release operations checklist: `target/m5-release-operations-checklist.md`
- Closure review: `docs/migration/PLAYABLE_OMEGA_CLOSURE_REVIEW.md`
- Readiness scorecard: `docs/migration/PLAYABLE_OMEGA_READINESS_SCORECARD.md`
