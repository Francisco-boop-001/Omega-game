# Context: Phase 3 - Visual FX System

## Vision & Tone
The visual system is "High-Fidelity ASCII." Effects are not static glyphs but dynamic, smooth-moving, physical entities that undergo visual cascades. An explosion isn't just an AOE; it's a white-hot wave that cools into colorful embers, leaving behind a physically altered environment.

## Elemental Visual Language
- **Fire (The Cascade):** Impacts start white-hot (@), expanding in yellow/orange, cooling into red/dim (*) and finally gray (.) embers. 
- **Water/Ice:** Smooth blue projectiles (arrows/bolts) that splash into droplets (,) or vaporize into steam (~). Droplets can re-freeze into ice if they drift into cold zones.
- **Wind:** Represented by short-lived gust glyphs (~) in a cone/line, which then vanish, leaving only the drift of other particles.
- **Earth/Debris:** Heavy glyphs (#, &) that rotate/spin through the air and bounce off walls.
- **Toxic/Acid:** Smoldering green splashes that leave toxic fume clouds.

## Particle Kinematics & Physics
- **Smooth Motion:** All projectiles and particles move smoothly across pixels, not tile-by-tile.
- **Environmental Reaction:** Particles react to the WindGrid (drift) and have their own logical Weight (e.g., Ash falls faster than Smoke).
- **Physical Interaction:** Particles bounce off structural tiles (walls) rather than just vanishing.
- **Z-Axis Behavior:** Gasses (Smoke/Steam) have a natural upward (Z) velocity.

## Visual Lifecycle (Ageing)
- **Morphing & Shrinking:** Particles change glyphs as they "cool" or age. They physically shrink in size as they approach the end of their lifetime.
- **Heat Mapping:** Particles at the epicenter of an explosion live longer and maintain higher intensity colors.
- **Settling:** Debris (Ash/Rubble) particles eventually "settle" and become static layers on the CA grid.

## Targeting UI (Arcane Cartographer)
- **Temporal Ghost:** Aiming shows a smooth pulsing path, but the final impact highlight is **Snapped** to the tile grid for tactical precision.
- **Interactive Ribbon:** Projectiles leave a "smoke ribbon" trail—a dense sequence of particles that dissipate over time.

## Deferred Ideas
- **Corpse Morphing:** Visually changing NPC glyphs to "Burnt" versions when they die in fire is noted for a future "Character Polish" phase.
