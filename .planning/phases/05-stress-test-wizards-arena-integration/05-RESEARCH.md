# Phase 5: Stress Test & Wizard's Arena Integration - Research

**Researched:** 2026-02-14
**Domain:** Performance testing, stress testing, sandbox environments, diagnostic UIs
**Confidence:** MEDIUM-HIGH

## Summary

Phase 5 validates the elemental simulation system under extreme conditions through a "Wizard's Arena" sandbox environment. This TUI-based test scene provides developer-friendly controls to spawn entities, trigger catastrophic elemental events, monitor performance metrics, and ensure the system maintains 60 FPS with 100+ projectiles and a 128x128 CA grid updating within <2ms.

The research reveals that while Omega already has robust testing infrastructure (omega-tools with smoke tests), Bevy's diagnostic ecosystem, and a working Wizard's Arena foundation, this phase requires integrating performance monitoring, implementing "God Mode" controls for TUI, creating catastrophe scenarios, and establishing safety mechanisms for performance degradation.

**Primary recommendation:** Leverage existing Bevy diagnostics (FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin) with custom CA grid timing diagnostics, build TUI control panels using Ratatui widgets for spawning/triggers, implement snapshot/restore via serde serialization of GameState, and create "traffic light" performance indicators that trigger automatic cleanup at critical thresholds.

## User Constraints (from CONTEXT.md)

### Vision & Tone
Phase 5 is the "God Simulator" for Omega - a robust sandbox environment providing informative, developer-friendly controls capable of triggering world-altering catastrophes with a single click.

### Locked Features (MUST implement)

**Stress Scenarios ("Catastrophe Suite"):**
- Individual triggers: "Great Flood" (Dam Break), "Forest Fire Jump", "Massive Windstorm"
- "Doomsday Button": Simultaneous triggering of all disasters
- "Interception Chaos": Automated turret mode firing random high-intensity projectiles
- "Fuel Field": Preset map layout dense with combustible materials

**Sandbox UX ("God Mode"):**
- Elemental Brush: Mouse-based painting to inject Fire, Water, or Ash into CA grid
- Action-Driven Simulation: CA ticks driven by player actions (respecting turn-based nature)
- Monster Spawner Presets: Dropdown menu for Rats, Goblins, Ogres
- Snapshot & Reset: Capture arena state before catastrophe, reset to "Pre-Disaster" state

**Performance Diagnostics (Newbie-Friendly HUD):**
- Traffic Light System: Green/Yellow/Red visual indicator for simulation health vs target Hz
- Live Counters: Real-time display of active Projectile and Particle counts
- Action-Based Logs: Debug logs detailing what triggered reactions (e.g., "Fireball hit Water -> Generated 15 Steam cells")
- Collapsible Panel: Out-of-the-way UI that expands for deep-dive diagnostics

**Stability & Safety ("Lag Kill-Switch"):**
- Emergency Extinguish: Automatic removal of all Gas/Liquid layers if FPS < 20
- Hard Particle Cap: Strict limit on total particles, despawning oldest to preserve performance
- Snapshot Recovery: Manual and automatic reset points to prevent permanently broken arena

**Implementation Priorities:**
- Monster Registry Integration: Spawner dropdown pulls dynamically from game's monster database
- Gizmo Persistence: Performance HUD utilizes Bevy's gizmo and egui systems

### Critical Constraint: TUI-Based Architecture
**IMPORTANT:** This is a TUI-based game using Ratatui for terminal rendering, NOT a graphical 2D/3D game. The "Wizard's Arena" is a terminal UI scene. Mouse input may be limited. Consider terminal-appropriate UI patterns.

This means:
- "Mouse-based painting" must work within terminal mouse event capabilities (Crossterm mouse events)
- Dropdowns are terminal UI widgets (Ratatui select/list widgets)
- "Traffic light" indicators are colored text/glyphs (using terminal colors)
- No actual gizmos/sprites - "gizmo" likely means visual debugging overlays in terminal
- Performance HUD renders as text panels in terminal layout

## Standard Stack

