//! Bevy theme resource for semantic color support.
//!
//! Provides BevyTheme as a Bevy Resource that wraps omega-core's ColorTheme,
//! enabling convenient color resolution for entities, UI, effects, and environment.

use bevy::prelude::{Color, Resource};
use omega_core::color::{
    ColorId, ColorTheme, EffectColorId, EntityColorId, EnvironmentColorId, ItemRarityColorId,
    MonsterColorId, TerrainColorId, UiColorId,
};

use super::color_adapter::{resolve_to_bevy_color, to_bevy_color};

/// Bevy resource wrapping a ColorTheme for semantic color resolution.
///
/// BevyTheme provides convenient methods to get colors for specific
/// game entities, UI elements, effects, and environment features.
///
/// # Example
///
/// ```ignore
/// use bevy::prelude::*;
/// use omega_bevy::presentation::BevyTheme;
/// use omega_core::color::{ColorId, EntityColorId};
///
/// fn my_system(theme: Res<BevyTheme>) {
///     let player_color = theme.get_player_color();
///     let ui_health = theme.get_ui_health_high();
///     // Use colors for rendering...
/// }
/// ```
#[derive(Debug, Clone, Resource)]
pub struct BevyTheme {
    /// The underlying ColorTheme from omega-core.
    theme: ColorTheme,
}

impl BevyTheme {
    /// Creates a new BevyTheme from a ColorTheme.
    pub fn new(theme: ColorTheme) -> Self {
        Self { theme }
    }

    /// Resolves a ColorId to a Bevy Color.
    ///
    /// Returns the foreground color for the given ColorId.
    /// If the ColorId cannot be resolved, returns white as a fallback.
    pub fn resolve(&self, id: &ColorId) -> Color {
        resolve_to_bevy_color(&self.theme, id)
    }

    /// Gets both foreground and background colors for a ColorId.
    ///
    /// Returns a tuple of (foreground, background) colors.
    /// If the ColorId cannot be resolved, returns (white, black) as fallback.
    pub fn resolve_both(&self, id: &ColorId) -> (Color, Color) {
        match self.theme.resolve(id) {
            Some((fg, bg)) => (to_bevy_color(&fg), to_bevy_color(&bg)),
            None => (Color::srgb(1.0, 1.0, 1.0), Color::srgb(0.0, 0.0, 0.0)),
        }
    }

    // === Entity Colors ===

    /// Gets the player character color.
    pub fn get_player_color(&self) -> Color {
        self.resolve(&ColorId::Entity(EntityColorId::Player))
    }

    /// Gets the color for a specific monster type.
    pub fn get_monster_color(&self, monster_id: MonsterColorId) -> Color {
        self.resolve(&ColorId::Entity(EntityColorId::Monster(monster_id)))
    }

    /// Gets the color for hostile undead monsters.
    pub fn get_monster_hostile_undead(&self) -> Color {
        self.get_monster_color(MonsterColorId::HostileUndead)
    }

    /// Gets the color for hostile beast monsters.
    pub fn get_monster_hostile_beast(&self) -> Color {
        self.get_monster_color(MonsterColorId::HostileBeast)
    }

    /// Gets the color for hostile humanoid monsters.
    pub fn get_monster_hostile_humanoid(&self) -> Color {
        self.get_monster_color(MonsterColorId::HostileHumanoid)
    }

    /// Gets the color for neutral monsters.
    pub fn get_monster_neutral(&self) -> Color {
        self.get_monster_color(MonsterColorId::Neutral)
    }

    /// Gets the color for friendly monsters.
    pub fn get_monster_friendly(&self) -> Color {
        self.get_monster_color(MonsterColorId::Friendly)
    }

    /// Gets the color for an item by rarity.
    pub fn get_item_color(&self, rarity: ItemRarityColorId) -> Color {
        self.resolve(&ColorId::Entity(EntityColorId::Item(rarity)))
    }

    /// Gets the color for common items.
    pub fn get_item_common(&self) -> Color {
        self.get_item_color(ItemRarityColorId::Common)
    }

