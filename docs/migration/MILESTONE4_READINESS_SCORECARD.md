# Milestone 4 Readiness Scorecard

> Deprecated as an active scorecard on 2026-02-07.
> Historical record only; active execution tracking is in `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md`.

Date: 2026-02-06
Reference: `MODERNIZATION_PLAN.md` sections `5.1` and `5.2`

Status legend:
- `PASS`: gate fully satisfied with evidence.
- `PARTIAL`: directionally good, but formal threshold not yet met.
- `FAIL`: gate currently unmet.
- `N/A`: not yet measurable.

## 5.1 Parity Targets

| Gate | Threshold | Current Evidence | Status |
|---|---|---|---|
| Golden scenario pass rate | >=98.0% for 14 daily runs | `target/m4-burnin-window.json` shows 14/14 passing windows with replay pass-rate gate satisfied | PASS |
| Critical-path pass rate | 100% for 14 daily runs | `target/m4-burnin-window.json` + `target/ws-d-regression-dashboard.json` show zero critical-path failures across 14 windows | PASS |
| Determinism | 100% for N=20 seeded runs per family | `target/ws-d-determinism-report.json` generated with N=20 repeats | PASS |
| Save compatibility | 100% for all in-scope legacy versions | `target/save-compat-report.json` + fixture corpus in `crates/omega-tools/fixtures/save-compat/` | PASS |
| Severity gate | 0 P0 and <=3 P1 | `docs/migration/PARITY_DEFECT_BOARD.json` snapshot | PASS |
| Frontend command parity | 100% TUI/Bevy shared command mapping | `target/frontend-command-parity.json` generated in CI | PASS |
| Replay denominator | >=500 scenarios/day | matrix fixtures in `crates/omega-tools/fixtures/replay/matrix/` + dashboard denominator | PASS |
| Flake policy | <2% excluded with issue+owner | `docs/quality/flake_exclusions.json` policy + exclusions | PASS |

## 5.2 Legacy Runtime Decommission

| Gate | Threshold | Current Evidence | Status |
|---|---|---|---|
| Rust default release path | 2 consecutive RCs | `docs/migration/RC-2026-02-06-01.md` + `docs/migration/RC-2026-02-06-02.md` | PASS |
| Legacy runtime default usage | not invoked by default | release workflow uses Rust-only path (`.github/workflows/release.yml`) | PASS |
| Rollback runbook | documented + staged dry-run | `docs/migration/ROLLBACK_RUNBOOK.md` + `docs/migration/ROLLBACK_DRY_RUN_2026-02-06.md` | PASS |
| Save rollback expectations | documented | `docs/migration/SAVE_MIGRATION_COMPATIBILITY.md` | PASS |
| Crash-free rate | >=99.5% over 14-day pre-release window | `target/m4-crashfree-window.json` reports 100% over 14-window burn-in (`8442/8442`) | PASS |
| P0 operational defects | 0 open in critical paths | parity defect board snapshot currently `open_p0=0` | PASS |
| Perf regression budget | <=10% median turn latency | `perf_baseline -- --check` included in M4 gate script | PASS |
| CI release jobs | legacy C removed from required jobs | Rust-only release workflow in `.github/workflows/release.yml` | PASS |
| Grace-release policy | optional/manual only then removed | legacy grace path removed from release workflow (`.github/workflows/release.yml`) | PASS |
| Legacy file ownership | archive/delete decision recorded | archived manifest in `archive/legacy-c-runtime/2026-02-06/MANIFEST.json` + ownership record | PASS |
| Migration guide | published/versioned | `docs/migration/MIGRATION_GUIDE.md` | PASS |
| Retirement release notes | date + policy announced | `docs/migration/RETIREMENT_RELEASE_NOTES.md` | PASS |
| Support playbook | Rust runtime paths only | `docs/migration/SUPPORT_PLAYBOOK.md` | PASS |

## Immediate Focus (Next 2 Weeks)

1. Keep `m4-daily` running for ongoing regression guardrails after Milestone 4 closure.
2. Continue attaching gate window artifacts to each RC.
3. Track post-closure parity drift and update defect board snapshots.
4. Execute Milestone 5 kickoff plan and begin scorecard tracking (`docs/migration/MILESTONE5_EXECUTION_PLAN.md`, `docs/migration/MILESTONE5_READINESS_SCORECARD.md`).
