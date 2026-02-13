# WS-F Execution: omega-tui Vertical Slice

Status: Completed (ratatui runtime integrated)

## Scope from modernization plan

- Event loop + input mapping.
- Main map/status/inventory panels.
- Command dispatch into `omega-core`.

## Delivered in this change

- `omega-tui` app shell with explicit input/action pipeline:
  - `UiKey` -> `UiAction` mapping (`App::map_input`)
  - action dispatch (`App::apply_action`)
  - scripted event loop (`run_scripted_session`)
- Real terminal runtime integration with `ratatui` + `crossterm`:
  - alternate screen + raw mode setup/restore
  - non-blocking input polling
  - frame loop via `Terminal::draw`
  - interactive entrypoint: `run_ratatui_app`
- Command dispatch to `omega-core::step(state, command, rng)` using deterministic RNG.
- Rataui widget/layout rendering:
  - map/status/inventory/log panels in a 2x2 split
  - rendering function: `render_frame`
  - deterministic render snapshot helper for tests: `render_to_string_with_ratatui`
- Tests for:
  - input mapping correctness
  - loop-to-core dispatch and time advancement
  - presence of all required panels in ratatui output

## Interfaces stabilized for cross-stream use

- Frontend command contract: `omega_core::Command`
- Outcome/event contract: `omega_core::Outcome` and `omega_core::Event`
- Frontend runtime shell: `omega_tui::App`

## Runtime Command

```powershell
# Integrator entrypoint inside the crate
# call omega_tui::run_ratatui_app(seed) from your binary/app launcher
```
