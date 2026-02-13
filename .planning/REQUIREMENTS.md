# Colorful Omega - Requirements Specification

**Project:** Colorful Omega - Terminal/UI Color Support  
**Version:** 1.0  
**Last Updated:** February 12, 2026  
**Status:** Approved - Ready for Implementation

---

## 1. Overview

This document defines the functional and non-functional requirements for the Colorful Omega project. All requirements are mapped to implementation phases in ROADMAP.md.

### Document Structure

- **COLOR-***: Core color system requirements
- **TUI-***: Terminal UI integration requirements
- **BEVY-***: Bevy GUI integration requirements
- **CUSTOM-***: User customization requirements
- **ADV-***: Advanced feature requirements
- **A11Y-***: Accessibility requirements
- **PERF-***: Performance requirements
- **COMPAT-***: Compatibility requirements

---

## 2. Functional Requirements

### 2.1 Core Color System (COLOR)

#### COLOR-01: Semantic Color Enumeration
**Priority:** High  
**Phase:** 1

Define a `ColorId` enum that provides semantic color identifiers for all game entities, decoupling color logic from color values.

**Acceptance Criteria:**
- Enum includes variants for: item rarities, monster types, terrain, UI elements, environmental effects
- Each variant has clear documentation explaining its semantic meaning
- Enum is serializable with serde
- Enum supports iteration over all variants

**Success Metrics:**
- All game entity types have corresponding ColorId variants
- No hardcoded color values in game logic (only ColorId references)

---

#### COLOR-02: Theme Definition and Serialization
**Priority:** High  
**Phase:** 1

Implement a `ColorTheme` struct that maps ColorId values to concrete color specifications, with TOML serialization support.

**Acceptance Criteria:**
- Theme struct maps ColorId → ColorSpec (supporting multiple color spaces)
- TOML serialization/deserialization using serde
- Theme can be loaded from file or embedded as default
- Graceful handling of missing theme entries (fallback to default)
- Theme validation (detect invalid color values, missing entries)

**Success Metrics:**
- Theme files can be saved and loaded without data loss
- Invalid themes produce helpful error messages
- Theme loading completes in < 50ms

---

#### COLOR-03: Multi-Color-Space Support
**Priority:** High  
**Phase:** 1

Support multiple color space representations: Hex (sRGB), OKLCH, and ANSI.

**Acceptance Criteria:**
- Hex format: `#RRGGBB` and `#RGB` shorthand
- OKLCH format: `{ l = 0.0-1.0, c = 0.0+, h = 0-360 }`
- ANSI format: numeric index 0-255 for 256-color, 0-15 for 16-color
- Automatic conversion between spaces using `palette` crate
- Perceptual uniformity maintained in OKLCH space

**Success Metrics:**
- All three formats parse correctly
- Conversions between spaces are accurate (within 1% tolerance)
- OKLCH provides smoother color transitions than sRGB

---

#### COLOR-04: Built-in Themes
**Priority:** High  
**Phase:** 1

Provide three built-in themes covering different use cases and accessibility needs.

**Acceptance Criteria:**
- **Classic Theme:** NetHack/Angband-inspired roguelike colors
  - Red for hostiles/damage
  - Green for nature/friendly
  - Blue for water/magic
  - Yellow for treasure
  - Purple for rare items
- **Accessible Theme:** Colorblind-safe, high contrast
  - Avoid red-green color pairs
  - WCAG AAA contrast ratios
  - Shape/symbol differentiation where possible
- **Modern Theme:** Contemporary aesthetics
  - High contrast
  - Reduced color palette
  - Consistent with modern UI trends

**Success Metrics:**
- All three themes pass validation
- All three themes meet WCAG AA minimum
- Accessible theme meets WCAG AAA

---

#### COLOR-05: Terminal Capability Detection
**Priority:** High  
**Phase:** 1

Detect terminal color capabilities at runtime and adapt color output accordingly.

**Acceptance Criteria:**
- Check `COLORTERM` environment variable for truecolor support
- Check `TERM` environment variable for 256-color support
- Check `NO_COLOR` environment variable to disable colors
- Use `termprofile` crate for robust detection
- Fallback chain: TrueColor → Ansi256 → Ansi16 → None
- Cache detection result for performance

**Success Metrics:**
- Correctly identifies terminal capabilities on:
  - macOS (iTerm2, Terminal.app)
  - Linux (GNOME Terminal, Konsole, xterm)
  - Windows (Windows Terminal, legacy cmd)
- Respects `NO_COLOR` in all cases

---

### 2.2 TUI Integration (TUI)

