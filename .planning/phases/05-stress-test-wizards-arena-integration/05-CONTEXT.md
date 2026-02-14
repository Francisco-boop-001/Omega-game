# Context: Phase 5 - Stress Test & Wizard's Arena Integration

## Vision & Tone
Phase 5 is the "God Simulator" for Omega. It provides a robust, sandbox environment ('Wizard's Arena') to validate every elemental interaction, physics rule, and visual effect at scale. The interface is informative, developer-friendly, and capable of triggering world-altering catastrophes with a single click.

## Stress Scenarios (The "Catastrophe" Suite)
- **Individual Triggers:** Dedicated buttons for "Great Flood" (Dam Break), "Forest Fire Jump," and "Massive Windstorm."
- **Doomsday Button:** A single button that triggers all elemental disasters simultaneously.
- **Interception Chaos:** Automated "Turret Mode" that fires high-intensity projectiles in random patterns to test mid-air collisions and deflections.
- **The "Fuel Field":** A preset map layout dense with combustible materials to test explosive chain reactions.

## Sandbox UX & "God Mode"
- **Elemental Brush:** Mouse-based "painting" mode to manually inject Fire, Water, or Ash into the CA grid.
- **Action-Driven Simulation:** CA ticks are driven by player actions (movement/interaction), ensuring the simulation respects the turn-based nature of the game.
- **Monster Spawner Presets:** A dropdown menu to spawn existing game monsters (Rats, Goblins, Ogres) to test drowning logic and "plop" animations.
- **Snapshot & Reset:** Ability to capture the arena state before a catastrophe and reset to that "Pre-Disaster" state.

## Performance Diagnostics (Newbie-Friendly HUD)
- **Traffic Light System:** Visual indicator (Green/Yellow/Red) for simulation health relative to target Hz.
- **Live Counters:** Real-time display of active Projectile and Particle counts.
- **Action-Based Logs:** Informative debug logs that detail what triggered a reaction (e.g., "Fireball hit Water -> Generated 15 Steam cells").
- **Collapsible Panel:** A clean, out-of-the-way UI that expands for deep-dive diagnostics.

## Stability & Safety (The "Lag Kill-Switch")
- **Emergency Extinguish:** Automatic removal of all Gas/Liquid layers if FPS drops below a critical threshold (e.g., 20 FPS).
- **Hard Particle Cap:** Strict limit on the total number of visual particles, despawning oldest ones to preserve performance.
- **Snapshot Recovery:** Manual and automatic reset points to prevent being stuck in a permanently "broken" or lagging arena.

## Implementation Priorities
- **Monster Registry Integration:** Ensure the spawner dropdown pulls dynamically from the game's monster database.
- **Gizmo Persistence:** Performance HUD utilizes Bevy's gizmo and egui systems for clear visualization.
