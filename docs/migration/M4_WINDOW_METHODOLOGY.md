# M4 Window Methodology (Remaining Criteria Finalization)

Updated: 2026-02-06

## Goal

Close the two remaining Milestone 4 criteria with auditable evidence artifacts:

1. Consecutive 14-window parity burn-in.
2. Crash-free rate >=99.5 over the same 14-window validation set.

## Method

- Script: `scripts/finalize-m4-remaining.ps1`
- Inputs:
  - `scripts/run-m4-gate.ps1`
  - `target/m4-gate-check-summary.md`
- Outputs:
  - `target/m4-burnin-window.json`
  - `target/m4-burnin-window.md`
  - `target/m4-crashfree-window.json`
  - `target/m4-crashfree-window.md`

The script executes 14 consecutive full M4 gate runs and aggregates replay-session stability metrics from each run.

## Current Evidence Snapshot

- Burn-in windows completed: 14/14 (`PASS`)
- Crash-free rate: 100% (`8442/8442`, `PASS`)
