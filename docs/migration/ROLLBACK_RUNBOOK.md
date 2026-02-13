# Rollback Runbook (Milestone 4)

Status: Active  
Owner: Release engineering + WS-E + WS-H  
Updated: 2026-02-06

## Scope

This runbook defines the rollback path if the Rust runtime release candidate must be reverted.

## Preconditions

- Current release candidate artifacts are produced by Rust-only path (`.github/workflows/release.yml`).
- Save artifacts are handled through `omega-save` (`version=1` envelope, with `version=0` migration support).
- Gate artifacts are available in `target/`:
  - `m4-gate-check-summary.md`
  - `ws-d-regression-dashboard.json`
  - `ws-d-determinism-report.json`
  - `save-compat-report.json`

## Rollback Triggers

- P0 startup, save/load, or core gameplay defect.
- Determinism regression.
- Severe perf regression beyond agreed budget.
- Release gating failure discovered post-cut.

## Rollback Procedure

1. Freeze promotion:
   - Mark current RC as blocked in release notes.
   - Stop any automated promotion to stable channels.
2. Re-point runtime:
   - Rebuild previous known-good Rust RC from release tag.
   - Do not switch save envelope version.
3. Validate rollback candidate:
   - Run `scripts/run-m4-gate.ps1`.
   - Confirm `target/save-compat-report.md` has zero failures.
4. Publish rollback hotfix:
   - Ship rollback candidate with explicit rollback note.
   - Link defect ticket and mitigation plan.
5. Post-rollback review:
   - File parity defect with severity.
   - Update `docs/migration/PARITY_DEFECT_BOARD.json`.

## Non-Rollbackable Boundaries

- Any save written with a future schema version not supported by current `omega-save` migration path.
- Out-of-band operational changes unrelated to runtime binaries.

## Communication Checklist

- Update release notes and support playbook.
- Notify on-call and support owners.
- Document ETA for corrected RC.
