---
phase: 03
plan: 01
subsystem: omega-bevy
tags: [bevy, color, theme, adapter, resource]
dependency_graph:
  requires: [phase-01-complete, omega-core-color-module]
  provides: [bevy-color-adapter, bevy-theme-resource]
  affects: [omega-bevy-presentation]
tech_stack:
  added: [bevy-color-conversion, embedded-toml-themes]
  patterns: [resource-wrapper, semantic-color-resolution]
key_files:
  created:
    - crates/omega-bevy/src/presentation/color_adapter.rs
    - crates/omega-bevy/src/presentation/bevy_theme.rs
  modified:
    - crates/omega-bevy/src/presentation/mod.rs
decisions:
  - "Embed themes via include_str! for zero filesystem dependency"
  - "Convert HexColor to Bevy::Color using sRGB (r/255.0, g/255.0, b/255.0)"
  - "Keep ThemeTokens alive for backward compatibility until plan 03-04"
  - "Load classic theme by default during plugin initialization"
metrics:
  duration_minutes: 5
  completed_date: "2026-02-13"
  tasks_completed: 3
  files_created: 2
  files_modified: 1
  commits: 3
---

# Phase 03 Plan 01: Bevy Theme Foundation Summary

Established semantic color infrastructure for omega-bevy by creating ColorAdapter and BevyTheme resource, enabling theme-based rendering with embedded classic theme.

## Completed Tasks

### Task 2.1: Color Adapter Implementation
**Commit:** `f1a11a3`

Created `color_adapter.rs` module with conversion utilities:
- `to_bevy_color(hex: &HexColor) -> Color` - Converts HexColor to Bevy sRGB color
- `resolve_to_bevy_color(theme: &ColorTheme, id: &ColorId) -> Color` - Resolves ColorId through theme
- `load_builtin_theme(name: &str) -> Result<ColorTheme>` - Loads embedded themes

**Key Implementation:**
```rust
pub fn to_bevy_color(hex: &HexColor) -> Color {
    let (r, g, b) = hex.to_rgb();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
```

**Embedded Themes:**
- Classic theme: `include_str!("../../../omega-content/themes/classic.toml")`
- Accessible theme: `include_str!("../../../omega-content/themes/accessible.toml")`

**Tests:** 10 tests covering RGB conversions, theme loading, and fallback behavior.

### Task 2.2: Theme Resource
**Commit:** `994b807`

Created `BevyTheme` as Bevy Resource wrapping `ColorTheme`:
- Generic `resolve(&ColorId) -> Color` for any color lookup
- `resolve_both(&ColorId) -> (Color, Color)` for foreground + background
- 70+ convenience methods organized by category:
  - **Entity colors:** `get_player_color()`, `get_monster_hostile_undead()`, etc.
  - **Item colors:** `get_item_common()` through `get_item_legendary()`
  - **Terrain colors:** `get_terrain_wall_stone()`, `get_terrain_water()`, etc.
  - **UI colors:** `get_ui_health_high()`, `get_ui_cursor()`, etc.
  - **Effect colors:** `get_effect_fire()`, `get_effect_lightning()`, etc.
  - **Environment colors:** `get_environment_light_torch()`, `get_environment_fog()`

**Design Pattern:** Resource wrapper with semantic method names matching game domain.

**Tests:** 10 tests covering all color categories and resolution methods.

### Task 2.3: Plugin Integration
**Commit:** `f785159`

Updated `ArcaneCartographerPlugin` to load and insert `BevyTheme`:
- Loads classic theme via `load_builtin_theme("classic")` during plugin build
- Inserts `BevyTheme` as Bevy resource
- Maintains `ThemeTokens` for backward compatibility (removed in plan 03-04)

**Plugin Code:**
```rust
impl Plugin for ArcaneCartographerPlugin {
    fn build(&self, app: &mut App) {
        let color_theme = color_adapter::load_builtin_theme("classic")
            .expect("Failed to load classic theme");
        let bevy_theme = BevyTheme::new(color_theme);

        app.insert_resource(bevy_theme)
            .insert_resource(theme::ThemeTokens::default())
            // ... other resources
    }
}
```