### Core Dependencies (Already Integrated)

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| bevy | 0.15 | ECS and simulation scheduling | Official version used throughout project |
| bevy_egui | 0.31 | Developer UI overlays (Bevy-visual only) | Standard for in-engine debug UIs |
| ratatui | 0.29 | Terminal UI rendering | Modern TUI framework, actively maintained |
| crossterm | 0.28 | Terminal input/output control | De facto standard for Ratatui backends |
| serde | 1.x | Serialization for snapshots | Rust ecosystem standard |
| serde_json | 1.x | JSON snapshot format | Human-readable, debuggable |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| omega-core | local | Simulation primitives (CaGrid, GameState) | All arena logic |
| omega-bevy | local | Bevy integration and simulation plugin | Performance monitoring systems |
| omega-tui | local | TUI rendering and input mapping | Arena TUI scene |
| omega-content | local | Monster catalog, bootstrap_wizard_arena | Entity spawning |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Built-in diagnostics | Custom timing macros | More control but duplicates Bevy's proven system |
| Serde JSON | Binary formats (bincode, postcard) | Faster but loses debuggability |
| Ratatui widgets | Custom TUI rendering | More flexibility but reinvents wheel |
| Crossterm mouse | Terminal-kit or similar | Minimal ecosystem benefit |

**Installation:**
All dependencies already in Cargo.toml workspace. No new external crates required.

## Architecture Patterns

### Recommended Project Structure
```
crates/
├── omega-bevy/
│   ├── src/
│   │   ├── simulation/
│   │   │   ├── diagnostics.rs      # Custom CA timing diagnostics
│   │   │   └── catastrophe.rs      # Pre-configured disaster scenarios
│   │   └── presentation/
│   │       ├── spawner.rs          # Already exists - arena spawner UI
│   │       └── inspector.rs        # Already exists - entity inspector
├── omega-tui/
│   └── src/
│       ├── arena_scene.rs          # Wizard's Arena TUI layout
│       ├── perf_hud.rs             # Performance overlay widgets
│       └── elemental_brush.rs      # Mouse painting for CA injection
└── omega-tools/
    └── src/bin/
        └── arena_stress_test.rs    # Automated stress test binary
```

### Pattern 1: Bevy Diagnostics Integration

**What:** Register custom diagnostics for CA grid update timing alongside built-in frame/entity metrics

**When to use:** For performance monitoring that integrates with Bevy's diagnostic ecosystem

**Example:**
```rust
// Source: Bevy docs + project context
use bevy::diagnostic::{Diagnostic, DiagnosticPath, Diagnostics, RegisterDiagnostic};
use bevy::prelude::*;

const CA_UPDATE_TIME: DiagnosticPath = DiagnosticPath::const_new("ca/update_ms");

pub struct CaDiagnosticsPlugin;

impl Plugin for CaDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(CA_UPDATE_TIME));
        app.add_systems(FixedUpdate, measure_ca_update_time);
    }
}

fn measure_ca_update_time(
    mut diagnostics: ResMut<Diagnostics>,
    time: Res<Time>,
) {
    let start = std::time::Instant::now();
    // CA update happens in environmental_behaviors system
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    diagnostics.add_measurement(&CA_UPDATE_TIME, || elapsed);
}
```

### Pattern 2: TUI Performance HUD with Ratatui

**What:** Terminal-based performance overlay using Ratatui layout and widgets

**When to use:** For rendering diagnostics in the TUI version (omega-tui)

**Example:**
```rust
// Derived from Ratatui patterns + project TUI architecture
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};

pub fn render_perf_hud(frame: &mut Frame, diagnostics: &PerfSnapshot, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Traffic light
            Constraint::Length(3), // Counters
            Constraint::Min(0),    // Logs (expandable)
        ])
        .split(area);

    // Traffic Light System
    let health_color = match diagnostics.fps {
        fps if fps >= 58 => Color::Green,
        fps if fps >= 45 => Color::Yellow,
        _ => Color::Red,
    };
    let traffic_light = Paragraph::new(format!("● {} FPS", diagnostics.fps))
        .style(Style::default().fg(health_color));
    frame.render_widget(traffic_light, chunks[0]);

    // Live Counters
    let counters = Paragraph::new(format!(
        "Projectiles: {} | Particles: {} | CA Update: {:.2}ms",
        diagnostics.projectile_count,
        diagnostics.particle_count,
        diagnostics.ca_update_ms
    ))
    .block(Block::default().borders(Borders::ALL).title("Arena Stats"));
    frame.render_widget(counters, chunks[1]);

    // Action-Based Logs (collapsible)
    if diagnostics.show_logs {
        let logs = Paragraph::new(diagnostics.event_log.join("\n"))
            .block(Block::default().borders(Borders::ALL).title("Event Log"));
        frame.render_widget(logs, chunks[2]);
    }
}
```