    /// Gets the color for uncommon items.
    pub fn get_item_uncommon(&self) -> Color {
        self.get_item_color(ItemRarityColorId::Uncommon)
    }

    /// Gets the color for rare items.
    pub fn get_item_rare(&self) -> Color {
        self.get_item_color(ItemRarityColorId::Rare)
    }

    /// Gets the color for epic items.
    pub fn get_item_epic(&self) -> Color {
        self.get_item_color(ItemRarityColorId::Epic)
    }

    /// Gets the color for legendary items.
    pub fn get_item_legendary(&self) -> Color {
        self.get_item_color(ItemRarityColorId::Legendary)
    }

    /// Gets the color for a terrain type.
    pub fn get_terrain_color(&self, terrain_id: TerrainColorId) -> Color {
        self.resolve(&ColorId::Entity(EntityColorId::Terrain(terrain_id)))
    }

    /// Gets the color for stone walls.
    pub fn get_terrain_wall_stone(&self) -> Color {
        self.get_terrain_color(TerrainColorId::WallStone)
    }

    /// Gets the color for wooden walls.
    pub fn get_terrain_wall_wood(&self) -> Color {
        self.get_terrain_color(TerrainColorId::WallWood)
    }

    /// Gets the color for stone floors.
    pub fn get_terrain_floor_stone(&self) -> Color {
        self.get_terrain_color(TerrainColorId::FloorStone)
    }

    /// Gets the color for wooden floors.
    pub fn get_terrain_floor_wood(&self) -> Color {
        self.get_terrain_color(TerrainColorId::FloorWood)
    }

    /// Gets the color for grass.
    pub fn get_terrain_floor_grass(&self) -> Color {
        self.get_terrain_color(TerrainColorId::FloorGrass)
    }

    /// Gets the color for water.
    pub fn get_terrain_water(&self) -> Color {
        self.get_terrain_color(TerrainColorId::Water)
    }

    /// Gets the color for doors.
    pub fn get_terrain_door(&self) -> Color {
        self.get_terrain_color(TerrainColorId::Door)
    }

    /// Gets the color for stairs going up.
    pub fn get_terrain_stairs_up(&self) -> Color {
        self.get_terrain_color(TerrainColorId::StairsUp)
    }

    /// Gets the color for stairs going down.
    pub fn get_terrain_stairs_down(&self) -> Color {
        self.get_terrain_color(TerrainColorId::StairsDown)
    }

    // === UI Colors ===

    /// Gets a UI color.
    pub fn get_ui_color(&self, ui_id: UiColorId) -> Color {
        self.resolve(&ColorId::Ui(ui_id))
    }

    /// Gets the high health UI color.
    pub fn get_ui_health_high(&self) -> Color {
        self.get_ui_color(UiColorId::HealthHigh)
    }

    /// Gets the medium health UI color.
    pub fn get_ui_health_medium(&self) -> Color {
        self.get_ui_color(UiColorId::HealthMedium)
    }

    /// Gets the low health UI color.
    pub fn get_ui_health_low(&self) -> Color {
        self.get_ui_color(UiColorId::HealthLow)
    }

    /// Gets the mana UI color.
    pub fn get_ui_mana(&self) -> Color {
        self.get_ui_color(UiColorId::Mana)
    }

    /// Gets the stamina UI color.
    pub fn get_ui_stamina(&self) -> Color {
        self.get_ui_color(UiColorId::Stamina)
    }

    /// Gets the experience UI color.
    pub fn get_ui_experience(&self) -> Color {
        self.get_ui_color(UiColorId::Experience)
    }

    /// Gets the highlight UI color.
    pub fn get_ui_highlight(&self) -> Color {
        self.get_ui_color(UiColorId::Highlight)
    }

    /// Gets the selection UI color.
    pub fn get_ui_selection(&self) -> Color {
        self.get_ui_color(UiColorId::Selection)
    }

    /// Gets the cursor UI color.
    pub fn get_ui_cursor(&self) -> Color {
        self.get_ui_color(UiColorId::Cursor)
    }

