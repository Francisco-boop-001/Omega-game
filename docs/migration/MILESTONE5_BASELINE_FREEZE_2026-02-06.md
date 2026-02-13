# Milestone 5 Baseline Freeze (M5-002)

Status: Complete
Date: 2026-02-06
Ticket: `M5-002`

## Objective

Freeze a Milestone 4 reference baseline for M5 comparisons (stability + performance) and store immutable snapshot artifacts with checksums.

## Execution

Command used:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\freeze-m4-baseline.ps1 -BaselineDate 2026-02-06 -PerfIterations 200
```

Generated artifacts:
- `target/m5-m4-baseline-freeze.json`
- `target/m5-m4-baseline-freeze.md`
- `target/m5-baseline/2026-02-06/` (copied source artifacts + checksums)

## Baseline Snapshot

- Overall M4 gate status: `PASS`
- Replay scenarios: `603` (pass `603`, fail `0`)
- Replay pass rate: `100%`
- Critical-path failures: `0` of `563`
- Determinism divergent runs: `0`
- Frontend parity mismatches: `0`
- Save compatibility failures: `0`
- Burn-in status: `PASS`
- Crash-free status: `PASS` (`100%`)

Performance baseline (`iterations=200`):
- `avg_ms`: `0.0845865`
- `p95_ms`: `0.0904`
- `max_ms`: `0.1261`

## Integrity and Provenance

- Source gate runner: `scripts/run-m4-gate.ps1`
- Freeze script: `scripts/freeze-m4-baseline.ps1`
- File-level SHA256 manifest is embedded in `target/m5-m4-baseline-freeze.json` and summarized in `target/m5-m4-baseline-freeze.md`.

## Notes

- This baseline is the reference anchor for the M5 turn-latency regression gate (`<=5%` vs frozen M4 baseline).