### Pattern 3: Snapshot/Restore via Serde

**What:** Serialize GameState before catastrophe, restore on reset

**When to use:** For implementing "Pre-Disaster" snapshot recovery

**Example:**
```rust
// Source: omega-save patterns + Serde documentation
use omega_core::GameState;
use serde_json;

#[derive(Clone)]
pub struct ArenaSnapshot {
    state: GameState,
    timestamp: u64,
    label: String,
}

impl ArenaSnapshot {
    pub fn capture(state: &GameState, label: String) -> Self {
        Self {
            state: state.clone(),
            timestamp: state.clock.turn,
            label,
        }
    }

    pub fn restore(&self) -> GameState {
        self.state.clone()
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

// Usage in arena system
fn handle_snapshot_key(
    mut snapshots: ResMut<SnapshotManager>,
    session: Res<GameSession>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::F5) {
        let snapshot = ArenaSnapshot::capture(&session.state, "Pre-Doomsday".to_string());
        snapshots.push(snapshot);
    }
    if keys.just_pressed(KeyCode::F9) {
        if let Some(snapshot) = snapshots.pop() {
            // Restore logic handled by session reset
        }
    }
}
```

### Pattern 4: Emergency Performance Cleanup

**What:** Automatic system that triggers cleanup when FPS drops below threshold

**When to use:** For the "Lag Kill-Switch" safety mechanism

**Example:**
```rust
// Derived from Bevy diagnostics patterns + game engine safety patterns
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const CRITICAL_FPS_THRESHOLD: f64 = 20.0;

fn emergency_cleanup_system(
    mut commands: Commands,
    diagnostics: Res<DiagnosticsStore>,
    mut ca_grid: ResMut<CaGrid>,
    particles: Query<Entity, With<Particle>>,
    projectiles: Query<Entity, With<Projectile>>,
) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps_diagnostic.smoothed() {
            if fps < CRITICAL_FPS_THRESHOLD {
                // Emergency Extinguish: Clear all gas/liquid layers
                for y in 0..ca_grid.height() {
                    for x in 0..ca_grid.width() {
                        let mut cell = ca_grid.get_mut(x, y);
                        cell.wet = 0;
                        cell.pressure = 0;
                    }
                }

                // Hard Particle Cap: Despawn all particles
                for entity in particles.iter() {
                    commands.entity(entity).despawn();
                }

                warn!("EMERGENCY CLEANUP: FPS dropped to {:.1}, cleared simulation", fps);
            }
        }
    }
}
```

### Pattern 5: Catastrophe Scenario Builders

**What:** Pre-configured functions that inject massive quantities of elemental effects

**When to use:** For implementing "Great Flood", "Forest Fire Jump", "Massive Windstorm" buttons

**Example:**
```rust
// Derived from project CA architecture + stress testing patterns
use omega_core::simulation::grid::CaGrid;
use omega_core::simulation::wind::WindGrid;

pub struct CatastropheScenarios;

impl CatastropheScenarios {
    /// "Great Flood" - Dam Break
    pub fn great_flood(ca_grid: &mut CaGrid, center: (usize, usize)) {
        let (cx, cy) = center;
        let radius = 15;
        for dy in 0..radius {
            for dx in 0..radius {
                let x = cx.saturating_sub(radius/2) + dx;
                let y = cy.saturating_sub(radius/2) + dy;
                if x < ca_grid.width() && y < ca_grid.height() {
                    let mut cell = ca_grid.get_mut(x, y);
                    cell.wet = 100; // Maximum saturation
                }
            }
        }
    }

    /// "Forest Fire Jump" - Rapid fire spread
    pub fn forest_fire_jump(ca_grid: &mut CaGrid, origin: (usize, usize)) {
        let (ox, oy) = origin;
        let radius = 20;
        for dy in 0..radius {
            for dx in 0..radius {
                let x = ox.saturating_sub(radius/2) + dx;
                let y = oy.saturating_sub(radius/2) + dy;
                if x < ca_grid.width() && y < ca_grid.height() {
                    let mut cell = ca_grid.get_mut(x, y);
                    cell.heat = 200; // Extreme heat for ignition
                }
            }
        }
    }

    /// "Massive Windstorm"
    pub fn massive_windstorm(wind_grid: &mut WindGrid) {
        for y in 0..wind_grid.height() {
            for x in 0..wind_grid.width() {
                let (fx, fy) = wind_grid.get_force(x, y);
                wind_grid.set_force(x, y, fx + 10.0, fy + 5.0); // Hurricane-force winds
            }
        }
    }

    /// "Doomsday Button" - All disasters simultaneously
    pub fn doomsday(ca_grid: &mut CaGrid, wind_grid: &mut WindGrid) {
        Self::great_flood(ca_grid, (ca_grid.width()/2, ca_grid.height()/2));
        Self::forest_fire_jump(ca_grid, (ca_grid.width()/4, ca_grid.height()/4));
        Self::massive_windstorm(wind_grid);
    }
}
```