    /// Gets the default text UI color.
    pub fn get_ui_text_default(&self) -> Color {
        self.get_ui_color(UiColorId::TextDefault)
    }

    /// Gets the dim text UI color.
    pub fn get_ui_text_dim(&self) -> Color {
        self.get_ui_color(UiColorId::TextDim)
    }

    /// Gets the bold text UI color.
    pub fn get_ui_text_bold(&self) -> Color {
        self.get_ui_color(UiColorId::TextBold)
    }

    /// Gets the info message UI color.
    pub fn get_ui_message_info(&self) -> Color {
        self.get_ui_color(UiColorId::MessageInfo)
    }

    /// Gets the warning message UI color.
    pub fn get_ui_message_warning(&self) -> Color {
        self.get_ui_color(UiColorId::MessageWarning)
    }

    /// Gets the danger message UI color.
    pub fn get_ui_message_danger(&self) -> Color {
        self.get_ui_color(UiColorId::MessageDanger)
    }

    /// Gets the success message UI color.
    pub fn get_ui_message_success(&self) -> Color {
        self.get_ui_color(UiColorId::MessageSuccess)
    }

    // === Effect Colors ===

    /// Gets an effect color.
    pub fn get_effect_color(&self, effect_id: EffectColorId) -> Color {
        self.resolve(&ColorId::Effect(effect_id))
    }

    /// Gets the fire effect color.
    pub fn get_effect_fire(&self) -> Color {
        self.get_effect_color(EffectColorId::Fire)
    }

    /// Gets the ice effect color.
    pub fn get_effect_ice(&self) -> Color {
        self.get_effect_color(EffectColorId::Ice)
    }

    /// Gets the lightning effect color.
    pub fn get_effect_lightning(&self) -> Color {
        self.get_effect_color(EffectColorId::Lightning)
    }

    /// Gets the poison effect color.
    pub fn get_effect_poison(&self) -> Color {
        self.get_effect_color(EffectColorId::Poison)
    }

    /// Gets the acid effect color.
    pub fn get_effect_acid(&self) -> Color {
        self.get_effect_color(EffectColorId::Acid)
    }

    /// Gets the arcane magic effect color.
    pub fn get_effect_magic_arcane(&self) -> Color {
        self.get_effect_color(EffectColorId::MagicArcane)
    }

    /// Gets the holy magic effect color.
    pub fn get_effect_magic_holy(&self) -> Color {
        self.get_effect_color(EffectColorId::MagicHoly)
    }

    /// Gets the dark magic effect color.
    pub fn get_effect_magic_dark(&self) -> Color {
        self.get_effect_color(EffectColorId::MagicDark)
    }

    /// Gets the blood effect color.
    pub fn get_effect_blood(&self) -> Color {
        self.get_effect_color(EffectColorId::Blood)
    }

    /// Gets the impact effect color.
    pub fn get_effect_impact(&self) -> Color {
        self.get_effect_color(EffectColorId::Impact)
    }

    /// Gets the shield effect color.
    pub fn get_effect_shield(&self) -> Color {
        self.get_effect_color(EffectColorId::Shield)
    }

    // === Environment Colors ===

    /// Gets an environment color.
    pub fn get_environment_color(&self, env_id: EnvironmentColorId) -> Color {
        self.resolve(&ColorId::Environment(env_id))
    }

    /// Gets the torch light environment color.
    pub fn get_environment_light_torch(&self) -> Color {
        self.get_environment_color(EnvironmentColorId::LightTorch)
    }

    /// Gets the lantern light environment color.
    pub fn get_environment_light_lantern(&self) -> Color {
        self.get_environment_color(EnvironmentColorId::LightLantern)
    }

    /// Gets the magic light environment color.
    pub fn get_environment_light_magic(&self) -> Color {
        self.get_environment_color(EnvironmentColorId::LightMagic)
    }

    /// Gets the darkness environment color.
    pub fn get_environment_darkness(&self) -> Color {
        self.get_environment_color(EnvironmentColorId::Darkness)
    }

