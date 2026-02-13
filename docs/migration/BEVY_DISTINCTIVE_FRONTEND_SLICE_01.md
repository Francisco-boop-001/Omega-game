# Bevy Distinctive Frontend Slice 01

## Scope
1. Add a real windowed Bevy visual client (`omega-bevy-visual`) on top of the existing deterministic runtime adapter.
2. Keep mechanics in `omega-core`; presentation is read-only and mode-aware.
3. Keep Classic frozen; Modern adds richer visual guidance.

## Design Thesis
1. Arcane Cartographer Console.
2. Signature element: objective compass ring around player and animated route pips toward objective markers.
3. UI shape: asymmetric map-first left panel + right command deck.

## Implemented Surfaces
1. `crates/omega-bevy/src/presentation/*` modular presentation layers:
   - `theme`, `scene`, `tilemap`, `overlays`, `hud`, `interaction`, `timeline`, `input`, `animation`
2. `crates/omega-bevy/src/bin/omega-bevy-visual.rs` windowed frontend entrypoint.
3. Non-breaking projection helpers in `omega-bevy`:
   - `UiEventSeverity`
   - `InteractionFocusState`
   - `MapFxFrame`
4. New strict tooling checks:
   - `modern_bevy_visual_smoke`
   - `bevy_visual_blackbox_suite`
   - `classic_visual_drift_guard`
5. Gate wiring:
   - `live_checks_all` includes the three new checks.
   - `true_parity_refresh` requires new artifacts/components.

## Artifact Paths
1. `target/modern/bevy-visual-smoke.json`
2. `target/modern/bevy-visual-smoke.md`
3. `target/dual/bevy-visual-blackbox.json`
4. `target/dual/bevy-visual-blackbox.md`
5. `target/classic/classic-visual-drift-guard.json`
6. `target/classic/classic-visual-drift-guard.md`

## Validation Commands
1. `cargo check -p omega-bevy`
2. `cargo run -p omega-tools --bin modern_bevy_visual_smoke`
3. `cargo run -p omega-tools --bin bevy_visual_blackbox_suite`
4. `cargo run -p omega-tools --bin classic_visual_drift_guard`
5. `cargo run -p omega-tools --bin true_parity_refresh`

## Known Limits
1. Slice 01 renders map and command deck text-first; sprite-atlas richness and post-processing are deferred.
2. Bevy CLI harness (`omega-bevy-app`) remains for deterministic parity/debug workflows.
