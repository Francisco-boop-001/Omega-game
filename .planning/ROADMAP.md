# Colorful Omega - Project Roadmap

**Project:** Colorful Omega - Terminal/UI Color Support for Omega Roguelike
**Version:** 1.0
**Last Updated:** February 12, 2026
**Status:** Phase 1 Complete - Phase 2 Planned (TUI Integration)

---

## 1. Project Overview

Colorful Omega adds comprehensive color and theming support to the Omega roguelike game during its C-to-Rust migration. The project implements a semantic color abstraction layer that decouples game logic from color values, enabling consistent theming across both TUI (terminal) and GUI (Bevy) frontends.

### Core Value Proposition

- **Visual Enhancement:** Bring the Omega dungeon to life with meaningful, atmospheric color
- **Accessibility First:** Built-in colorblind support and WCAG-compliant contrast ratios
- **User Control:** Customizable themes via TOML configuration
- **Developer Friendly:** Hot-reload, validation tools, and comprehensive documentation

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    GAME LOGIC (omega-core)                   │
│  ┌─────────────┐    ┌──────────────┐    ┌──────────────┐   │
│  │  ColorId    │───▶│    Theme     │───▶│ ColorSpec    │   │
│  │  (Semantic) │    │  (TOML/JSON) │    │ (Concrete)   │   │
│  └─────────────┘    └──────────────┘    └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
              ▼                               ▼
