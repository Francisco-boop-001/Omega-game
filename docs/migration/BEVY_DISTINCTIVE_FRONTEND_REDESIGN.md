# Bevy Distinctive Frontend Redesign

## Thesis
Occult Navigation Instrument:
- Modern mode keeps ASCII glyph gameplay at the center.
- Presentation layers add atmosphere, hierarchy, and action clarity.
- Classic behavior remains frozen and untouched mechanically.

## Signature Elements
1. Living objective halo around the player in Modern mode.
2. Instrument-panel composition:
- Survey Grid
- Objective Halo
- Status Deck
- Interaction Focus
- Outcome Timeline
3. Dynamic panel focus tinting based on active runtime context.

## Scope
This slice is presentation-only:
- no combat, progression, quest, economy, or command logic changes
- no save schema changes
- no Classic parity drift allowed

## Implementation Notes
1. Added readability resource:
- `UiReadabilityConfig { scale, high_contrast, reduced_motion }`
2. Added focus resource:
- `UiFocusState { active_panel, urgency }`
3. Added panel card components for style routing:
- map, compass, status, interaction, timeline
4. Added tokenized theme expansion for:
- contrast tiers
- panel depth colors
- objective halo accents
- typography scale
5. Added focus-style renderer:
- styles are derived from read-only `RenderFrame` and motion state
- interaction focus always outranks timeline when active
6. Updated map compositor:
- objective marker glyph emphasis
- pulse-aware route pips
- objective halo responds to interaction/objective distance

## Verification Commands
```powershell
cargo test -p omega-bevy
cargo run -p omega-tools --bin modern_bevy_visual_smoke
cargo run -p omega-tools --bin classic_visual_drift_guard
cargo run -p omega-tools --bin true_parity_refresh
cargo test --workspace
```

## Evidence Policy
A change is accepted only when all checks pass in the same revision and
Classic drift guard remains green.