### Pattern 6: Turret Mode for Interception Testing

**What:** Automated system that spawns projectiles in random patterns

**When to use:** For "Interception Chaos" stress testing

**Example:**
```rust
// Derived from existing projectile spawning + automation patterns
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct TurretMode {
    pub active: bool,
    pub fire_rate_hz: f32,
    pub accumulator: f32,
}

fn turret_mode_system(
    mut turret: ResMut<TurretMode>,
    mut commands: Commands,
    time: Res<Time>,
    ca_grid: Res<CaGrid>,
) {
    if !turret.active { return; }

    turret.accumulator += time.delta_secs();
    let interval = 1.0 / turret.fire_rate_hz;

    while turret.accumulator >= interval {
        turret.accumulator -= interval;

        let mut rng = rand::thread_rng();
        let origin_x = rng.gen_range(0..ca_grid.width());
        let origin_y = rng.gen_range(0..ca_grid.height());
        let target_x = rng.gen_range(0..ca_grid.width());
        let target_y = rng.gen_range(0..ca_grid.height());

        // Spawn random projectile type
        let projectile_kind = match rng.gen_range(0..3) {
            0 => ProjectileKind::Fireball,
            1 => ProjectileKind::WaterBlast,
            _ => ProjectileKind::MagicMissile,
        };

        // Commands to spawn projectile entity (using existing spawning logic)
        // ...
    }
}
```

### Anti-Patterns to Avoid

- **Blocking diagnostics**: Don't use synchronous IO in diagnostic systems - use async or fire-and-forget logging
- **Over-aggressive cleanup**: Emergency extinguish should be last resort, not triggered at first FPS dip
- **Snapshot spam**: Don't auto-snapshot every frame - only on user request or pre-catastrophe
- **TUI complexity**: Keep terminal UI simple - avoid deeply nested widgets that slow rendering
- **Hard-coded thresholds**: Make FPS thresholds, particle caps configurable via Resource constants

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| FPS measurement | Manual delta time averaging | FrameTimeDiagnosticsPlugin | Proven smoothing, standardized API |
| Entity counting | Query iteration in app code | EntityCountDiagnosticsPlugin | Optimized, built-in |
| Terminal mouse events | Raw ANSI parsing | Crossterm event::MouseEvent | Handles terminal quirks |
| JSON serialization | Manual string building | serde_json | Type-safe, handles edge cases |
| TUI layouts | Manual cursor positioning | Ratatui Layout system | Responsive, constraint-based |
| CA grid timing | std::time in app code | Bevy's Time resource + custom Diagnostic | Integrates with diagnostic ecosystem |

**Key insight:** Bevy's diagnostic system is battle-tested for exactly this use case. Custom CA timing can piggyback on it rather than duplicating frame timing logic. Ratatui handles terminal complexity better than manual ANSI codes.

## Common Pitfalls

### Pitfall 1: TUI Mouse Limitations

**What goes wrong:** Assuming terminal mouse works like GUI - smooth tracking, pixel precision, hover states

**Why it happens:** Crossterm mouse events are cell-based (character grid), not pixel-based. Not all terminals support mouse at all.

**How to avoid:**
- Use `crossterm::event::EnableMouseCapture` at startup
- Handle `MouseEvent::Down` for clicks, not `MouseEvent::Moved` for painting (too noisy)
- Provide keyboard alternatives for all mouse actions
- Test in common terminals (Windows Terminal, iTerm2, GNOME Terminal)

**Warning signs:** Users report "mouse doesn't work" or "painting too slow"

### Pitfall 2: Diagnostic Overhead in FixedUpdate

**What goes wrong:** Adding diagnostics to every FixedUpdate tick slows the simulation, defeating the purpose

**Why it happens:** FixedUpdate runs at 64Hz - adding diagnostics every tick creates overhead