┌─────────────────────────┐    ┌─────────────────────────────┐
│  omega-tui (Terminal)   │    │    omega-bevy (GUI)         │
│  ┌─────────────────┐    │    │  ┌─────────────────────┐    │
│  │ ratatui Adapter │    │    │  │ bevy_color Adapter  │    │
│  │  Style          │    │    │  │  TextColor          │    │
│  │  Color::Rgb     │    │    │  │  ColorMaterial      │    │
│  └─────────────────┘    │    │  └─────────────────────┘    │
└─────────────────────────┘    └─────────────────────────────┘
```

### Key Technologies

| Component | Technology | Purpose |
|-----------|------------|---------|
| Core Types | Rust + serde | Semantic color system |
| Theme Format | TOML | Human-readable configuration |
| Color Spaces | OKLCH, sRGB, ANSI | Accessibility + terminal support |
| TUI Colors | ratatui 0.29 | Terminal color rendering |
| GUI Colors | bevy_color 0.15 | GUI color rendering |
| Detection | termprofile 0.2 | Terminal capability detection |
| Validation | palette 0.7 | Contrast calculations |

---

## 2. Requirements Summary

### 2.1 Functional Requirements

#### Core Color System (COLOR-01 through COLOR-05)

| ID | Requirement | Priority | Phase |
|----|-------------|----------|-------|
| COLOR-01 | Define semantic `ColorId` enum for game entities | High | 1 |
| COLOR-02 | Implement `ColorTheme` struct with TOML serialization | High | 1 |
| COLOR-03 | Support multiple color spaces (Hex, OKLCH, ANSI) | High | 1 |
| COLOR-04 | Provide 3 built-in themes (classic, accessible, modern) | High | 1 |
| COLOR-05 | Implement terminal capability detection and adaptation | High | 1 |

#### TUI Integration (TUI-01 through TUI-05)

| ID | Requirement | Priority | Phase |
|----|-------------|----------|-------|
| TUI-01 | Create Theme → ratatui::Style adapter | High | 2 |
| TUI-02 | Implement colored map rendering | High | 2 |
| TUI-03 | Add colored status and log panels | High | 2 |
| TUI-04 | Support `--theme` CLI option | Medium | 2 |
| TUI-05 | Enable runtime theme switching | Medium | 2 |

#### Bevy Integration (BEVY-01 through BEVY-04)

| ID | Requirement | Priority | Phase |
|----|-------------|----------|-------|
| BEVY-01 | Create Theme → bevy::Color adapter | High | 3 |
| BEVY-02 | Implement colored sprite/text rendering | High | 3 |
| BEVY-03 | Add theme resource for Bevy ECS | High | 3 |
| BEVY-04 | Handle color space conversions (sRGB vs Linear) | Medium | 3 |

#### User Customization (CUSTOM-01 through CUSTOM-06)

| ID | Requirement | Priority | Phase |
|----|-------------|----------|-------|
| CUSTOM-01 | Support user theme directory (`~/.config/omega/themes/`) | Medium | 4 |
| CUSTOM-02 | Implement theme hot-reload for development | Medium | 4 |
| CUSTOM-03 | Create `omega-theme` CLI tool (validate, preview, convert) | Medium | 4 |
| CUSTOM-04 | Add WCAG contrast checking in CI | Medium | 4 |
| CUSTOM-05 | Build theme editor/debugger | Low | 4 |
| CUSTOM-06 | Export themes to other formats (alacritty, wezterm) | Low | 4 |

#### Advanced Features (ADV-01 through ADV-04)

| ID | Requirement | Priority | Phase |
|----|-------------|----------|-------|
| ADV-01 | Color animations and transitions | Low | 5 |
| ADV-02 | Per-environment themes (city vs dungeon) | Low | 5 |
| ADV-03 | Dynamic lighting effects | Low | 5 |
| ADV-04 | Procedural color generation for item rarities | Low | 5 |

### 2.2 Non-Functional Requirements

#### Accessibility (A11Y)

| ID | Requirement | Priority |
|----|-------------|----------|
| A11Y-01 | WCAG 2.2 AA contrast compliance (4.5:1 minimum) | High |
| A11Y-02 | Colorblind-friendly themes (avoid red-green pairs) | High |
| A11Y-03 | Support `NO_COLOR` environment variable | High |
| A11Y-04 | High-contrast theme option | Medium |
| A11Y-05 | Automated contrast regression testing | Medium |

#### Performance (PERF)

| ID | Requirement | Target |
|----|-------------|--------|
| PERF-01 | Theme loading < 50ms | < 50ms |
| PERF-02 | Color lookup O(1) via precomputed maps | O(1) |
| PERF-03 | No per-frame allocations in render loop | Zero |
| PERF-04 | TrueColor overhead < 5% vs 256-color | < 5% |

#### Compatibility (COMPAT)

| ID | Requirement | Priority |
|----|-------------|----------|
| COMPAT-01 | Preserve existing `RuntimeOptions.colour` field | High |
| COMPAT-02 | Load existing save files without errors | High |
| COMPAT-03 | Graceful degradation for limited terminals | High |
| COMPAT-04 | Backward-compatible config migrations | Medium |

---

## 3. Phase Breakdown

### Phase 1: Foundation (Core Color Infrastructure)

**Goal:** Establish shared types and theme system in omega-core before frontend work

**Effort:** Medium (2-3 weeks)
**Dependencies:** None
**Research Flag:** SKIP (well-documented patterns)

#### Success Criteria

1. Developers can define colors using `ColorId` semantics instead of hardcoded values
2. Three built-in themes load successfully and pass validation
3. Terminal capability detection identifies TrueColor, 256-color, 16-color, and no-color modes
4. Theme serialization/deserialization works with TOML
5. All new code has >80% test coverage

#### Tasks

| Task | Description | Effort | Owner |
|------|-------------|--------|-------|
| 1.1 | Define `ColorId` enum with semantic meanings | S | Core |
| 1.2 | Create `ColorSpec` struct supporting multiple color spaces | S | Core |
| 1.3 | Implement `ColorTheme` struct with TOML serialization | M | Core |
| 1.4 | Add terminal capability detection (`termprofile` integration) | M | Core |
| 1.5 | Create 3 built-in themes (classic, accessible, modern) | M | Content |
| 1.6 | Implement theme validation and default handling | S | Core |
| 1.7 | Add unit tests for all core types | M | Core |
| 1.8 | Write documentation for theme format | S | Docs |

#### Deliverables

- `omega-core/src/color/color_id.rs` - Semantic color enum
- `omega-core/src/color/color_spec.rs` - Color specification
- `omega-core/src/color/theme.rs` - Theme definition and loading
- `omega-core/src/color/capability.rs` - Terminal detection
- `omega-content/themes/` - Built-in theme files

#### Pitfalls to Avoid

- Don't hardcode colors in game logic (use ColorId)
- Don't skip accessibility considerations in built-in themes
- Don't break existing save format compatibility

---

### Phase 2: TUI Color Integration

**Goal:** Full color support in omega-tui with automatic terminal capability adaptation

**Effort:** Medium (2-3 weeks)
**Dependencies:** Phase 1
**Research Flag:** SKIP (ratatui API mature)
**Plans:** 3 plans

Plans:
- [ ] 02-01-PLAN.md -- Style adapter foundation (StyleCache + App integration)
- [ ] 02-02-PLAN.md -- Colored panel rendering (map, status, log, interaction)
- [ ] 02-03-PLAN.md -- CLI theme option, runtime switching, visual verification

#### Success Criteria

1. Users see colored dungeon maps with appropriate colors for entities
2. Status and log panels display colored text based on severity/type
3. Terminal auto-detection selects appropriate color mode on startup
4. Users can override theme via `--theme` CLI option
5. Theme can be changed at runtime without restart
6. `NO_COLOR` environment variable disables all colors

#### Tasks

| Task | Description | Effort | Owner |
|------|-------------|--------|-------|
| 2.1 | Create Theme → ratatui::Style adapter | M | TUI |
| 2.2 | Implement colored map tile rendering | M | TUI |
| 2.3 | Add colored status bar rendering | S | TUI |
| 2.4 | Add colored message log rendering | S | TUI |
| 2.5 | Integrate terminal capability detection | M | TUI |
| 2.6 | Add `--theme` CLI option | S | TUI |
| 2.7 | Implement runtime theme switching | M | TUI |
| 2.8 | Respect `NO_COLOR` environment variable | S | TUI |
| 2.9 | Add 16-color fallback mode | S | TUI |
| 2.10 | Write integration tests with snapshot testing | M | TUI |

#### Deliverables

- `omega-tui/src/color_adapter.rs` - Theme to ratatui adapter with StyleCache
- Updated `omega-tui/src/lib.rs` - Colored render functions, App theme integration
- Updated `omega-tui/src/bin/omega-tui-app.rs` - CLI --theme option handling

#### Pitfalls to Avoid

- Don't change style per character (batch for performance)
- Don't ignore `NO_COLOR` environment variable
- Don't forget 16-color fallback for old terminals
- Don't emit ANSI sequences when colors are disabled

---

### Phase 3: Bevy Color Integration

**Goal:** Consistent theming in omega-bevy GUI frontend

**Effort:** Medium (2-3 weeks)
**Dependencies:** Phase 1
**Research Flag:** OPTIONAL (Bevy integration details)

#### Success Criteria

1. GUI displays colored sprites and text consistent with TUI
2. Theme changes apply immediately without restart
3. Color spaces are correctly handled (sRGB vs Linear)
4. Performance impact is negligible (< 5% frame time)

#### Tasks

| Task | Description | Effort | Owner |
|------|-------------|--------|-------|
| 3.1 | Create Theme → bevy::Color adapter | M | Bevy |
| 3.2 | Implement colored sprite rendering | M | Bevy |
| 3.3 | Implement colored text rendering | M | Bevy |
| 3.4 | Add theme resource to Bevy ECS | S | Bevy |
| 3.5 | Create theme change event system | M | Bevy |
| 3.6 | Handle color space conversions | S | Bevy |
| 3.7 | Add GUI theme selector | S | Bevy |
| 3.8 | Write integration tests | M | Bevy |

#### Deliverables

- `omega-bevy/src/color_adapter.rs` - Theme to Bevy adapter
- `omega-bevy/src/systems/theme_systems.rs` - Theme management
- Updated `omega-bevy/src/main.rs` - Theme resource setup

#### Pitfalls to Avoid

- Don't create separate theme system (share omega-core types)
- Don't forget color space conversions (sRGB vs LinearRGB)
- Don't hardcode colors in Bevy systems

---

### Phase 4: User Customization & Tooling

**Goal:** Advanced features for power users, accessibility, and theme development

**Effort:** Large (3-4 weeks)
**Dependencies:** Phase 2, Phase 3
**Research Flag:** SKIP (tool patterns established)

#### Success Criteria

1. Users can create custom themes in `~/.config/omega/themes/`
2. Theme developers can hot-reload themes without restarting game
3. CLI tool validates themes and checks WCAG compliance
4. CI blocks PRs that introduce contrast regressions
5. Users can preview themes against different UI states

#### Tasks

| Task | Description | Effort | Owner |
|------|-------------|--------|-------|
| 4.1 | Implement user theme directory loading | M | Core |
| 4.2 | Add file watcher for hot-reload | M | Core |
| 4.3 | Create `omega-theme` CLI binary | L | Tools |
| 4.4 | Add `validate` subcommand | M | Tools |
| 4.5 | Add `preview` subcommand | M | Tools |
| 4.6 | Add `convert` subcommand (color space conversion) | S | Tools |
| 4.7 | Add `export` subcommand (to other formats) | S | Tools |
| 4.8 | Integrate WCAG contrast checking in CI | M | DevOps |
| 4.9 | Create theme editor/debugger UI | L | Bevy/TUI |
| 4.10 | Write tooling documentation | M | Docs |

#### Deliverables

- `omega-core/src/theme/loader.rs` - User theme loading
- `omega-tools/src/bin/omega-theme.rs` - CLI tool
- `.github/workflows/contrast-check.yml` - CI integration
- `omega-bevy/src/tools/theme_editor.rs` - Theme editor

#### Pitfalls to Avoid

- Don't allow invalid TOML to crash the game (graceful fallback)
- Don't break built-in themes with user customizations
- Don't require game restart for theme development

---

### Phase 5: Advanced Features

**Goal:** Enhanced visual features for future development

**Effort:** Large (4-6 weeks)
**Dependencies:** Phase 4
**Research Flag:** RECOMMENDED (terminal animations need validation)

#### Success Criteria

1. Critical UI elements can flash/pulse for warnings
2. Smooth color transitions between states
3. Different environments can have distinct color schemes
4. Item rarities use procedurally generated distinct colors

#### Tasks

| Task | Description | Effort | Owner |
|------|-------------|--------|-------|
| 5.1 | Research terminal animation feasibility | S | Research |
| 5.2 | Design animation system for TUI | M | TUI |
| 5.3 | Design animation system for Bevy | M | Bevy |
| 5.4 | Implement flashing/pulsing warnings | M | Both |
| 5.5 | Implement smooth color transitions | L | Both |
| 5.6 | Add per-environment theme support | M | Core |
| 5.7 | Implement procedural color generation | M | Core |
| 5.8 | Add dynamic lighting effects | L | Bevy |
| 5.9 | Performance optimization | M | Both |
| 5.10 | Documentation and examples | S | Docs |

#### Deliverables

- `omega-core/src/color/animation.rs` - Animation primitives
- `omega-tui/src/rendering/animations.rs` - TUI animations
- `omega-bevy/src/systems/color_animations.rs` - Bevy animations
- Environment-specific theme loading

#### Pitfalls to Avoid

- Don't let animation impact performance (frame drops)
- Don't add complexity before core features are stable
- Don't break accessibility with animations (respect prefers-reduced-motion)

---

## 4. Risk Assessment

### Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Terminal detection edge cases** | Medium | Medium | Use `termprofile` crate; test on Windows, macOS, Linux; provide manual override |
| **Color space conversion errors** | Low | High | Use `palette` crate for conversions; comprehensive unit tests; visual regression tests |
| **Performance degradation** | Low | Medium | Benchmark theme lookups; cache resolved styles; profile render loop |
| **Bevy integration complexity** | Medium | Medium | Prototype early; use Bevy's built-in color types; follow Bevy patterns |
| **Accessibility compliance gaps** | Medium | High | Automated WCAG testing; CVD simulation tests; user testing with colorblind users |

### Integration Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Save file format breakage** | Low | Critical | Extensive backward compatibility testing; serde defaults; migration tests |
| **Cross-frontend inconsistency** | Medium | Medium | Shared omega-core types; visual regression tests; design system documentation |
| **Config migration issues** | Low | Medium | Version field in config; graceful degradation; clear error messages |

### Project Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Scope creep** | Medium | Medium | Clear phase boundaries; defer advanced features; regular scope reviews |
| **Technical debt** | Low | Medium | Code review requirements; >80% test coverage; documentation requirements |
| **Dependency churn** | Low | Low | Pin dependency versions; regular dependency updates; minimal external deps |

### Risk Monitoring

- **Weekly:** Review open blockers and technical concerns
- **Per-Phase:** Risk assessment update before proceeding to next phase
- **Continuous:** CI monitors for contrast regressions and test failures

---

## 5. Definition of Done

### Project-Level Completion Criteria

The Colorful Omega project is considered complete when:

#### Functional Completion

- [ ] All Phase 1-4 requirements implemented and tested
- [ ] Phase 5 requirements implemented (or explicitly deferred)
- [ ] Both TUI and Bevy frontends display colors consistently
- [ ] All 3 built-in themes (classic, accessible, modern) functional
- [ ] User can create, load, and switch custom themes
- [ ] Theme hot-reload works in development mode
- [ ] `omega-theme` CLI tool validates themes and checks contrast

#### Quality Criteria

- [ ] Code coverage >80% for all new code
- [ ] All CI checks pass (format, lint, test, doc, contrast)
- [ ] No critical or high-severity bugs open
- [ ] Documentation complete (API docs, user guide, dev guide)
- [ ] Performance benchmarks meet targets (< 50ms theme load, < 5% overhead)

#### Accessibility Criteria

- [ ] WCAG 2.2 AA compliance verified for all built-in themes
- [ ] Colorblind-friendly themes tested and documented
- [ ] `NO_COLOR` support verified across all frontends
- [ ] Automated contrast checking in CI pipeline
- [ ] Accessibility documentation complete

#### Compatibility Criteria

- [ ] Existing save files load without errors
- [ ] Existing configs migrate gracefully
- [ ] 16-color terminal fallback works
- [ ] TrueColor terminals display full palette
- [ ] Windows, macOS, and Linux tested

#### User Experience Criteria

- [ ] Theme switching is intuitive and responsive
- [ ] Error messages are clear when themes fail to load
- [ ] Default theme loads automatically on first run
- [ ] Help text documents all color-related options

### Phase Completion Gates

Each phase must pass these gates before proceeding:

1. **Code Review:** All changes reviewed by code owner
2. **Testing:** Unit tests >80%, integration tests passing
3. **Documentation:** API docs and user-facing docs updated
4. **Acceptance:** Success criteria demonstrably met
5. **Sign-off:** Stakeholder approval (user + maintainer review)

---

## 6. Progress Tracking

### Current Status

| Phase | Status | Progress | Target | Completion Date |
|-------|--------|----------|--------|-----------------|
| Phase 1: Foundation | 🟢 Complete | 100% | Week 1-3 | 2026-02-12 |
| Phase 2: TUI Integration | 🟡 Planned | 0% | Week 3-6 | TBD |
| Phase 3: Bevy Integration | ⚪ Blocked | 0% | Week 5-8 | TBD |
| Phase 4: Customization | ⚪ Blocked | 0% | Week 8-12 | TBD |
| Phase 5: Advanced | ⚪ Blocked | 0% | Week 12-18 | TBD |

**Legend:** 🟢 Complete | 🟡 Planned/In Progress | 🔵 Not Started | ⚪ Blocked

### Requirements Coverage

| Category | Total | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|----------|-------|---------|---------|---------|---------|---------|
| Core Color | 5 | ✅ 5 | 0 | 0 | 0 | 0 |
| TUI | 5 | 0 | 5 | 0 | 0 | 0 |
| Bevy | 4 | 0 | 0 | 4 | 0 | 0 |
| Customization | 6 | 0 | 0 | 0 | 6 | 0 |
| Advanced | 4 | 0 | 0 | 0 | 0 | 4 |
| **Total** | **24** | **5** | **5** | **4** | **6** | **4** |

**Coverage:** 24/24 requirements mapped
**Phase 1 Complete:** All 5 core color requirements implemented

---

## 7. Dependencies & Prerequisites

### External Dependencies

| Dependency | Version | Purpose | Phase |
|------------|---------|---------|-------|
| termprofile | ^0.2 | Terminal capability detection | 1 |
| palette | ^0.7 | Color space conversions | 1 |
| toml | ^0.8 | Theme serialization | 1 |
| clap | ^4 | CLI argument parsing | 2 |
| notify | ^6.0 | File watching for hot-reload | 4 |
| insta | ^1.42 | Snapshot testing (dev) | 2 |

### Internal Dependencies

```
Phase 1: omega-core
    ↓
