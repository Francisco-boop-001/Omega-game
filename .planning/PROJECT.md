# Omega Elemental Systems

## Core Value
Implement a high-fidelity elemental interaction system using Cellular Automata for environmental effects and ECS for projectile management, enabling emergent gameplay in a Bevy-based roguelike.

## Success Criteria
- Performant 2D Cellular Automata grid handling heat, liquid, and gas.
- ECS-based projectile system with varied trajectories (arcs, beams).
- Reactive environment where fire spreads, water pools, and steam rises.
- Visual immersion via glyph-based particle systems.

## Constraints
- **Engine:** Bevy 0.15+
- **Rendering:** TUI/Glyph-based (Modernized Bevy Front-end)
- **Performance:** Must maintain 60 FPS on target hardware during high-density elemental reactions.
- **Architecture:** ECS-first design, simulation decoupled from rendering.
