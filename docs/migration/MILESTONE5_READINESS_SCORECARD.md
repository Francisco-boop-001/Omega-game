# Milestone 5 Readiness Scorecard

> Deprecated as the active progress scorecard on 2026-02-07.
> Track playable progress using `docs/migration/PLAYABLE_OMEGA_STRICT_PLAN.md` until closure.


Date: 2026-02-06
Reference:
- `MODERNIZATION_PLAN.md` section `5.3`
- `docs/migration/MILESTONE5_EXECUTION_PLAN.md`

Status legend:
- `PASS`: gate fully satisfied with evidence.
- `PARTIAL`: measurable progress, threshold not fully met.
- `FAIL`: gate currently unmet.
- `N/A`: not yet measured.

## Productization and Runtime-Maturity Gates

| Gate | Threshold | Current Evidence | Status |
|---|---|---|---|
| E2E flow reliability (TUI + Bevy) | 30 consecutive daily passing runs | Combined TUI+Bevy automation report is live (`target/m5-e2e-journey-report.json`); window accumulation pending | PARTIAL |
| Crash-free session rate | >=99.9% over rolling 30 days | M4 window exists (`target/m4-crashfree-window.json`), M5 window not started | PARTIAL |
| Boot-to-interactive reliability | >=99.95% on CI smoke matrix | Matrix harness not yet published | N/A |
| Turn-latency regression | <=5% vs frozen M4 baseline | Baseline is frozen (`target/m5-m4-baseline-freeze.json`); M5 comparison gate not yet running (`target/m5-perf-budget-report.json`) | PARTIAL |
| Bevy p99 frame-time budget | <=16.7ms on benchmark scene | Benchmark scene/report pending | N/A |
| Cold-start budget | <=3.0s on reference hardware | Timing harness pending | N/A |
| Severity gate | 0 open P0 | `docs/migration/PARITY_DEFECT_BOARD.json` currently open_p0=0 | PASS |
| Aging P1 gate | <=2 open P1 older than 7 days | No open defects snapshot exists; aging metric pipeline pending | PARTIAL |
| Flake budget | <=1% excluded tests with owner+expiry | `docs/quality/flake_exclusions.json` present; M5 threshold not yet validated | PARTIAL |
| Security fuzz cadence | daily smoke + weekly deep-fuzz report | fuzz smoke exists; weekly report pipeline pending | PARTIAL |
| Dependency security gate | 0 high/critical at RC cut | Automated audit report not yet attached | N/A |
| Cross-platform RC artifacts | 2 consecutive RCs verified | RC drill template pending | N/A |
| Install/update/rollback drill | 1 drill per RC cycle | Prior rollback dry-run exists for M4; M5 cadence pending | PARTIAL |
| Content/mod compatibility matrix | published and versioned | initial migration docs exist; M5 matrix format pending | PARTIAL |
| Onboarding validation | guide dry-run by contributor | dry-run evidence not yet produced | N/A |
| Gate ownership coverage | 100% of section 5.3 gates have primary+backup owner | `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.md`, `docs/migration/MILESTONE5_GOVERNANCE_ROSTER.json` | PASS |
| M4 baseline freeze package | frozen stability+perf reference with checksums | `target/m5-m4-baseline-freeze.json`, `target/m5-m4-baseline-freeze.md`, `docs/migration/MILESTONE5_BASELINE_FREEZE_2026-02-06.md` | PASS |
| M5 daily workflow skeleton | workflow runs and publishes M5 summary artifacts | `.github/workflows/m5-daily.yml`, `scripts/run-m5-gate.ps1`, `target/m5-gate-check-summary.md`, `target/m5-artifact-summary.json` | PASS |
| TUI E2E automation | scripted TUI journey generates report artifacts | `crates/omega-tools/src/bin/m5_e2e_journey.rs`, `target/m5-e2e-journey-report.json`, `docs/migration/MILESTONE5_TUI_E2E_2026-02-06.md` | PASS |
| Bevy E2E automation | scripted Bevy journey is included in shared E2E report | `crates/omega-tools/src/bin/m5_e2e_journey.rs`, `target/m5-e2e-journey-report.json`, `docs/migration/MILESTONE5_BEVY_E2E_2026-02-06.md` | PASS |
| Lifecycle parity reporting | startup/save/load/restart parity report published each M5 gate run | `crates/omega-tools/src/bin/m5_lifecycle_parity.rs`, `target/m5-lifecycle-parity-report.json`, `docs/migration/MILESTONE5_LIFECYCLE_PARITY_2026-02-06.md` | PASS |

## Immediate Focus (Next 2 Weeks)

1. Define content/mod compatibility matrix format and save metadata extension scope.
2. Start RC operations drill template and attach first dry-run evidence.
3. Implement M5 perf comparison report against the frozen baseline artifacts.
4. Add boot-reliability artifact generation and reporting to raise M5 coverage.
