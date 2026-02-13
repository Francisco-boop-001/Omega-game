# Milestone 5 Execution Plan: Productization, Reliability, and Extensibility

> Deprecated as an active execution plan on 2026-02-07.
> Superseded by `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md`.


Status: Active (M5-001 through M5-006 complete)
Owner: Modernization leads (WS-A/B/C/D/E/F/G/H/I + Release + Support)
Updated: 2026-02-06
Depends on:
- `docs/migration/MILESTONE4_CLOSURE_REVIEW.md`
- `docs/migration/MILESTONE4_DECOMMISSION_CHECKLIST.md`
- `docs/migration/MILESTONE4_READINESS_SCORECARD.md`

## Objective

Move from "parity achieved" to "ship-grade modern product" by hardening runtime reliability, product UX, release operations, and extension workflows while preserving deterministic core behavior.

## Outcomes Required for Milestone 5 Closure

- Productized gameplay loop across both frontends with stable startup/save/load/restart journeys.
- Operational reliability and performance budgets enforced by CI/release gates.
- Security and supply-chain checks running on a fixed cadence with ownership.
- Versioned content/mod compatibility contract and tooling for validation.
- Contributor and support workflows updated for faster safe iteration.

## Current Baseline (as of 2026-02-06)

- Milestone 4 closure approved and decommission checklist is fully PASS.
- Legacy C release grace path retired; Rust-only release path is active.
- Daily parity/regression artifacts exist and can be reused as M5 guardrails.
- Core quality gates (`fmt`, `clippy`, `test`) already pass in workspace.

## Scope Pillars

1. Productization and UX consistency
2. Runtime reliability and performance
3. Content/mod extensibility contracts
4. Security and release engineering
5. Developer experience and operational handoff

## Execution Phases

## Phase 0: Charter and Baseline Lock (Week 1)

Goal:
- Establish a frozen baseline and ratified M5 scorecard before feature expansion.

Work packages:
- A2-01: Ratify section `5.3` thresholds and owners in weekly governance review. (Complete 2026-02-06; evidence: `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.md`)
- H5-01: Freeze Milestone 4 perf/stability baseline artifacts for comparison. (Complete 2026-02-06; evidence: `target/m5-m4-baseline-freeze.json`, `docs/migration/MILESTONE5_BASELINE_FREEZE_2026-02-06.md`)
- D4-01: Ensure M4 regression harness remains active as a non-negotiable guardrail. (Complete 2026-02-06 via `scripts/run-m5-gate.ps1`)
- I3-01: Publish M5 operating cadence and escalation matrix.
- A2-03: Stand up M5 daily workflow skeleton and artifact summary reporting. (Complete 2026-02-06; evidence: `.github/workflows/m5-daily.yml`, `docs/migration/MILESTONE5_DAILY_WORKFLOW_2026-02-06.md`)

Exit criteria:
- Baseline artifacts published under `target/` and linked in scorecard.
- Owner assigned for each M5 gate.
- M5 readiness scorecard committed.

## Phase 1: Client Productization (Weeks 2-4)

Goal:
- Make both frontends reliable for day-to-day play with equivalent core command behavior.

Work packages:
- F3-01: End-to-end scripted journey tests for TUI (new/save/load/gameover/restart). (Complete 2026-02-06; evidence: `target/m5-e2e-journey-report.json`, `docs/migration/MILESTONE5_TUI_E2E_2026-02-06.md`)
- G3-01: End-to-end scripted journey tests for Bevy runtime app loop. (Complete 2026-02-06; evidence: `target/m5-e2e-journey-report.json`, `docs/migration/MILESTONE5_BEVY_E2E_2026-02-06.md`)
- F3-02/G3-02: Input profile and accessibility pass (rebinding schema + defaults).
- B5-01: Core session-state edge handling hardening (resume/restart/empty-save behavior).
- D4-02: Shared frontend contract assertions expanded to include lifecycle events. (Complete 2026-02-06; evidence: `target/m5-lifecycle-parity-report.json`, `docs/migration/MILESTONE5_LIFECYCLE_PARITY_2026-02-06.md`)

Exit criteria:
- E2E journey tests pass in CI for both frontends.
- Shared command and lifecycle parity reports are published per run.
- No open P0 in startup/save/load/gameover paths.

## Phase 2: Content and Mod Contract Hardening (Weeks 3-6)

Goal:
- Define and enforce versioned compatibility rules for content and mods.

Work packages:
- C4-01: Content schema versioning policy and compatibility table.
- C4-02: Content validation CLI emits machine-readable compatibility report.
- E4-01: Save envelope metadata extension for content-pack/mod fingerprinting.
- E4-02: Compatibility failure modes and user-facing migration errors standardized.
- I3-02: Modder migration guide and examples updated and versioned.

Exit criteria:
- Compatibility matrix published and consumed in release checklist.
- Save files encode compatibility metadata required for support triage.
- Invalid/incompatible content fails early with actionable diagnostics.

## Phase 3: Reliability, Security, and Release Operations (Weeks 4-8)

Goal:
- Convert quality checks into production-grade release gates.

Work packages:
- H5-02: Boot reliability smoke matrix (`>=99.95%` target) across supported OS builds.
- H5-03: Bevy p99 frame-time benchmark scene integrated into CI checks.
- H5-04: M5 perf gate (`<=5%` turn-latency regression) integrated into release check.
- H5-05: Daily fuzz smoke + weekly deep-fuzz report publication.
- A2-02: Dependency audit integration and release-blocking policy for high/critical findings.
- Release-04: Cross-platform artifact verification and install/update/rollback drill template.

