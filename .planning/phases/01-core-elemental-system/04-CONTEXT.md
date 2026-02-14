# Context: Phase 4 - Environmental Interaction

## Vision & Tone
The environment is no longer a static backdrop; it is a live, reactive participant in gameplay. Structures can be sieged, floors can be flooded, and the air itself can become a weapon. The "Siege Mage" experience is defined by manipulating these systems—melting stone walls, flooding corridors, and trapping enemies in corrosive gas.

## Fluid & Fire Dynamics
- **The Breach Pulse:** Dramatic events (e.g., dam breaking) inject massive Liquid/Pressure values in a single frame to trigger immediate flooding.
- **Physical Floods:** Flash floods (high Pressure liquids) can physically displace items on the ground based on their Weight stat.
- **Viscosity-Driven Flow:** Spread speed is a function of depth (Pressure). High-pressure liquids move faster horizontally.
- **Fuel-Based Expansion:** Fire spread is realistic; high-density fuel (oil fields) triggers instant "jumps" to adjacent tiles.

## Structural Integrity & Melting
- **Hardness Tiers:** Structures have tiers. "Palace/Temple" walls are impervious to standard heat/pressure, requiring "Siege Grade" magic to affect.
- **Phase Transformation:** Stone melts into viscous, high-heat Lava. 
- **Waterlogging:** Wooden structures can become "Waterlogged," granting temporary fire immunity.
- **Rubble Persistence:** Collapsed structures leave "Rubble" layers that block movement until eroded by "Nature Reclaims."

## Submersion & Hazard Navigation
- **Dangerous Air Signals:** Poor air (Smoke/Steam) is signaled by desaturating/dimming tile glyphs to indicate low visibility.
- **The Drowning Logic:** Intelligence allows "treading water" (delaying damage); Weight determines damage rate once sinking.
- **"Plop" Animation:** Drowning entities undergo a visual cascade (O -> o -> . -> splash).
- **Gas Afflictions:** Different gases have specific effects: Blinding, Corrosive (Health damage), or Choking (Stamina/Mana drain).

## Gas Pooling & Ventilation
- **Ventilation Sinks:** Open doors and windows act as gas sinks, pulling pressure toward exits.
- **Common Door Leakage:** Standard doors have a "Leaking Factor," allowing small amounts of gas to seep through.
- **Corrosive Deterioration:** Acidic gas not only damages entities but deteriorates structures, potentially leading to collapses if not ventilated.
- **Volatility:** Corrosive gasses can explode or ignite if they touch extreme heat sources.

## Implementation Priorities
- **System Sequencing:** Gas/Liquid flow logic is integrated into the FixedUpdate CA pass from Phase 1.
- **Magic Density:** The simulation architecture supports a future "Magic Density" overlay to differentiate magical lava from natural lava.
