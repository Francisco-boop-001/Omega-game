# Milestone 5 TUI E2E Journey Automation (M5-004)

Status: Complete
Date: 2026-02-06
Ticket: `M5-004`

## Objective

Implement scripted end-to-end journey automation for the TUI path and publish report artifacts for M5 gating.

## Delivered

- New runner: `crates/omega-tools/src/bin/m5_e2e_journey.rs`
- Generated artifacts:
  - `target/m5-e2e-journey-report.json`
  - `target/m5-e2e-journey-report.md`
- Integrated into daily M5 gate execution:
  - `scripts/run-m5-gate.ps1`
  - `.github/workflows/m5-daily.yml`

## Journey Coverage

Implemented scripted flow for TUI:
- `new_game`
- `save`
- `mutate_after_save` (validates divergence after save point)
- `load`
- `game_over` (simulated by forcing player HP to `0`)
- `restart`

Current output status at M5-004 closure:
- TUI run: `PASS`
- Bevy frontend pending at that time (`M5-005`)

## Execution Evidence

Commands used:

```powershell
cargo run -p omega-tools --bin m5_e2e_journey
powershell -ExecutionPolicy Bypass -File .\scripts\run-m5-gate.ps1
```

Observed:
- `m5_e2e_journey` exits successfully and writes both report artifacts.
- `run-m5-gate.ps1` now includes E2E report generation, and M5 artifact coverage increases accordingly.

## Notes

- The current core loop has no enemy-turn damage path, so `game_over` is explicitly simulated for this ticket.
- Bevy coverage is completed by `M5-005` (`docs/migration/MILESTONE5_BEVY_E2E_2026-02-06.md`); the formal `30`-day threshold remains open until daily windows accumulate.