**How to avoid:**
- Sample diagnostics every N ticks (e.g., every 10th tick)
- Use Bevy's diagnostic smoothing, don't re-implement averaging
- Don't allocate strings in hot path - use `format_args!` or pre-allocated buffers

**Warning signs:** CA update time increases when diagnostics enabled

### Pitfall 3: Snapshot Memory Explosion

**What goes wrong:** GameState clones consume hundreds of MB, causing memory pressure

**Why it happens:** GameState contains large Vecs (site_grid, monsters, ground_items) that clone deeply

**How to avoid:**
- Limit snapshot history (e.g., max 5 snapshots)
- Implement cleanup of oldest snapshots
- Consider delta-based snapshots (only serialize changes)
- Use reference counting (Rc/Arc) for large immutable data

**Warning signs:** Arena becomes sluggish after multiple snapshots

### Pitfall 4: Emergency Cleanup Thrashing

**What goes wrong:** FPS dips momentarily, triggers cleanup, FPS recovers, triggers cleanup again in loop

**Why it happens:** No hysteresis - single threshold triggers both cleanup and recovery

**How to avoid:**
- Use two thresholds: cleanup at <20 FPS, re-enable at >30 FPS
- Add cooldown timer after cleanup (e.g., 5 seconds)
- Log cleanup events to help debug what caused FPS drop

**Warning signs:** Constant cleanup logs, unstable FPS readings

### Pitfall 5: CA Grid Update Measurement Inaccuracy

**What goes wrong:** Measuring CA update time shows <1ms but actual frame budget higher

**Why it happens:** Measuring only one system, not accounting for particle physics, collision, etc.

**How to avoid:**
- Measure entire FixedUpdate schedule, not just CA systems
- Use Bevy's built-in system profiling (cargo feature `trace`)
- Account for Bevy's parallelism - systems may overlap

**Warning signs:** Low CA time but high total frame time

### Pitfall 6: Ratatui Render Overhead

**What goes wrong:** Performance HUD itself causes frame drops due to complex widgets

**Why it happens:** Nested layouts, many text allocations per frame, inefficient widget usage

**How to avoid:**
- Cache formatted strings between updates
- Use `Paragraph::new(vec![])` with pre-built spans
- Simplify layout when FPS drops (hide logs panel)
- Profile TUI rendering separately (Ratatui can be slow)

**Warning signs:** Disabling HUD improves FPS noticeably

## Code Examples

### Example 1: Arena Scene Setup (TUI)

```rust
// Wizard's Arena TUI scene initialization
// Source: omega-tui patterns + Ratatui examples

use omega_core::GameState;
use omega_content::bootstrap_wizard_arena;
use ratatui::prelude::*;

pub struct ArenaScene {
    pub state: GameState,
    pub perf_hud_visible: bool,
    pub spawner_visible: bool,
    pub snapshots: Vec<ArenaSnapshot>,
    pub turret_mode: bool,
}

impl ArenaScene {
    pub fn new() -> anyhow::Result<Self> {
        let (state, _) = bootstrap_wizard_arena()?;
        Ok(Self {
            state,
            perf_hud_visible: true,
            spawner_visible: true,
            snapshots: Vec::new(),
            turret_mode: false,
        })
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Map viewport
                Constraint::Percentage(30), // Controls panel
            ])
            .split(frame.area());

        // Render map in left panel
        self.render_map(frame, chunks[0]);

        // Right panel: stacked controls
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10), // Spawner
                Constraint::Length(8),  // Catastrophe buttons
                Constraint::Min(0),     // Performance HUD
            ])
            .split(chunks[1]);

        if self.spawner_visible {
            self.render_spawner(frame, right_chunks[0]);
        }
        self.render_catastrophe_controls(frame, right_chunks[1]);
        if self.perf_hud_visible {
            self.render_perf_hud(frame, right_chunks[2]);
        }
    }
}
```

### Example 2: Elemental Brush (TUI Mouse Painting)