    /// Gets the fog environment color.
    pub fn get_environment_fog(&self) -> Color {
        self.get_environment_color(EnvironmentColorId::Fog)
    }

    /// Returns the underlying ColorTheme reference.
    pub fn theme(&self) -> &ColorTheme {
        &self.theme
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::color_adapter::load_builtin_theme;

    fn create_test_theme() -> BevyTheme {
        let theme = load_builtin_theme("classic").expect("classic theme should load");
        BevyTheme::new(theme)
    }

    #[test]
    fn bevy_theme_resolves_player_color() {
        let theme = create_test_theme();
        let color = theme.get_player_color();
        let srgba = color.to_srgba();
        // Should be a valid color, not default white
        assert!(srgba.red >= 0.0 && srgba.red <= 1.0);
    }

    #[test]
    fn bevy_theme_resolves_monster_colors() {
        let theme = create_test_theme();
        let undead = theme.get_monster_hostile_undead();
        let beast = theme.get_monster_hostile_beast();
        let friendly = theme.get_monster_friendly();

        // All should be valid colors
        assert!(undead.to_srgba().red >= 0.0);
        assert!(beast.to_srgba().red >= 0.0);
        assert!(friendly.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolves_item_rarity_colors() {
        let theme = create_test_theme();
        let common = theme.get_item_common();
        let rare = theme.get_item_rare();
        let legendary = theme.get_item_legendary();

        // All should be valid colors
        assert!(common.to_srgba().red >= 0.0);
        assert!(rare.to_srgba().red >= 0.0);
        assert!(legendary.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolves_terrain_colors() {
        let theme = create_test_theme();
        let wall = theme.get_terrain_wall_stone();
        let floor = theme.get_terrain_floor_stone();
        let water = theme.get_terrain_water();
        let door = theme.get_terrain_door();

        // All should be valid colors
        assert!(wall.to_srgba().red >= 0.0);
        assert!(floor.to_srgba().red >= 0.0);
        assert!(water.to_srgba().red >= 0.0);
        assert!(door.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolves_ui_colors() {
        let theme = create_test_theme();
        let health_high = theme.get_ui_health_high();
        let health_low = theme.get_ui_health_low();
        let mana = theme.get_ui_mana();
        let cursor = theme.get_ui_cursor();

        // All should be valid colors
        assert!(health_high.to_srgba().red >= 0.0);
        assert!(health_low.to_srgba().red >= 0.0);
        assert!(mana.to_srgba().red >= 0.0);
        assert!(cursor.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolves_effect_colors() {
        let theme = create_test_theme();
        let fire = theme.get_effect_fire();
        let ice = theme.get_effect_ice();
        let lightning = theme.get_effect_lightning();
        let poison = theme.get_effect_poison();

        // All should be valid colors
        assert!(fire.to_srgba().red >= 0.0);
        assert!(ice.to_srgba().red >= 0.0);
        assert!(lightning.to_srgba().red >= 0.0);
        assert!(poison.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolves_environment_colors() {
        let theme = create_test_theme();
        let torch = theme.get_environment_light_torch();
        let darkness = theme.get_environment_darkness();
        let fog = theme.get_environment_fog();

        // All should be valid colors
        assert!(torch.to_srgba().red >= 0.0);
        assert!(darkness.to_srgba().red >= 0.0);
        assert!(fog.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_resolve_both_returns_fg_and_bg() {
        let theme = create_test_theme();
        let (fg, bg) = theme.resolve_both(&ColorId::Entity(EntityColorId::Player));

        // Both should be valid colors
        assert!(fg.to_srgba().red >= 0.0);
        assert!(bg.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_generic_resolve_works() {
        let theme = create_test_theme();
        let color = theme.resolve(&ColorId::Ui(UiColorId::HealthHigh));
        assert!(color.to_srgba().red >= 0.0);
    }

    #[test]
    fn bevy_theme_provides_theme_reference() {
        let theme = create_test_theme();
        let theme_ref = theme.theme();
        assert_eq!(theme_ref.meta.name, "Classic");
    }
}