**Tests:** 2 integration tests verifying BevyTheme and all plugin resources are inserted.

## Deviations from Plan

None. Plan executed exactly as written.

## Verification Results

All verification criteria met:

### Unit Tests (Task 2.1)
- ✅ `to_bevy_color` converts #FF0000 → Color::srgb(1.0, 0.0, 0.0)
- ✅ `to_bevy_color` converts all pure colors correctly (red, green, blue, white, black, gray)
- ✅ `resolve_to_bevy_color` works with classic theme
- ✅ `resolve_to_bevy_color` returns white fallback for missing ColorId
- ✅ `load_builtin_theme` loads classic and accessible themes
- ✅ Case-insensitive theme name matching

### Integration Check (Task 2.3)
- ✅ `ArcaneCartographerPlugin` inserts `BevyTheme` resource into ECS world
- ✅ Inserted theme is classic theme (verified via `theme.meta.name`)
- ✅ All plugin resources inserted (BevyTheme, ThemeTokens, UiReadabilityConfig, etc.)

### Compilation
- ✅ `cargo check --package omega-bevy` passes without errors
- ✅ All tests compile successfully

## Technical Notes

### Color Conversion Formula
Bevy uses linear sRGB with f32 values [0.0, 1.0], so conversion from u8 [0, 255]:
```rust
f32_value = u8_value as f32 / 255.0
```

### Theme Embedding Strategy
Following omega-tui pattern:
- Themes embedded via `include_str!` at compile time
- Zero filesystem dependency for built-in themes
- Guaranteed valid themes (parse errors caught at build time if TOML invalid)
- Custom themes can still be loaded from files in future plans

### Resource Wrapper Pattern
`BevyTheme` provides three resolution levels:
1. **Generic:** `resolve(&ColorId)` for dynamic lookups
2. **Typed:** `get_monster_color(MonsterColorId)` for category-specific lookups
3. **Semantic:** `get_monster_hostile_undead()` for direct domain names

This hierarchy enables both flexibility and ergonomics.

## Files Created

- `crates/omega-bevy/src/presentation/color_adapter.rs` (254 lines)
  - Color conversion functions
  - Embedded theme constants
  - Theme loading utilities
  - 10 unit tests

- `crates/omega-bevy/src/presentation/bevy_theme.rs` (479 lines)
  - BevyTheme resource wrapper
  - 70+ convenience methods
  - Organized by color category
  - 10 unit tests

## Files Modified

- `crates/omega-bevy/src/presentation/mod.rs`
  - Added `color_adapter` module
  - Added `bevy_theme` module
  - Re-exported `BevyTheme` for convenience
  - Updated `ArcaneCartographerPlugin` to load and insert BevyTheme
  - Added 2 plugin integration tests

## Commits

| Commit | Description | Files Changed |
|--------|-------------|---------------|
| `f1a11a3` | Implement color adapter for Bevy | 2 files (+578 lines) |
| `994b807` | Add BevyTheme resource wrapper | 2 files (+497 lines) |
| `f785159` | Integrate BevyTheme into plugin | 1 file (+54, -1 lines) |

## Next Steps

**Plan 03-02:** Entity color application
- Apply BevyTheme colors to TileRender entities
- Color player, monsters, items, and terrain
- Replace hardcoded sprite colors with semantic lookups

**Plan 03-03:** UI color integration
- Apply theme colors to Bevy UI components
- Color HUD elements, messages, and interactions
- Update panel rendering to use BevyTheme

**Plan 03-04:** Remove ThemeTokens
- Migrate all ThemeTokens usage to BevyTheme
- Remove hardcoded color constants
- Complete migration to semantic color system

## Self-Check: PASSED

**Created files exist:**
```
FOUND: crates/omega-bevy/src/presentation/color_adapter.rs
FOUND: crates/omega-bevy/src/presentation/bevy_theme.rs
```

**Modified files exist:**
```
FOUND: crates/omega-bevy/src/presentation/mod.rs
```

**Commits exist:**
```
FOUND: f1a11a3
FOUND: 994b807
FOUND: f785159
```

**Compilation:**
```
cargo check --package omega-bevy: PASSED
```

All deliverables verified. Plan 03-01 complete.