Exit criteria:
- Section `5.3` reliability/perf/security gates are measurable in CI artifacts.
- Two consecutive RC cycles run new release operations checklist without major escape.
- Rollback drill evidence attached for each RC cycle.

## Phase 4: Launch Stabilization and Handoff (Weeks 8-10)

Goal:
- Lock Milestone 5 with operational confidence and handoff readiness.

Work packages:
- Release-05: 30-day stabilization window tracking with weekly review.
- Ops-02: Support playbook updates for M5-era diagnostics and compatibility handling.
- I3-03: Contributor onboarding dry-run validation for command/effect/content extension docs.
- Governance-02: Final M5 closure review and sign-off package.

Exit criteria:
- All scorecard gates PASS for required windows.
- Support and contributor playbooks validated by dry-run evidence.
- Milestone 5 closure review approved.

## Governance Cadence

- Daily:
  - Run guardrail gate workflows and publish artifact summaries.
  - Triage P0/P1 and flake exceptions with owner + expiry.
- Weekly:
  - Review scorecard status, unblock critical path, update risk register.
  - Validate override records and remove expired waivers.
- Per RC:
  - Execute cross-platform artifact verification.
  - Execute install/update/rollback drill and attach evidence.

## Artifact Contract

Required artifacts to claim Milestone 5 complete:
- `target/m5-e2e-journey-report.json`
- `target/m5-e2e-journey-report.md`
- `target/m5-lifecycle-parity-report.json`
- `target/m5-lifecycle-parity-report.md`
- `target/m5-boot-reliability.json`
- `target/m5-boot-reliability.md`
- `target/m5-perf-budget-report.json`
- `target/m5-perf-budget-report.md`
- `target/m5-frame-time-report.json`
- `target/m5-frame-time-report.md`
- `target/m5-security-audit.json`
- `target/m5-fuzz-weekly-report.md`
- `target/m5-release-operations-checklist.md`
- `target/m5-m4-baseline-freeze.json`
- `target/m5-m4-baseline-freeze.md`
- `target/m5-gate-check-summary.md`
- `target/m5-artifact-summary.json`
- `target/m5-artifact-summary.md`
- `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.md`
- `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.json`
- `docs/migration/MILESTONE5_BASELINE_FREEZE_2026-02-06.md`
- `docs/migration/MILESTONE5_DAILY_WORKFLOW_2026-02-06.md`
- `docs/migration/MILESTONE5_TUI_E2E_2026-02-06.md`
- `docs/migration/MILESTONE5_BEVY_E2E_2026-02-06.md`
- `docs/migration/MILESTONE5_LIFECYCLE_PARITY_2026-02-06.md`
- `docs/migration/MILESTONE5_READINESS_SCORECARD.md`
- updated mod/content compatibility matrix and contributor playbook references
- final milestone closure review sign-off

## Risk Register (M5)

Risk: Scope creep into feature redesign.
- Mitigation: prioritize reliability/productization gates before net-new gameplay.

Risk: Regression introduced while tightening performance budgets.
- Mitigation: retain M4 parity guardrails as blocking checks for M5 PRs touching core/frontends.

Risk: Mod/content compatibility churn.
- Mitigation: versioned compatibility matrix + explicit deprecation windows.

Risk: CI instability from expanded matrix.
- Mitigation: phase in workloads and maintain strict flake budget with expiry policy.

Risk: Security gate fatigue.
- Mitigation: automate reports and make only high/critical findings release-blocking.

## Initial Ticket Pack (First 15)

1. M5-001: Ratify gate owners and publish governance roster. (Complete 2026-02-06; `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.md`)
2. M5-002: Freeze M4 baseline artifacts for perf and stability. (Complete 2026-02-06; `docs/migration/MILESTONE5_BASELINE_FREEZE_2026-02-06.md`)
3. M5-003: Create M5 daily workflow skeleton and artifact summary. (Complete 2026-02-06; `docs/migration/MILESTONE5_DAILY_WORKFLOW_2026-02-06.md`)
4. M5-004: Implement TUI E2E journey automation. (Complete 2026-02-06; `docs/migration/MILESTONE5_TUI_E2E_2026-02-06.md`)
5. M5-005: Implement Bevy E2E journey automation. (Complete 2026-02-06; `docs/migration/MILESTONE5_BEVY_E2E_2026-02-06.md`)
6. M5-006: Add lifecycle parity report (startup/save/load/restart). (Complete 2026-02-06; `docs/migration/MILESTONE5_LIFECYCLE_PARITY_2026-02-06.md`)
7. M5-007: Add boot-reliability matrix harness.
8. M5-008: Add Bevy frame-time benchmark scene/report.
9. M5-009: Add perf regression check (`<=5%`) against frozen baseline.
10. M5-010: Integrate dependency/security audit output in CI.
11. M5-011: Define content schema version policy and matrix format.
12. M5-012: Extend save metadata with content/mod fingerprint.
13. M5-013: Implement compatibility-report CLI output contract.
14. M5-014: Update modder/contributor playbooks and run onboarding dry-run.
15. M5-015: RC drill template for install/update/rollback validation.

## Definition of Done (Milestone 5)

Milestone 5 is done only if:
- Section `5.3` thresholds are met with linked artifacts.
- Required artifact contract is complete and auditable in-repo.
- Support and contributor workflows are validated by dry-run evidence.
- Final closure review is approved and linked from modernization docs.
