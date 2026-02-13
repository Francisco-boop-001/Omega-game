# Milestone 4 Decommission Checklist

Updated: 2026-02-06

| Gate | Evidence | Status |
|---|---|---|
| Replay denominator >=500/day | `target/ws-d-regression-dashboard.json` | PASS |
| Determinism N=20 per family | `target/ws-d-determinism-report.json` | PASS |
| Frontend parity 100% shared commands | `target/frontend-command-parity.json` | PASS |
| Save compatibility in-scope versions | `target/save-compat-report.json`, `docs/migration/SAVE_MIGRATION_COMPATIBILITY.md` | PASS |
| Flake policy <2% with owner/issue | `docs/quality/flake_exclusions.json` | PASS |
| Severity gate (P0=0, P1<=3) | `docs/migration/PARITY_DEFECT_BOARD.json` | PASS |
| Rollback runbook + dry-run | `docs/migration/ROLLBACK_RUNBOOK.md`, `docs/migration/ROLLBACK_DRY_RUN_2026-02-06.md` | PASS |
| CI release jobs use Rust path by default | `.github/workflows/release.yml` | PASS |
| Legacy grace path removed | `.github/workflows/release.yml` | PASS |
| Legacy file ownership recorded and executed | `docs/migration/LEGACY_FILE_OWNERSHIP.md`, `archive/legacy-c-runtime/2026-02-06/MANIFEST.json` | PASS |
| Migration guide published | `docs/migration/MIGRATION_GUIDE.md` | PASS |
| Support playbook updated | `docs/migration/SUPPORT_PLAYBOOK.md` | PASS |
| Two RC notes with Rust default path | `docs/migration/RC-2026-02-06-01.md`, `docs/migration/RC-2026-02-06-02.md` | PASS |
| 14 consecutive daily run window | `target/m4-burnin-window.json`, `target/m4-burnin-window.md`, `docs/migration/M4_WINDOW_METHODOLOGY.md` | PASS |
| Crash-free 14-day operational window | `target/m4-crashfree-window.json`, `target/m4-crashfree-window.md`, `docs/migration/M4_WINDOW_METHODOLOGY.md` | PASS |
| Final closure review sign-off | `docs/migration/MILESTONE4_CLOSURE_REVIEW.md` | PASS |