```rust
// Terminal-based "painting" to inject elements into CA grid
// Source: Crossterm mouse handling + CA grid patterns

use crossterm::event::{MouseEvent, MouseEventKind};
use omega_core::simulation::grid::CaGrid;

pub enum BrushMode {
    Fire,
    Water,
    Ash,
}

pub fn handle_brush_painting(
    event: MouseEvent,
    ca_grid: &mut CaGrid,
    brush_mode: BrushMode,
    map_area: Rect,
) {
    if let MouseEventKind::Down(_) | MouseEventKind::Drag(_) = event.kind {
        // Convert terminal cell coordinates to CA grid coordinates
        let grid_x = event.column.saturating_sub(map_area.x) as usize;
        let grid_y = event.row.saturating_sub(map_area.y) as usize;

        if grid_x < ca_grid.width() && grid_y < ca_grid.height() {
            let mut cell = ca_grid.get_mut(grid_x, grid_y);
            match brush_mode {
                BrushMode::Fire => {
                    cell.heat = 150; // Ignition temperature
                }
                BrushMode::Water => {
                    cell.wet = 80; // High saturation
                }
                BrushMode::Ash => {
                    cell.heat = 0;
                    cell.wet = 0;
                    cell.pressure = 20; // Ash as low-pressure gas
                }
            }
        }
    }
}
```

### Example 3: Traffic Light Performance Indicator

```rust
// Simple colored health indicator for TUI
// Source: Ratatui styling + diagnostic patterns

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub fn traffic_light_widget(fps: f64, target_hz: f64) -> Paragraph<'static> {
    let percentage = (fps / target_hz) * 100.0;
    let (symbol, color, label) = match percentage {
        p if p >= 95.0 => ("●", Color::Green, "HEALTHY"),
        p if p >= 75.0 => ("●", Color::Yellow, "STRESSED"),
        _ => ("●", Color::Red, "CRITICAL"),
    };

    Paragraph::new(format!("{} {} ({:.1} FPS / {:.0} Hz)", symbol, label, fps, target_hz))
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
}
```

### Example 4: Monster Spawner Dropdown (TUI)

