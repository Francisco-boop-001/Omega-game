//! Color identifier types for the Omega game engine.
//!
//! This module defines the `ColorId` enum and its nested variants, which provide
//! semantic color identifiers for all game entities, UI elements, effects, and
//! environment lighting. These identifiers are resolved to concrete colors through
//! the theme system.
//!
//! # Design Philosophy
//!
//! Colors are identified by semantic meaning rather than raw color values. This
//! allows themes to provide different color schemes while maintaining consistent
//! visual language (e.g., "hostile" monsters are always distinguishable from
//! "friendly" ones, regardless of the specific theme).
//!
//! # Example
//!
//! ```rust
//! use omega_core::color::{ColorId, EntityColorId, MonsterColorId};
//!
//! let player_color = ColorId::Entity(EntityColorId::Player);
//! let monster_color = ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead));
//! ```

use serde::{Deserialize, Serialize};

/// Top-level color identifier enum.
///
/// Organizes all game colors into four semantic categories:
/// - `Entity`: Colors for game entities (player, monsters, items, terrain)
/// - `Ui`: Colors for user interface elements (health bars, text, messages)
/// - `Effect`: Colors for transient visual effects (spells, particles)
/// - `Environment`: Colors for lighting and atmospheric effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorId {
    /// Colors for game entities visible on the map.
    Entity(EntityColorId),
    /// Colors for UI elements like status bars, text, and highlights.
    Ui(UiColorId),
    /// Colors for spell effects, particles, and transient visuals.
    Effect(EffectColorId),
    /// Colors for lighting and environmental effects.
    Environment(EnvironmentColorId),
}

/// Entity-specific color identifiers.
///
/// Categorizes entities by their semantic role in the game:
/// - Player character (always visible)
/// - Monsters (categorized by behavior and taxonomy)
/// - Items (categorized by rarity tier)
/// - Terrain (categorized by material and features)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityColorId {
    /// The player character - always rendered with maximum visibility.
    Player,
    /// Monsters categorized by behavior (hostile/neutral/friendly) and taxonomy.
    Monster(MonsterColorId),
    /// Items categorized by rarity tier (common to legendary).
    Item(ItemRarityColorId),
    /// Terrain features categorized by material type.
    Terrain(TerrainColorId),
}

/// Monster color categories.
///
/// Organized by hostility level and monster taxonomy:
/// - Hostile variants are grouped by their fundamental nature (undead, beasts, etc.)
/// - Neutral and friendly provide non-combat interaction indicators
///
/// This categorization helps players quickly assess threats and make tactical decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonsterColorId {
    /// Undead monsters (skeletons, zombies, ghosts) - unnatural abominations.
    HostileUndead,
    /// Beast monsters (wolves, bears, giant insects) - natural predators.
    HostileBeast,
    /// Humanoid monsters (orcs, goblins, bandits) - intelligent adversaries.
    HostileHumanoid,
    /// Magical monsters (elementals, summoned creatures) - arcane threats.
    HostileMagical,
    /// Construct monsters (golems, animated objects) - artificial guardians.
    HostileConstruct,
    /// Dragon monsters - apex predators requiring special tactics.
    HostileDragon,
    /// Neutral creatures that don't attack unless provoked.
    Neutral,
    /// Friendly creatures that may assist the player.
    Friendly,
}

/// Item rarity color tiers.
///
/// Five-tier system with color + intensity progression:
/// - Common: Gray, dim (ordinary items)
/// - Uncommon: Green, moderate brightness (slightly special)
/// - Rare: Blue, bright (notable finds)
/// - Epic: Purple, vibrant (exceptional treasures)
/// - Legendary: Gold, glowing intensity (artifacts and unique items)
///
/// The progression creates an intuitive visual hierarchy that helps players
/// quickly identify valuable loot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarityColorId {
    /// Ordinary items found everywhere - gray, dim appearance.
    Common,
    /// Slightly special items - green with moderate brightness.
    Uncommon,
    /// Notable finds - blue with bright appearance.
    Rare,
    /// Exceptional treasures - purple with vibrant color.
    Epic,
    /// Artifacts and unique items - gold with glowing intensity.
    Legendary,
}

/// Terrain color identifiers.
///
/// Organized by material type and structural purpose:
/// - Walls: Different materials (stone, wood, metal, brick)
/// - Floors: Different surfaces (stone, wood, dirt, grass)
/// - Features: Interactive elements (water, lava, doors, stairs)
///
/// Material-based coloring helps players understand the dungeon environment
/// and navigate effectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainColorId {
    // Walls by material
    /// Stone walls - dungeon boundaries and structures.
    WallStone,
    /// Wooden walls - lighter structures, often flammable.
    WallWood,
    /// Metal walls - reinforced areas, industrial or magical.
    WallMetal,
    /// Brick walls - constructed areas, civilized regions.
    WallBrick,

    // Floors by material
    /// Stone floors - typical dungeon walking surface.
    FloorStone,
    /// Wooden floors - buildings and constructed areas.
    FloorWood,
    /// Dirt floors - natural caves and outdoor areas.
    FloorDirt,
    /// Grass floors - outdoor and wilderness areas.
    FloorGrass,

    // Features
    /// Water - pools, rivers, and other water features.
    Water,
    /// Lava - dangerous terrain, typically in volcanic areas.
    Lava,
    /// Doors - entryways that may be opened or closed.
    Door,
    /// Stairs leading upward - exit to previous level.
    StairsUp,
    /// Stairs leading downward - descent to deeper levels.
    StairsDown,
}

