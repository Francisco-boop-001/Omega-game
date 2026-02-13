# Omega Theme System

Omega uses a semantic, three-tier color system to ensure consistent visual language across both TUI (terminal) and GUI (Bevy) frontends.

## 1. Architecture

The theme system is organized into three hierarchical tiers:

### Tier 1: Base Palette (`[base]`)
Defines the raw color values available in the theme.
- **Example:** `red = "#FF0000"`, `gray = "#808080"`

### Tier 2: Semantic Mappings (`[semantic]`)
Maps base colors to game-wide meanings.
- **Keys:** `danger`, `success`, `info`, `warning`, `magic`, `neutral`.
- **Reference:** `{ ref = "base.red" }`

### Tier 3: Usage Categories (`[entity]`, `[ui]`, `[effect]`)
Maps specific game entities or UI elements to semantic keys or base colors.
- **Entity:** `player`, `monster.hostile_undead`, `terrain.wall_stone`.
- **UI:** `health_high`, `mana`, `text_default`.
- **Effect:** `fire`, `ice`, `arcane`.

## 2. Creating a Custom Theme

1. Create a new `.toml` file in your user theme directory:
   - **Linux:** `~/.config/omega/themes/my-theme.toml`
   - **macOS:** `~/Library/Application Support/omega/themes/my-theme.toml`
   - **Windows:** `%AppData%\omega	hemes\my-theme.toml`

2. Add the required metadata and sections. You can use `crates/omega-content/themes/classic.toml` as a template.

3. Validate your theme using the `omega-theme` tool:
   ```bash
   cargo run --bin omega-theme validate path/to/my-theme.toml
   ```

## 3. Developer Tooling

### omega-theme CLI
- `validate <PATH>`: Deep semantic validation.
- `check-contrast <PATH>`: Audits foreground/background contrast ratios against WCAG 2.2.
- `export --format alacritty <PATH>`: Generates terminal colors.
- `preview <PATH>`: Visualizes the palette in your terminal.

### Bevy Theme Editor
While running the Bevy frontend, press **F6** to open the Theme Editor. You can tweak colors in real-time to preview them in the dungeon.

## 4. Advanced Features

### Animations
Themes can include animations for semantic keys.
- **Flash:** Alternates between two colors (e.g., low health).
- **Pulse:** Smoothly transitions between colors (e.g., UI highlights).

### Contextual Theming
Omega automatically switches themes based on the game environment (City, Dungeon, Abyss, etc.).

### Procedural Colors
Developers can use the `ProceduralPalette` utility to generate infinite, visually distinct colors for rare items or special effects, ensuring they maintain proper contrast and distinctness.

## 5. Hot-Reloading
Omega automatically monitors the user theme directory. If you modify and save a `.toml` file while the game is running, the changes will be applied instantly (in Bevy) or upon next refresh (in TUI).