```rust
// Terminal-based entity spawner with catalog
// Source: omega-content patterns + Ratatui List widget

use ratatui::widgets::{List, ListItem, Block, Borders};
use omega_core::{GameState, Stats};

pub struct MonsterSpawner {
    pub catalog: Vec<String>,
    pub selected: usize,
}

impl MonsterSpawner {
    pub fn new() -> Self {
        Self {
            catalog: vec![
                "rat".to_string(),
                "goblin".to_string(),
                "ogre".to_string(),
                "wolf".to_string(),
            ],
            selected: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.catalog.iter()
            .enumerate()
            .map(|(i, name)| {
                let marker = if i == self.selected { ">" } else { " " };
                ListItem::new(format!("{} {}", marker, name))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Spawn Monster"));
        frame.render_widget(list, area);
    }

    pub fn spawn_selected(&self, state: &mut GameState, pos: omega_core::Position) {
        let name = &self.catalog[self.selected];
        state.spawn_monster(
            name,
            pos,
            Stats { hp: 10, max_hp: 10, attack_min: 1, attack_max: 3, defense: 0, weight: 50 }
        );
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual FPS counting | FrameTimeDiagnosticsPlugin | Bevy 0.5+ | Standardized, smoothed metrics |
| TUI library (tui-rs) | Ratatui | Fork in 2023 | Active maintenance, new features |
| Blocking diagnostics output | Non-blocking event channels | Bevy 0.13+ | No frame hitches |
| Global state mutation | Bevy Resources + Change Detection | Bevy core | Safe concurrent access |
| Hardcoded thresholds | Configurable Resource constants | Best practice | Runtime tuning |

**Deprecated/outdated:**
- tui-rs: Unmaintained, use Ratatui instead
- Manual Bevy schedule ordering: Use `.chain()` and `.before()`/`.after()` explicitly
- Direct Time::delta access: Prefer specialized Time<Fixed> for FixedUpdate

## Open Questions

### Question 1: CA Grid Timing Granularity

**What we know:** Bevy FixedUpdate runs at 64Hz (15.6ms), target is <2ms for CA update

**What's unclear:** Whether to measure just `environmental_behaviors` system or entire FixedUpdate schedule

**Recommendation:** Measure both separately - `environmental_behaviors` for CA-specific, total FixedUpdate for frame budget. This reveals if bottleneck is CA or other systems (particle physics, projectile collision).

### Question 2: TUI Mouse Painting Performance

**What we know:** Crossterm provides mouse events, but terminal rendering is slow

**What's unclear:** Whether painting at high frequency (e.g., drag) will cause unacceptable lag

**Recommendation:** Implement rate-limiting on brush events (max 10 paints/second). Batch CA grid writes and only flush on mouse up. Profile early.

### Question 3: Snapshot Serialization Format

**What we know:** Serde JSON is human-readable, already used for save slots

**What's unclear:** Whether JSON size/speed is acceptable for arena snapshots

**Recommendation:** Start with JSON for debuggability. If snapshots exceed 10MB or take >100ms, switch to MessagePack or Postcard (serde-compatible, much faster).

### Question 4: Emergency Cleanup Scope

**What we know:** Need to remove Gas/Liquid layers and cap particles

**What's unclear:** Should cleanup also despawn projectiles? Clear wind grid?

**Recommendation:** Three-tier cleanup: (1) Level 1 (FPS <30): Cap particles. (2) Level 2 (FPS <20): Clear gas/liquid. (3) Level 3 (FPS <15): Despawn all projectiles. Log which tier triggered.

### Question 5: Bevy vs TUI Arena Parity

**What we know:** omega-bevy has WizardArena AppState with spawner/inspector, omega-tui doesn't

**What's unclear:** Should TUI have full parity or simplified controls?

**Recommendation:** TUI gets simplified version - spawner list + basic catastrophe buttons. Bevy gets full egui panels. Share underlying arena logic (catastrophe scenarios, snapshots) in omega-core.

## Sources

### Primary (HIGH confidence)

- [Bevy Diagnostics Documentation](https://docs.rs/bevy/latest/bevy/diagnostic/index.html) - Official Bevy diagnostic API
- [Bevy FixedUpdate Schedule](https://docs.rs/bevy/latest/bevy/prelude/struct.FixedUpdate.html) - Official FixedUpdate timing
- [Bevy Fixed Timestep Guide](https://bevy-cheatbook.github.io/fundamentals/fixed-timestep.html) - Unofficial Bevy Cheat Book
- [Ratatui Official Docs](https://docs.rs/ratatui/latest/ratatui/) - TUI rendering API
- [Serde Overview](https://serde.rs/) - Official serialization framework

### Secondary (MEDIUM confidence)

- [Show Framerate - Bevy Cheat Book](https://bevy-cheatbook.github.io/cookbook/print-framerate.html) - FPS monitoring patterns
- [Bevy Profiling Guide](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md) - Official profiling documentation
- [IyesGames/iyes_perf_ui](https://github.com/IyesGames/iyes_perf_ui) - Third-party performance UI overlay
- [Bevymark Stress Test](https://bevy.org/examples/stress-tests/bevymark/) - Official stress test example
- [Using Insta for Rust Snapshot Testing](https://blog.logrocket.com/using-insta-rust-snapshot-testing/) - Snapshot testing patterns
- [Cellular Automata Optimization](https://cell-auto.com/optimisation/) - CA performance patterns
- [Complete Game Optimization Guide 2025](https://generalistprogrammer.com/tutorials/game-optimization-complete-performance-guide-2025) - FPS management techniques
- [Unity Kinematica Snapshot Debugger](https://docs.unity3d.com/Packages/com.unity.kinematica@0.5/manual/Debugger.html) - Snapshot debugging patterns
- [Ratatui.cs Headless Testing](https://github.com/holo-q/Ratatui.cs) - TUI testing patterns

### Tertiary (LOW confidence - project-specific inference)

- omega-bevy spawner.rs - Existing spawner implementation (needs adaptation for TUI)
- omega-content bootstrap_wizard_arena - Existing arena bootstrap (50x50 map)
- omega-tools smoke tests - Existing test infrastructure patterns
- omega-core simulation modules - CA grid, wind, displacement APIs

## Metadata

**Confidence breakdown:**
- Standard stack: **HIGH** - All dependencies already integrated, versions confirmed
- Architecture: **MEDIUM-HIGH** - Bevy patterns proven, TUI patterns derived from Ratatui docs
- Pitfalls: **MEDIUM** - TUI-specific pitfalls inferred from terminal limitations, Bevy pitfalls from common patterns

**Research date:** 2026-02-14
**Valid until:** 60 days for stable stack (Bevy 0.15, Ratatui 0.29), 7 days for fast-moving diagnostics ecosystem

**Key unknowns requiring validation during planning:**
1. Exact CA grid timing measurement point (single system vs full schedule)
2. TUI mouse painting performance ceiling
3. Snapshot size/speed for GameState (needs profiling)
4. Emergency cleanup threshold tuning (requires stress testing)
5. Bevy/TUI feature parity decisions (UX design)