Phase 2: omega-tui (depends on Phase 1)
    ↓
Phase 3: omega-bevy (depends on Phase 1)
    ↓
Phase 4: omega-tools (depends on Phase 1, 2, 3)
    ↓
Phase 5: All crates
```

### Blockers

None currently identified. Phase 2 plans are ready for execution.

---

## 8. Appendix

### A. Theme Format Example

```toml
# omega-dark.toml - Example theme file
name = "Omega Dark"
author = "Omega Team"
description = "A dark, atmospheric theme for Omega"
version = "1.0.0"

# Base palette - define your colors here
[base]
red = { hex = "#e94560" }
green = { hex = "#0f3460" }
blue = { hex = "#16c79a" }
gold = { hex = "#f9a825" }
purple = { hex = "#7c4dff" }
cyan = { hex = "#00bcd4" }
white = { hex = "#e0e0e0" }
gray = { hex = "#9e9e9e" }
black = { hex = "#212121" }

# Semantic mappings - what colors mean
[semantic]
danger = { ref = "base.red" }
success = { ref = "base.green" }
info = { ref = "base.blue" }
warning = { ref = "base.gold" }
magic = { ref = "base.purple" }
water = { ref = "base.cyan" }
neutral = { ref = "base.gray" }

# Component-specific colors
[component]
player = { fg = "base.cyan", bg = "base.black" }
monster_hostile = { fg = "base.red", bg = "base.black" }
monster_neutral = { fg = "base.gray", bg = "base.black" }
item_common = { fg = "base.white", bg = "base.black" }
item_rare = { fg = "base.gold", bg = "base.black" }
item_legendary = { fg = "base.purple", bg = "base.black" }
wall = { fg = "base.gray", bg = "base.black" }
floor = { fg = "base.black", bg = "base.black" }

