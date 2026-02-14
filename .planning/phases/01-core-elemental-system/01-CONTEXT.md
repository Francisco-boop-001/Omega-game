# Context: Phase 1 - Core Elemental State & Reaction System

## Vision & Tone
The elemental system follows a "Violent vs. Smoldering" philosophy. Powerful magic (Fireballs) triggers immediate, explosive reactions, while mundane items (Torches) cause gradual changes based on intensity thresholds. The world is physically reactive, interactive, and persistent, with a long-term "Nature Reclaims" cycle.

## Reaction Dynamics
- **Intensity Thresholds:** Materials have "Flash Points" (e.g., Grass < Wood < Stone). High-intensity triggers (Fireball) bypass gradual heat-up.
- **Gas Layering:** The simulation supports gas layers (Steam, Smoke) existing concurrently over liquids (Water, Oil).
- **Explosive Displacement:** Violent reactions physically push Heat, Pressure, and Gasses into neighboring tiles in a single frame.

## Sensory & UI
- **Visual Signals:** High-heat tiles exhibit a "shimmer/haze" effect. Active elements (Fire, Lava) bleed color/glow into adjacent tiles.
- **Feedback:** The log uses descriptive, atmospheric warnings ("The air smells of ozone").
- **Inspection:** The 'Look' command provides "Vibe" descriptions (e.g., "Saturated with moisture," "Scorching") rather than raw percentage numbers.

## Interaction Priority (The "Layered" System)
- **State Multiplicity:** Tiles support concurrent Solid, Liquid, and Gas states (e.g., "Swamp Fire").
- **Dousing Logic:** "Waterlogged" (max Wet) status grants fire immunity until evaporated.
- **Wind Vectors:** Wind acts as a force map that pushes, pulls, or "cuts" (disperses) Gas and Heat values.
- **Earth Anchoring:** Earth is the structural base; it cannot be displaced by Wind/Water, only transformed (Dirt -> Mud, Stone -> Rubble).

## Stability, Decay & Combustion
- **Interactive Debris:** Burning leaves "Ash"; breaking leaves "Rubble." These are interactive (e.g., Wind blows Ash).
- **Combustion Mechanics:** Fire expansion is governed by fuel density. High combustible concentrations trigger pressure-based explosions.
- **Persistence:** Heat is "Residual" and decays slowly. The environment features a "Nature Reclaims" mechanic where scorched earth or debris eventually recovers/recycles into natural states.

## Deferred Ideas
- **Illumination System:** Light and Darkness interactions (e.g., Fire illuminating, Shadows affecting visibility) are deferred to a post-core phase.
- **Electrical Conductivity:** Chain reactions through Water/Metal are deferred to a later "Shock" phase.