#### TUI-01: ratatui Style Adapter
**Priority:** High  
**Phase:** 2

Create an adapter that converts Theme + ColorId into ratatui's Style type.

**Acceptance Criteria:**
- Function `theme_to_style(theme: &Theme, id: ColorId) -> Style`
- Handles foreground and background colors
- Supports style modifiers (bold, italic, underline) via theme
- Caches resolved styles for O(1) lookup
- Returns appropriate Style for terminal capability level

**Success Metrics:**
- Adapter works with all ratatui widgets
- No per-frame allocations
- Style lookup < 1μs

---

#### TUI-02: Colored Map Rendering
**Priority:** High  
**Phase:** 2

Render the dungeon map with appropriate colors for each entity type.

**Acceptance Criteria:**
- Walls, floors, doors, stairs have distinct colors
- Monsters colored by disposition (hostile/neutral/friendly)
- Items colored by rarity
- Environmental effects (fire, water, poison) have thematic colors
- Batch style changes (don't change style per character)

**Success Metrics:**
- Map renders correctly with all three built-in themes
- ANSI sequences batched efficiently
- No visual artifacts or flickering

---

#### TUI-03: Colored Status and Log Panels
**Priority:** High  
**Phase:** 2

Add color to status bar and message log panels.

**Acceptance Criteria:**
- Status bar: Health colored by level (green/yellow/red), mana in blue
- Message log: Different colors for message types (info, warning, danger, success)
- Highlight current player status effects with appropriate colors
- Dim outdated messages

**Success Metrics:**
- Status bar colors update in real-time
- Message log colors persist correctly
- Color changes don't cause layout shifts

---

#### TUI-04: CLI Theme Option
**Priority:** Medium  
**Phase:** 2

Add `--theme` command-line option for selecting theme on startup.

**Acceptance Criteria:**
- `--theme <name>` selects built-in or user theme by name
- `--theme <path>` loads theme from file path
- Invalid theme name shows helpful error with available options
- Theme loaded before any rendering occurs

**Success Metrics:**
- CLI option documented in `--help`
- Works with both built-in and user themes
- Error messages are actionable

---

#### TUI-05: Runtime Theme Switching
**Priority:** Medium  
**Phase:** 2

Allow users to switch themes without restarting the game.

**Acceptance Criteria:**
- Key binding or menu option to open theme selector
- Theme changes apply immediately
- No memory leaks when switching themes
- Current theme persisted to config

**Success Metrics:**
- Theme switch completes in < 100ms
- No screen corruption during switch
- Config updated with new selection

---

### 2.3 Bevy Integration (BEVY)

#### BEVY-01: Bevy Color Adapter
**Priority:** High  
**Phase:** 3

Create an adapter that converts Theme + ColorId into Bevy's Color type.

**Acceptance Criteria:**
- Function `theme_to_bevy_color(theme: &Theme, id: ColorId) -> Color`
- Supports both sRGB and LinearRGB color spaces
- Handles alpha/transparency if specified in theme
- Returns Color in appropriate space for Bevy rendering

**Success Metrics:**
- Colors match TUI rendering (perceptually)
- No color banding or artifacts
- Performance overhead < 5%

---

#### BEVY-02: Colored Sprite Rendering
**Priority:** High  
**Phase:** 3

Render game entities with themed colors using Bevy sprites.

**Acceptance Criteria:**
- Sprite tinting based on entity ColorId
- Support for tileset-based and ASCII rendering modes
- Environmental lighting affects sprite colors appropriately
- Consistent coloring with TUI frontend

**Success Metrics:**
- Sprites render with correct colors
- Color changes apply immediately
- No z-fighting or rendering artifacts

---

#### BEVY-03: Colored Text Rendering
**Priority:** High  
**Phase:** 3

Render UI text with themed colors using Bevy's text system.

**Acceptance Criteria:**
- TextColor component set from theme
- Support for rich text (different colors within single text block)
- UI panels (health, mana, status) use appropriate theme colors
- Font rendering remains crisp with colored text

**Success Metrics:**
- Text colors match TUI rendering
- No performance degradation with colored text
- Accessible contrast ratios maintained

---

#### BEVY-04: Theme Resource and Events
**Priority:** High  
**Phase:** 3

Integrate theme management into Bevy's ECS as a resource with change events.

**Acceptance Criteria:**
- `ThemeResource` holds current theme
- `ThemeChanged` event fired when theme updates
- Systems respond to theme changes (re-render with new colors)
- Theme resource accessible from any system

**Success Metrics:**
- Theme change propagates to all relevant systems
- No systems miss theme change events
- Resource access doesn't block rendering

---

### 2.4 User Customization (CUSTOM)

#### CUSTOM-01: User Theme Directory
**Priority:** Medium  
**Phase:** 4

Support loading themes from user configuration directory.

**Acceptance Criteria:**
- Load themes from `~/.config/omega/themes/` (Linux/macOS)
- Load themes from `%APPDATA%\omega\themes\` (Windows)
- Support arbitrary TOML files in theme directory
- User themes override built-in themes with same name
- Invalid themes don't crash the game (graceful skip + log warning)

**Success Metrics:**
- Themes load from user directory correctly
- Theme names don't collide with built-ins unexpectedly
- Invalid themes produce helpful log messages

---

#### CUSTOM-02: Theme Hot-Reload
**Priority:** Medium  
**Phase:** 4

Implement file watching for automatic theme reloading during development.

**Acceptance Criteria:**
- Watch user theme directory for changes
- Reload modified themes automatically
- Debounce rapid file changes (e.g., during save)
- Hot-reload available in debug builds only
- Log theme reload events

**Success Metrics:**
- Theme changes detected within 1 second
- No duplicate reloads from rapid saves
- No performance impact when not watching

---

#### CUSTOM-03: omega-theme CLI Tool
**Priority:** Medium  
**Phase:** 4

Create a command-line tool for theme development and management.

**Acceptance Criteria:**
- Binary: `omega-theme`
- Subcommands: `validate`, `preview`, `convert`, `export`
- Helpful error messages and usage documentation
- Exit codes: 0 for success, non-zero for errors

**Success Metrics:**
- Tool installs with `cargo install omega-tools`
- All subcommands work as documented
- Tool is scriptable (exit codes, JSON output where appropriate)

---

#### CUSTOM-04: Theme Validation
**Priority:** Medium  
**Phase:** 4

Implement comprehensive theme validation with helpful error reporting.

**Acceptance Criteria:**
- Validate TOML syntax
- Check all required ColorId mappings present
- Verify color values are valid (correct hex format, valid OKLCH ranges)
- Check for contrast issues (WCAG AA warnings)
- Provide line numbers for errors
- Suggest fixes for common issues

**Success Metrics:**
- Catches all invalid themes before game loads them
- Error messages pinpoint exact problems
- Validation completes in < 100ms

---

#### CUSTOM-05: Theme Preview
**Priority:** Medium  
**Phase:** 4

Add preview capability to omega-theme CLI.

**Acceptance Criteria:**
- `omega-theme preview --theme <path>` renders sample UI
- Show all ColorId mappings
- Display sample game entities (player, monsters, items)
- Show color swatches with hex/OKLCH values
- Option to preview different UI states (combat, inventory, dialog)

**Success Metrics:**
- Preview renders in terminal
- All ColorIds visible
- Output is useful for theme debugging

---

#### CUSTOM-06: Theme Export
**Priority:** Low  
**Phase:** 4

Export Omega themes to other terminal emulator formats.

**Acceptance Criteria:**
- `omega-theme export --format alacritty`
- `omega-theme export --format wezterm`
- `omega-theme export --format kitty`
- Exported files are valid for target emulator
- Maps Omega semantic colors to emulator color slots

**Success Metrics:**
- Exported themes load in target emulators
- Colors are reasonably mapped
- Documentation explains mapping strategy

---

### 2.5 Advanced Features (ADV)

#### ADV-01: Color Animations
**Priority:** Low  
**Phase:** 5

Support animated color transitions and effects.

**Acceptance Criteria:**
- Flashing warnings for critical low health
- Smooth transitions when theme changes
- Pulsing effects for important UI elements
- Respect `prefers-reduced-motion` accessibility setting

**Success Metrics:**
- Animations run at 60fps
- No CPU overhead when animations inactive
- Reduced motion mode disables animations

---

#### ADV-02: Per-Environment Themes
**Priority:** Low  
**Phase:** 5

Allow different color schemes for different game environments.

**Acceptance Criteria:**
- Dungeon environment: dark, atmospheric
- City environment: brighter, warmer tones
- Forest environment: green palette
- Volcano environment: red/orange palette
- Automatic environment detection or manual assignment

**Success Metrics:**
- Environment changes trigger theme change
- Smooth transition between environments
- Configurable per save file

---

#### ADV-03: Dynamic Lighting
**Priority:** Low  
**Phase:** 5

Implement dynamic lighting effects in Bevy frontend.

**Acceptance Criteria:**
- Torch light effect around player
- Colored lights from environmental sources (lava, magic)
- Light radius and intensity affect entity colors
- Performance maintained with multiple light sources

**Success Metrics:**
- Lighting looks convincing
- Maintains 60fps on target hardware
- Configurable light intensity

---

#### ADV-04: Procedural Colors
**Priority:** Low  
**Phase:** 5

Generate distinct colors procedurally for item rarities or other categories.

**Acceptance Criteria:**
- Generate N visually distinct colors
- Ensure minimum perceptual distance between colors
- Avoid problematic color pairs (red-green for CVD)
- Deterministic generation (same seed → same colors)

**Success Metrics:**
- Generated colors are distinct
- Pass colorblind simulation tests
- Generation completes in < 10ms

---

## 3. Non-Functional Requirements

### 3.1 Accessibility (A11Y)

#### A11Y-01: WCAG AA Contrast Compliance
**Priority:** High  
**Phase:** 1-4

All text and UI elements must meet WCAG 2.2 AA contrast requirements (4.5:1 for normal text, 3:1 for large text).

**Acceptance Criteria:**
- Automated contrast checking in CI
- All built-in themes pass AA validation
- Contrast ratio calculable for any color pair
- Failures block PR merge

**Success Metrics:**
- 100% of UI elements pass AA
- CI contrast check runs on every PR

---

#### A11Y-02: Colorblind-Friendly Themes
**Priority:** High  
**Phase:** 1

Provide themes that work for users with color vision deficiencies.

**Acceptance Criteria:**
- Accessible theme avoids red-green pairs
- Alternative indicators (symbols, text labels) where color is critical
- Tested with deuteranopia and protanopia simulation
- Documentation explains colorblind support

**Success Metrics:**
- Accessible theme usable by CVD users
- Game is playable without relying on color alone

---

#### A11Y-03: NO_COLOR Support
**Priority:** High  
**Phase:** 1-2

Respect the `NO_COLOR` environment variable (de facto standard for disabling colored output).

**Acceptance Criteria:**
- Check `NO_COLOR` environment variable
- When set, disable all color output
- Still show symbols/shapes that indicate meaning
- Document NO_COLOR support

**Success Metrics:**
- Game runs correctly with NO_COLOR=1
- No ANSI escape sequences emitted when disabled

---

#### A11Y-04: High Contrast Theme
**Priority:** Medium  
**Phase:** 1

Provide an extreme high-contrast theme for users who need maximum differentiation.

**Acceptance Criteria:**
- WCAG AAA contrast ratios (7:1)
- Pure black/white where appropriate
- No subtle color distinctions
- Sharp borders on all UI elements

**Success Metrics:**
- High contrast theme passes AAA validation
- Usable by users with low vision

---

#### A11Y-05: Automated Contrast Regression Testing
**Priority:** Medium  
**Phase:** 4

Prevent contrast regressions through automated CI checks.

**Acceptance Criteria:**
- CI job calculates contrast ratios for all theme color pairs
- Fails build if any pair falls below AA threshold
- Generates report of all contrast ratios
- Tracks contrast trends over time

**Success Metrics:**
- CI catches contrast regressions before merge
- Reports are readable and actionable

---

### 3.2 Performance (PERF)

#### PERF-01: Theme Loading Performance
**Priority:** High  
**Phase:** 1

Theme loading must complete quickly to avoid startup delays.

**Acceptance Criteria:**
- Theme loading < 50ms from file
- Embedded themes < 10ms
- No blocking I/O on main thread
- Lazy loading of unused theme elements

**Success Metrics:**
- Benchmark shows < 50ms load time
- No perceptible startup delay

---

#### PERF-02: Color Lookup Performance
**Priority:** High  
**Phase:** 1-2

ColorId to concrete color lookup must be O(1) and fast.

**Acceptance Criteria:**
- Use precomputed hash map for lookups
- Cache resolved ratatui/Bevy colors
- Lookup time < 1μs per color
- No allocations during lookup

**Success Metrics:**
- Benchmark shows < 1μs lookup
- No regressions in frame time

---

#### PERF-03: Render Loop Performance
**Priority:** High  
**Phase:** 2-3

Color rendering must not impact frame time.

**Acceptance Criteria:**
- No per-frame allocations in color code
- Batch style changes (TUI)
- Minimal system overhead (Bevy)
- Color overhead < 5% of total render time

**Success Metrics:**
- Profiling shows < 5% overhead
- No FPS drops attributed to color system

---

#### PERF-04: TrueColor Overhead
**Priority:** Medium  
**Phase:** 2

TrueColor (24-bit) must have minimal overhead vs 256-color.

**Acceptance Criteria:**
- TrueColor overhead < 5% vs 256-color
- Automatic fallback to 256-color when beneficial
- No extra allocations for TrueColor

**Success Metrics:**
- Benchmark shows < 5% difference
- Users can force specific modes if needed

---

### 3.3 Compatibility (COMPAT)

#### COMPAT-01: Existing colour Field Preservation
**Priority:** High  
**Phase:** 1

Preserve the existing `RuntimeOptions.colour: bool` field for backward compatibility.

**Acceptance Criteria:**
- Existing field remains in struct
- Use `#[serde(default)]` for new fields
- `colour: true` enables auto-detection
- `colour: false` disables all colors

**Success Metrics:**
- Old configs load without modification
- No breaking changes to existing API

---

#### COMPAT-02: Save File Compatibility
**Priority:** High  
**Phase:** 1

Existing save files must load without errors after color system changes.

**Acceptance Criteria:**
- Save format changes don't break loading
- Serde defaults handle missing color fields
- Test with legacy save files
- Migration path documented if needed

**Success Metrics:**
- All legacy saves load successfully
- No data loss during migration

---

#### COMPAT-03: Graceful Degradation
**Priority:** High  
**Phase:** 1-2

System must work on terminals with limited color support.

**Acceptance Criteria:**
- 16-color terminals show appropriate colors
- Monochrome terminals show symbols/intensity differences
- No crashes on limited terminals
- Clear messaging about limited capabilities

**Success Metrics:**
- Game playable on 16-color terminals
- Monochrome mode functional

---

#### COMPAT-04: Config Migration
**Priority:** Medium  
**Phase:** 1

Support migration of configuration files when format changes.

**Acceptance Criteria:**
- Add `version` field to config
- Detect old versions and migrate automatically
- Log migration events
- Document migration rules

**Success Metrics:**
- Configs from v0.x load in v1.x
- Migrations are transparent to users

---

## 4. Traceability Matrix

| Requirement | Phase | Status | Test Method |
|-------------|-------|--------|-------------|
| COLOR-01 | 1 | Pending | Unit test enum variants |
| COLOR-02 | 1 | Pending | Integration test load/save |
| COLOR-03 | 1 | Pending | Unit test conversions |
| COLOR-04 | 1 | Pending | Visual validation |
| COLOR-05 | 1 | Pending | Integration test detection |
| TUI-01 | 2 | Pending | Unit test adapter |
| TUI-02 | 2 | Pending | Visual regression test |
| TUI-03 | 2 | Pending | Visual regression test |
| TUI-04 | 2 | Pending | CLI test |
| TUI-05 | 2 | Pending | Integration test |
| BEVY-01 | 3 | Pending | Unit test adapter |
| BEVY-02 | 3 | Pending | Visual test |
| BEVY-03 | 3 | Pending | Visual test |
| BEVY-04 | 3 | Pending | Unit test conversion |
| CUSTOM-01 | 4 | Pending | Integration test loading |
| CUSTOM-02 | 4 | Pending | Manual test |
| CUSTOM-03 | 4 | Pending | CLI test |
| CUSTOM-04 | 4 | Pending | Unit test validation |
| CUSTOM-05 | 4 | Pending | CLI test |
| CUSTOM-06 | 4 | Pending | Export validation |
| ADV-01 | 5 | Pending | Visual test |
| ADV-02 | 5 | Pending | Integration test |
| ADV-03 | 5 | Pending | Visual test |
| ADV-04 | 5 | Pending | Unit test generation |
| A11Y-01 | 1-4 | Pending | Automated CI check |
| A11Y-02 | 1 | Pending | CVD simulation test |
| A11Y-03 | 1-2 | Pending | Environment test |
| A11Y-04 | 1 | Pending | Contrast test |
| A11Y-05 | 4 | Pending | CI integration |
| PERF-01 | 1 | Pending | Benchmark |
| PERF-02 | 1-2 | Pending | Benchmark |
| PERF-03 | 2-3 | Pending | Profiling |
| PERF-04 | 2 | Pending | Benchmark |
| COMPAT-01 | 1 | Pending | Integration test |
| COMPAT-02 | 1 | Pending | Legacy save test |
| COMPAT-03 | 1-2 | Pending | Terminal test matrix |
| COMPAT-04 | 1 | Pending | Migration test |

**Total Requirements:** 34  
**Mapped:** 34/34 ✓  
**Coverage:** 100%

---

## 5. Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-12 | Claude | Initial requirements based on research synthesis |

**Next Review:** After Phase 2 completion