/// UI element color identifiers.
///
/// Covers all interface elements separate from the game map:
/// - Status bars: Health, mana, stamina with gradient indicators
/// - Interactive elements: Highlights, selections, cursor
/// - Text: Various emphasis levels for readability
/// - Messages: Severity-based coloring for game feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UiColorId {
    // Status bars with health gradient
    /// Full health - vibrant color indicating good condition.
    HealthHigh,
    /// Moderate health - warning color for damaged state.
    HealthMedium,
    /// Critical health - urgent color requiring immediate attention.
    HealthLow,
    /// Mana/magic points - mystical energy indicator.
    Mana,
    /// Stamina/endurance - physical exertion indicator.
    Stamina,
    /// Experience progress - advancement and growth indicator.
    Experience,

    // Interactive elements
    /// Highlighted elements - drawing attention to interactive items.
    Highlight,
    /// Selected elements - current player choice or focus.
    Selection,
    /// Cursor position - player input location indicator.
    Cursor,

    // Text styling
    /// Default text color - standard readable text.
    TextDefault,
    /// Dimmed text - secondary or less important information.
    TextDim,
    /// Bold/emphasized text - important information requiring attention.
    TextBold,

    // Message severity levels
    /// Informational messages - neutral game feedback.
    MessageInfo,
    /// Warning messages - cautionary alerts about potential issues.
    MessageWarning,
    /// Danger/critical messages - urgent threats or failures.
    MessageDanger,
    /// Success messages - positive outcomes and achievements.
    MessageSuccess,
}

/// Effect and particle color identifiers.
///
/// Colors for spells, combat effects, and transient visual phenomena:
/// - Elemental: Natural forces (fire, ice, lightning, poison, acid)
/// - Magical: Arcane energies (arcane, holy, dark magic)
/// - Physical: Combat impacts (blood, impacts, shields)
///
/// These colors help players identify spell types and danger sources quickly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectColorId {
    // Elemental effects
    /// Fire - burning, heat, and flame-based effects.
    Fire,
    /// Ice - freezing, cold, and frost-based effects.
    Ice,
    /// Lightning - electricity, thunder, and storm effects.
    Lightning,
    /// Poison - toxic, venomous, and biological hazards.
    Poison,
    /// Acid - corrosive, dissolving, and chemical effects.
    Acid,

    // Magical effects
    /// Arcane magic - general spell energy and wizardry.
    MagicArcane,
    /// Holy magic - divine, blessed, and sacred energy.
    MagicHoly,
    /// Dark magic - unholy, necromantic, and forbidden energy.
    MagicDark,

    // Physical effects
    /// Blood - injury, gore, and vitality effects.
    Blood,
    /// Impact - blunt force, collision, and physical strikes.
    Impact,
    /// Shield - protective barriers and defensive effects.
    Shield,
}