# UI colors
[ui]
health_high = { ref = "base.green" }
health_medium = { ref = "base.gold" }
health_low = { ref = "base.red" }
mana = { ref = "base.blue" }
highlight = { ref = "base.cyan" }
```

### B. ColorId Enum Design

```rust
/// Semantic color identifiers used throughout the game.
/// These abstract colors from concrete values, enabling theming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorId {
    // Item rarity levels
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,

    // Monster dispositions
    MonsterHostile,
    MonsterNeutral,
    MonsterFriendly,

    // Terrain types
    TerrainWall,
    TerrainFloor,
    TerrainWater,
    TerrainLava,
    TerrainDoor,
    TerrainStairs,

    // Environmental
    Fire,
    Ice,
    Poison,
    Magic,

    // UI elements
    UiHealthHigh,
    UiHealthMedium,
    UiHealthLow,
    UiMana,
    UiStamina,
    UiHighlight,
    UiText,
    UiTextDim,

    // Special
    Player,
    Cursor,
    Selection,
}
```

### C. Terminal Capability Matrix

| Terminal | TrueColor | 256-Color | 16-Color | Detection Method |
|----------|-----------|-----------|----------|------------------|
| iTerm2 (macOS) | ✓ | ✓ | ✓ | COLORTERM=truecolor |
| Windows Terminal | ✓ | ✓ | ✓ | COLORTERM=truecolor |
| Alacritty | ✓ | ✓ | ✓ | COLORTERM=truecolor |
| GNOME Terminal | ✓ | ✓ | ✓ | COLORTERM=truecolor |
| tmux | ✓* | ✓ | ✓ | TERM=screen-256color |
| Linux console | ✗ | ✗ | ✓ | TERM=linux |
| PuTTY | ✗ | ✓ | ✓ | TERM=xterm-256color |
| cmd.exe (legacy) | ✗ | ✗ | ✓ | TERM not set |

*Depends on underlying terminal support

---

## Document Control

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-02-12 | Claude | Initial roadmap based on research synthesis |
| 1.1 | 2026-02-12 | Claude | Phase 2 plans added (3 plans, 3 waves) |

**Next Review:** After Phase 2 completion

**Distribution:** Omega development team, stakeholders
