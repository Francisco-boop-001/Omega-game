# Swarm Execution Board

This board defines parallel streams and ownership for the modernization effort.

## Agent A: Core Simulation
- Owns `crates/omega-core`.
- Plan: build deterministic state model, command pipeline, and turn systems.
- Deliverables: `GameState`, `Command`, `Outcome`, invariants + tests.

## Agent B: Content Pipeline
- Owns `crates/omega-content` and conversion tools in `omega-tools`.
- Plan: parse legacy maps/content, validate, and expose typed content APIs.
- Deliverables: loaders, validators, migration reports.

## Agent C: Save System
- Owns `crates/omega-save`.
- Plan: schema envelope, migrations, corruption handling, round-trip guarantees.
- Deliverables: v1 save schema + migration harness.

## Agent D: TUI Frontend
- Owns `crates/omega-tui`.
- Plan: input loop, command dispatch, map/status rendering, message log.
- Deliverables: playable terminal vertical slice.

## Agent E: Bevy Frontend
- Owns `crates/omega-bevy`.
- Plan: app states, rendering adapter, HUD, input mapping parity.
- Deliverables: playable graphical vertical slice.

## Agent F: Verification and Tooling
- Owns replay tests, property tests, fuzzing, and CI quality gates.
- Plan: parity harness and performance regression checks.
- Deliverables: golden replay suite + quality dashboard.

## Integration Rules
- Shared contracts frozen in `omega-core` before large frontend work.
- Weekly integration merge with replay parity checks.
- Frontends consume core APIs; no direct coupling between frontends.
- Workstreams must be claimed via `scripts/ws-lock.ps1` before edits.
- Lock protocol details: `swarm/LOCKING.md`.