/// Environment and lighting color identifiers.
///
/// Colors for atmospheric and lighting effects:
/// - Light sources: Various illumination types with different qualities
/// - Darkness: Absence of light, shadow effects
/// - Weather: Fog and atmospheric conditions
///
/// These colors create mood and atmosphere while affecting gameplay visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentColorId {
    /// Torch light - warm, flickering illumination with limited range.
    LightTorch,
    /// Lantern light - steady, reliable artificial illumination.
    LightLantern,
    /// Magical light - supernatural illumination, often colored.
    LightMagic,
    /// Darkness - areas with no illumination.
    Darkness,
    /// Fog - reduced visibility atmospheric condition.
    Fog,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_id_variants_exist() {
        // Verify all top-level variants can be constructed
        let _entity = ColorId::Entity(EntityColorId::Player);
        let _ui = ColorId::Ui(UiColorId::HealthHigh);
        let _effect = ColorId::Effect(EffectColorId::Fire);
        let _environment = ColorId::Environment(EnvironmentColorId::LightTorch);
    }

    #[test]
    fn entity_color_variants_exist() {
        // Verify entity sub-variants
        let _player = EntityColorId::Player;
        let _monster = EntityColorId::Monster(MonsterColorId::HostileUndead);
        let _item = EntityColorId::Item(ItemRarityColorId::Legendary);
        let _terrain = EntityColorId::Terrain(TerrainColorId::WallStone);
    }

    #[test]
    fn monster_color_categories_exist() {
        // Verify all monster categories
        let _undead = MonsterColorId::HostileUndead;
        let _beast = MonsterColorId::HostileBeast;
        let _humanoid = MonsterColorId::HostileHumanoid;
        let _magical = MonsterColorId::HostileMagical;
        let _construct = MonsterColorId::HostileConstruct;
        let _dragon = MonsterColorId::HostileDragon;
        let _neutral = MonsterColorId::Neutral;
        let _friendly = MonsterColorId::Friendly;
    }

    #[test]
    fn item_rarity_tiers_exist() {
        // Verify all rarity tiers
        let _common = ItemRarityColorId::Common;
        let _uncommon = ItemRarityColorId::Uncommon;
        let _rare = ItemRarityColorId::Rare;
        let _epic = ItemRarityColorId::Epic;
        let _legendary = ItemRarityColorId::Legendary;
    }

    #[test]
    fn terrain_variants_exist() {
        // Verify wall variants
        let _wall_stone = TerrainColorId::WallStone;
        let _wall_wood = TerrainColorId::WallWood;
        let _wall_metal = TerrainColorId::WallMetal;
        let _wall_brick = TerrainColorId::WallBrick;

        // Verify floor variants
        let _floor_stone = TerrainColorId::FloorStone;
        let _floor_wood = TerrainColorId::FloorWood;
        let _floor_dirt = TerrainColorId::FloorDirt;
        let _floor_grass = TerrainColorId::FloorGrass;

        // Verify feature variants
        let _water = TerrainColorId::Water;
        let _lava = TerrainColorId::Lava;
        let _door = TerrainColorId::Door;
        let _stairs_up = TerrainColorId::StairsUp;
        let _stairs_down = TerrainColorId::StairsDown;
    }

    #[test]
    fn ui_variants_exist() {
        // Verify status bar variants
        let _health_high = UiColorId::HealthHigh;
        let _health_medium = UiColorId::HealthMedium;
        let _health_low = UiColorId::HealthLow;
        let _mana = UiColorId::Mana;
        let _stamina = UiColorId::Stamina;
        let _experience = UiColorId::Experience;

        // Verify interactive element variants
        let _highlight = UiColorId::Highlight;
        let _selection = UiColorId::Selection;
        let _cursor = UiColorId::Cursor;

        // Verify text variants
        let _text_default = UiColorId::TextDefault;
        let _text_dim = UiColorId::TextDim;
        let _text_bold = UiColorId::TextBold;

        // Verify message variants
        let _message_info = UiColorId::MessageInfo;
        let _message_warning = UiColorId::MessageWarning;
        let _message_danger = UiColorId::MessageDanger;
        let _message_success = UiColorId::MessageSuccess;
    }

    #[test]
    fn effect_variants_exist() {
        // Verify elemental variants
        let _fire = EffectColorId::Fire;
        let _ice = EffectColorId::Ice;
        let _lightning = EffectColorId::Lightning;
        let _poison = EffectColorId::Poison;
        let _acid = EffectColorId::Acid;

        // Verify magical variants
        let _magic_arcane = EffectColorId::MagicArcane;
        let _magic_holy = EffectColorId::MagicHoly;
        let _magic_dark = EffectColorId::MagicDark;

        // Verify physical variants
        let _blood = EffectColorId::Blood;
        let _impact = EffectColorId::Impact;
        let _shield = EffectColorId::Shield;
    }

    #[test]
    fn environment_variants_exist() {
        let _light_torch = EnvironmentColorId::LightTorch;
        let _light_lantern = EnvironmentColorId::LightLantern;
        let _light_magic = EnvironmentColorId::LightMagic;
        let _darkness = EnvironmentColorId::Darkness;
        let _fog = EnvironmentColorId::Fog;
    }

    #[test]
    fn color_id_equality() {
        assert_eq!(
            ColorId::Entity(EntityColorId::Player),
            ColorId::Entity(EntityColorId::Player)
        );
        assert_ne!(
            ColorId::Entity(EntityColorId::Player),
            ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileUndead))
        );
    }

    #[test]
    fn serde_roundtrip() {
        // Test that all variants serialize and deserialize correctly
        let variants = vec![
            ColorId::Entity(EntityColorId::Player),
            ColorId::Entity(EntityColorId::Monster(MonsterColorId::HostileDragon)),
            ColorId::Entity(EntityColorId::Item(ItemRarityColorId::Legendary)),
            ColorId::Entity(EntityColorId::Terrain(TerrainColorId::Lava)),
            ColorId::Ui(UiColorId::HealthLow),
            ColorId::Effect(EffectColorId::MagicArcane),
            ColorId::Environment(EnvironmentColorId::Darkness),
        ];

        for original in variants {
            let serialized = serde_json::to_string(&original).expect("Failed to serialize");
            let deserialized: ColorId =
                serde_json::from_str(&serialized).expect("Failed to deserialize");
            assert_eq!(
                original, deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    #[test]
    fn hash_consistency() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(ColorId::Ui(UiColorId::HealthHigh), "high");
        map.insert(ColorId::Ui(UiColorId::HealthLow), "low");

        assert_eq!(map.get(&ColorId::Ui(UiColorId::HealthHigh)), Some(&"high"));
        assert_eq!(map.get(&ColorId::Ui(UiColorId::HealthLow)), Some(&"low"));
    }
}
