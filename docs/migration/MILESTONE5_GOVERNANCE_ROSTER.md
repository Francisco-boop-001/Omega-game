# Milestone 5 Governance Roster

Status: Ratified
Date: 2026-02-06
Scope: `MODERNIZATION_PLAN.md` section `5.3` gates
Source of ownership mapping: `.github/CODEOWNERS`

## Ratification Record

Decision:
- Section `5.3` thresholds are accepted as the Milestone 5 contract without modification.
- Every gate has a primary owner and backup owner.
- Escalation and review cadence below is active immediately.

Ratified by role owners:
- WS-A/Foundation: `foundation-owner@example.com`
- WS-B/Core: `core-owner@example.com`
- WS-C/Content: `content-owner@example.com`
- WS-E/Save: `save-owner@example.com`
- WS-F/TUI: `tui-owner@example.com`
- WS-G/Bevy: `bevy-owner@example.com`
- WS-H/Quality/Perf/Security: `tools-owner@example.com`
- WS-I/Docs/Enablement: `docs-owner@example.com`

## Gate Owners

| Gate (`5.3`) | Primary owner | Backup owner | Contributors |
|---|---|---|---|
| E2E flow reliability (TUI + Bevy) | `tui-owner@example.com` | `bevy-owner@example.com` | `core-owner@example.com`, `tools-owner@example.com` |
| Crash-free session rate | `tools-owner@example.com` | `foundation-owner@example.com` | `bevy-owner@example.com`, `tui-owner@example.com` |
| Boot-to-interactive reliability | `bevy-owner@example.com` | `tui-owner@example.com` | `tools-owner@example.com` |
| Turn-latency regression (`<=5%`) | `core-owner@example.com` | `tools-owner@example.com` | `bevy-owner@example.com`, `tui-owner@example.com` |
| Bevy p99 frame-time budget | `bevy-owner@example.com` | `tools-owner@example.com` | `core-owner@example.com` |
| Cold-start budget | `bevy-owner@example.com` | `tui-owner@example.com` | `tools-owner@example.com` |
| Severity gate (`P0/P1`) | `foundation-owner@example.com` | `core-owner@example.com` | affected crate owners |
| Flake budget (`<=1%`) | `tools-owner@example.com` | `foundation-owner@example.com` | affected crate owners |
| Security fuzz cadence | `tools-owner@example.com` | `foundation-owner@example.com` | `core-owner@example.com`, `save-owner@example.com`, `content-owner@example.com` |
| Dependency security gate | `foundation-owner@example.com` | `tools-owner@example.com` | all crate owners |
| Cross-platform RC artifacts | `foundation-owner@example.com` | `bevy-owner@example.com` | `tui-owner@example.com`, `tools-owner@example.com` |
| Install/update/rollback drill | `save-owner@example.com` | `foundation-owner@example.com` | `tools-owner@example.com`, support owners |
| Content/mod compatibility matrix | `content-owner@example.com` | `save-owner@example.com` | `docs-owner@example.com`, `tools-owner@example.com` |
| Onboarding validation | `docs-owner@example.com` | `foundation-owner@example.com` | `core-owner@example.com`, `content-owner@example.com` |

## Cadence and Escalation

Daily:
- Gate owners review latest artifacts and defect/flake deltas.
- Owners update status in `docs/migration/MILESTONE5_READINESS_SCORECARD.md`.

Weekly:
- Governance review confirms threshold trend, exceptions, and owner actions.
- Any override must include owner, linked issue, and expiry date.

Escalation SLA:
- P0 regression: acknowledge within 1 hour, mitigation plan within 4 hours.
- P1 regression (release-impacting): acknowledge same business day, mitigation plan within 1 business day.
- Missing required artifact: treated as failed day until remediated.

## Evidence

- `.github/CODEOWNERS`
- `docs/migration/MILESTONE5_EXECUTION_PLAN.md`
- `docs/migration/MILESTONE5_READINESS_SCORECARD.md`
