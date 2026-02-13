# Milestone 5 Bevy E2E Journey Automation (M5-005)

Status: Complete
Date: 2026-02-06
Ticket: `M5-005`

## Objective

Implement scripted end-to-end journey automation for the Bevy runtime path and merge it into the shared M5 E2E report.

## Delivered

- Extended runner: `crates/omega-tools/src/bin/m5_e2e_journey.rs`
  - now executes both `tui` and `bevy` journeys in one report.
- Generated artifacts:
  - `target/m5-e2e-journey-report.json`
  - `target/m5-e2e-journey-report.md`
- Integrated into M5 gate flow:
  - `scripts/run-m5-gate.ps1`
  - `.github/workflows/m5-daily.yml`

## Bevy Journey Coverage

Implemented scripted flow for Bevy:
- `new_game` (boot/menu validation)
- `start_session` (enter gameplay)
- `save`
- `mutate_after_save`
- `load`
- `game_over` (simulated by forcing player HP to `0` then dispatching one command)
- `restart`

Current output status:
- TUI run: `PASS`
- Bevy run: `PASS`
- Pending frontends: `none`

## Execution Evidence

Commands used:

```powershell
cargo run -p omega-tools --bin m5_e2e_journey
powershell -ExecutionPolicy Bypass -File .\scripts\run-m5-gate.ps1
```

Observed:
- Combined E2E report now contains `2` runs (`tui`, `bevy`) with zero failures.
- M5 gate summary remains PASS in skeleton mode and reports E2E artifacts as present.

## Notes

- As with M5-004, `game_over` is simulated because current core flow does not model enemy-turn damage to drive a natural player death path yet.
- Formal section `5.3` E2E threshold remains PARTIAL until the `30`-day consecutive run window is accumulated in daily workflow history.
