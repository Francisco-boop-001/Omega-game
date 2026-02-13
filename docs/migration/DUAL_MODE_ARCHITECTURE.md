# Dual-Mode Architecture

## Goal
Support two playable runtime modes with strict isolation:

1. `Classic`: parity-frozen behavior/content.
2. `Modern`: independently evolving behavior/content.

## Core Design
1. `omega-core` carries `GameState.mode: GameMode`.
2. Mode dispatch is centralized in `crates/omega-core/src/core/mode/`.
3. Subsystem policy hooks are executed at orchestration boundaries (`step()` before/after command).
4. Content is loaded through mode-scoped pack selection (`omega-content`).
5. Saves are mode-tagged and mode-validated before load (`omega-save`).
6. Frontends select mode before session start and use mode-specific save slots.

## Module Map
1. `crates/omega-core/src/core/mode/contracts.rs`
2. `crates/omega-core/src/core/mode/classic.rs`
3. `crates/omega-core/src/core/mode/modern.rs`
4. `crates/omega-core/src/core/mode/mod.rs`

## Save/Load Rules
1. Saves include metadata mode (`classic` or `modern`).
2. Legacy/mode-less saves default to `classic`.
3. Cross-mode load is rejected by policy check in frontend load flows.

## Content Rules
1. `ContentPackId::Classic` loads frozen classic pack (`tools/libsrc`).
2. `ContentPackId::Modern` loads `tools/libsrc-modern` if present; falls back to base pack.
3. Classic pack fingerprint is exported by `classic_content_fingerprint()`.

## Frontend Rules
1. TUI launcher mode selection sets bootstrap mode and slot namespace.
2. Bevy launcher mode selection sets bootstrap mode and slot namespace.
3. Slot naming is mode-scoped (`target/omega-{mode}-slot-1.json`).

## Verification
Required dual-mode verification binaries:

1. `classic_mode_drift_guard`
2. `modern_mode_smoke`
3. `dual_mode_blackbox_suite`
4. `mode_artifact_integrity_guard`

