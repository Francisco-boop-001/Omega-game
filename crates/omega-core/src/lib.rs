use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub mod color;
pub mod core;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapBounds {
    pub width: i32,
    pub height: i32,
}

impl MapBounds {
    pub fn contains(self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn offset(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self { x: self.x, y: self.y - 1 },
            Direction::South => Self { x: self.x, y: self.y + 1 },
            Direction::East => Self { x: self.x + 1, y: self.y },
            Direction::West => Self { x: self.x - 1, y: self.y },
        }
    }

    fn manhattan_distance(self, other: Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack_min: i32,
    pub attack_max: i32,
    pub defense: i32,
}

impl Stats {
    fn apply_damage(&mut self, raw_damage: i32) -> i32 {
        let applied = raw_damage.max(0).min(self.hp.max(0));
        self.hp -= applied;
        applied
    }

    fn is_alive(self) -> bool {
        self.hp > 0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ItemFamily {
    #[default]
    Unknown,
    Thing,
    Food,
    Scroll,
    Potion,
    Weapon,
    Armor,
    Shield,
    Cloak,
    Boots,
    Ring,
    Stick,
    Artifact,
    Cash,
    Corpse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Item {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub legacy_id: i32,
    #[serde(default)]
    pub family: ItemFamily,
    #[serde(default)]
    pub usef: String,
    #[serde(default)]
    pub item_type: String,
    #[serde(default)]
    pub weight: i32,
    #[serde(default)]
    pub plus: i32,
    #[serde(default)]
    pub charge: i32,
    #[serde(default)]
    pub dmg: i32,
    #[serde(default)]
    pub hit: i32,
    #[serde(default)]
    pub aux: i32,
    #[serde(default)]
    pub number: i32,
    #[serde(default)]
    pub fragility: i32,
    #[serde(default)]
    pub basevalue: i64,
    #[serde(default)]
    pub known: bool,
    #[serde(default)]
    pub used: bool,
    #[serde(default)]
    pub blessing: i32,
    #[serde(default)]
    pub level: u8,
    #[serde(default)]
    pub uniqueness: String,
    #[serde(default)]
    pub objchar: String,
    #[serde(default)]
    pub objstr: String,
    #[serde(default)]
    pub truename: String,
    #[serde(default)]
    pub cursestr: String,
}

impl Item {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self { id, name: name.into(), ..Self::default() }
    }

    fn basic(id: u32, name: impl Into<String>) -> Self {
        Self::new(id, name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EquipmentSlots {
    #[serde(default)]
    pub up_in_air: Option<u32>,
    #[serde(default)]
    pub ready_hand: Option<u32>,
    #[serde(default)]
    pub weapon_hand: Option<u32>,
    #[serde(default)]
    pub left_shoulder: Option<u32>,
    #[serde(default)]
    pub right_shoulder: Option<u32>,
    #[serde(default)]
    pub belt_1: Option<u32>,
    #[serde(default)]
    pub belt_2: Option<u32>,
    #[serde(default)]
    pub belt_3: Option<u32>,
    #[serde(default)]
    pub shield: Option<u32>,
    #[serde(default)]
    pub armor: Option<u32>,
    #[serde(default)]
    pub boots: Option<u32>,
    #[serde(default)]
    pub cloak: Option<u32>,
    #[serde(default)]
    pub ring_1: Option<u32>,
    #[serde(default)]
    pub ring_2: Option<u32>,
    #[serde(default)]
    pub ring_3: Option<u32>,
    #[serde(default)]
    pub ring_4: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WorldMode {
    #[default]
    DungeonCity,
    Countryside,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum GameMode {
    #[default]
    Classic,
    Modern,
}

impl GameMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Modern => "modern",
        }
    }
}

pub const TILE_FLAG_NO_CITY_MOVE: u16 = 0x0001;
pub const TILE_FLAG_PORTCULLIS: u16 = 0x0002;
pub const TILE_FLAG_SECRET: u16 = 0x0004;
pub const TILE_FLAG_BLOCK_MOVE: u16 = 0x0008;
pub const TILE_FLAG_OPENED_DOOR: u16 = 0x0010;
pub const TILE_FLAG_BURNING: u16 = 0x0020;
pub const TILE_FLAG_BURNT: u16 = 0x0040;

pub const SITE_AUX_NONE: i32 = 0;
pub const SITE_AUX_EXIT_COUNTRYSIDE: i32 = 1;
pub const SITE_AUX_EXIT_ARENA: i32 = 2;
pub const SITE_AUX_SERVICE_SHOP: i32 = 10;
pub const SITE_AUX_SERVICE_BANK: i32 = 11;
pub const SITE_AUX_SERVICE_MERC_GUILD: i32 = 12;
pub const SITE_AUX_SERVICE_TEMPLE: i32 = 13;
pub const SITE_AUX_SERVICE_COLLEGE: i32 = 14;
pub const SITE_AUX_SERVICE_SORCERORS: i32 = 15;
pub const SITE_AUX_SERVICE_CASTLE: i32 = 16;
pub const SITE_AUX_SERVICE_ORDER: i32 = 17;
pub const SITE_AUX_SERVICE_CHARITY: i32 = 18;
pub const SITE_AUX_SERVICE_ARENA: i32 = 19;
pub const SITE_AUX_SERVICE_THIEVES: i32 = 20;
pub const SITE_AUX_SERVICE_PALACE: i32 = 21;
pub const SITE_AUX_SERVICE_MONASTERY: i32 = 22;
pub const SITE_AUX_SERVICE_ARMORER: i32 = 23;
pub const SITE_AUX_SERVICE_CLUB: i32 = 24;
pub const SITE_AUX_SERVICE_GYM: i32 = 25;
pub const SITE_AUX_SERVICE_HEALER: i32 = 26;
pub const SITE_AUX_SERVICE_CASINO: i32 = 27;
pub const SITE_AUX_SERVICE_COMMANDANT: i32 = 28;
pub const SITE_AUX_SERVICE_DINER: i32 = 29;
pub const SITE_AUX_SERVICE_CRAPS: i32 = 30;
pub const SITE_AUX_SERVICE_TAVERN: i32 = 31;
pub const SITE_AUX_SERVICE_PAWN_SHOP: i32 = 32;
pub const SITE_AUX_SERVICE_BROTHEL: i32 = 33;
pub const SITE_AUX_SERVICE_CONDO: i32 = 34;
pub const SITE_AUX_ALTAR_ODIN: i32 = 101;
pub const SITE_AUX_ALTAR_SET: i32 = 102;
pub const SITE_AUX_ALTAR_ATHENA: i32 = 103;
pub const SITE_AUX_ALTAR_HECATE: i32 = 104;
pub const SITE_AUX_ALTAR_DESTINY: i32 = 105;
pub const DEITY_ID_ODIN: u8 = 1;
pub const DEITY_ID_SET: u8 = 2;
pub const DEITY_ID_ATHENA: u8 = 3;
pub const DEITY_ID_HECATE: u8 = 4;
pub const DEITY_ID_DESTINY: u8 = 5;

pub const COUNTRY_SITE_NONE: u16 = 0;
pub const COUNTRY_SITE_CITY: u16 = 1;
pub const COUNTRY_SITE_VILLAGE: u16 = 2;
pub const COUNTRY_SITE_TEMPLE: u16 = 3;
pub const COUNTRY_SITE_CASTLE: u16 = 4;
pub const COUNTRY_SITE_PALACE: u16 = 5;
pub const COUNTRY_SITE_CAVES: u16 = 6;
pub const COUNTRY_SITE_VOLCANO: u16 = 7;
pub const COUNTRY_SITE_DRAGON_LAIR: u16 = 8;
pub const COUNTRY_SITE_STARPEAK: u16 = 9;
pub const COUNTRY_SITE_MAGIC_ISLE: u16 = 10;
pub const CITY_SITE_GARDEN: u16 = 41;
pub const CITY_SITE_CEMETARY: u16 = 44;

pub const LEGACY_STATUS_CHEATED: u64 = 0x0004_0000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LegacyEnvironment {
    #[default]
    City,
    Countryside,
    Village,
    TacticalMap,
    Sewers,
    Castle,
    Palace,
    Caves,
    Volcano,
    Astral,
    Arena,
    Hovel,
    Mansion,
    House,
    DragonLair,
    Abyss,
    StarPeak,
    MagicIsle,
    Temple,
    Circle,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MapSemanticKind {
    #[default]
    Unknown,
    City,
    Country,
    Dungeon,
    Site,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapBinding {
    pub semantic: MapSemanticKind,
    pub map_id: u16,
    pub level_index: u16,
    pub source: String,
}

impl Default for MapBinding {
    fn default() -> Self {
        Self {
            semantic: MapSemanticKind::Unknown,
            map_id: 0,
            level_index: 0,
            source: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TileSiteCell {
    pub glyph: char,
    pub site_id: u16,
    pub aux: i32,
    pub flags: u16,
}

impl Default for TileSiteCell {
    fn default() -> Self {
        Self { glyph: '.', site_id: 0, aux: 0, flags: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SiteMapDefinition {
    pub map_id: u16,
    pub level_index: u16,
    pub source: String,
    pub environment: LegacyEnvironment,
    pub semantic: MapSemanticKind,
    pub spawn: Position,
    pub rows: Vec<String>,
    #[serde(default)]
    pub site_grid: Vec<TileSiteCell>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CountryTerrainKind {
    #[default]
    Unknown,
    Plains,
    Tundra,
    Road,
    Mountains,
    Pass,
    River,
    City,
    Village,
    Forest,
    Jungle,
    Swamp,
    Volcano,
    Castle,
    Temple,
    Caves,
    Desert,
    ChaosSea,
    StarPeak,
    DragonLair,
    MagicIsle,
    Palace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CountryCell {
    pub glyph: char,
    pub base_terrain: CountryTerrainKind,
    pub current_terrain: CountryTerrainKind,
    pub aux: u8,
    pub status: u8,
}

impl Default for CountryCell {
    fn default() -> Self {
        Self {
            glyph: '.',
            base_terrain: CountryTerrainKind::Unknown,
            current_terrain: CountryTerrainKind::Unknown,
            aux: 0,
            status: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CountryGrid {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<CountryCell>,
}

impl Default for CountryGrid {
    fn default() -> Self {
        Self { width: 0, height: 0, cells: Vec::new() }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Alignment {
    Lawful,
    #[default]
    Neutral,
    Chaotic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LegacyVerbosity {
    Terse,
    #[default]
    Medium,
    Verbose,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum LegacyQuestState {
    #[default]
    NotStarted,
    Active,
    ArtifactRecovered,
    ReturnToPatron,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum EndingKind {
    #[default]
    None,
    Defeat,
    Victory,
    TotalWinner,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VictoryTrigger {
    RetireCondo,
    QuitConfirmed,
    ExplicitQuestCompletion,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CombatLine {
    High,
    #[default]
    Center,
    Low,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum CombatManeuver {
    #[default]
    Attack,
    Block,
    Riposte,
    Lunge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatStep {
    pub maneuver: CombatManeuver,
    pub line: CombatLine,
}

impl Default for CombatStep {
    fn default() -> Self {
        Self { maneuver: CombatManeuver::Attack, line: CombatLine::Center }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuildTrackState {
    #[serde(default)]
    pub rank: i16,
    #[serde(default)]
    pub xp: i64,
    #[serde(default)]
    pub dues_paid: i64,
    #[serde(default)]
    pub salary_due: i64,
    #[serde(default)]
    pub promotion_flags: u64,
    #[serde(default)]
    pub quest_flags: u64,
}

impl Default for GuildTrackState {
    fn default() -> Self {
        Self { rank: 0, xp: 0, dues_paid: 0, salary_due: 0, promotion_flags: 0, quest_flags: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuestProgression {
    #[serde(default)]
    pub merc: GuildTrackState,
    #[serde(default)]
    pub arena: GuildTrackState,
    #[serde(default)]
    pub thieves: GuildTrackState,
    #[serde(default)]
    pub college: GuildTrackState,
    #[serde(default)]
    pub sorcerors: GuildTrackState,
    #[serde(default)]
    pub order: GuildTrackState,
    #[serde(default)]
    pub temple: GuildTrackState,
    #[serde(default)]
    pub castle: GuildTrackState,
    #[serde(default)]
    pub palace: GuildTrackState,
    #[serde(default)]
    pub charity: GuildTrackState,
    #[serde(default)]
    pub bank: GuildTrackState,
    #[serde(default)]
    pub monastery: GuildTrackState,
}

impl Default for QuestProgression {
    fn default() -> Self {
        Self {
            merc: GuildTrackState::default(),
            arena: GuildTrackState::default(),
            thieves: GuildTrackState::default(),
            college: GuildTrackState::default(),
            sorcerors: GuildTrackState::default(),
            order: GuildTrackState::default(),
            temple: GuildTrackState::default(),
            castle: GuildTrackState::default(),
            palace: GuildTrackState::default(),
            charity: GuildTrackState::default(),
            bank: GuildTrackState::default(),
            monastery: GuildTrackState::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MainQuestState {
    #[serde(default)]
    pub stage: LegacyQuestState,
    #[serde(default)]
    pub objective: String,
    #[serde(default)]
    pub completion_flags: u64,
    #[serde(default)]
    pub palace_access: bool,
    #[serde(default)]
    pub chaos_path: bool,
    #[serde(default)]
    pub law_path: bool,
}

impl Default for MainQuestState {
    fn default() -> Self {
        Self {
            stage: LegacyQuestState::NotStarted,
            objective: String::new(),
            completion_flags: 0,
            palace_access: false,
            chaos_path: false,
            law_path: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObjectiveStep {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObjectiveHint {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub target: Option<Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ObjectiveSnapshot {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub steps: Vec<ObjectiveStep>,
    #[serde(default)]
    pub hints: Vec<ObjectiveHint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerProgression {
    pub guild_rank: u8,
    pub priest_rank: u8,
    pub alignment: Alignment,
    pub law_chaos_score: i32,
    pub deity_favor: i32,
    #[serde(default)]
    pub patron_deity: u8,
    #[serde(default)]
    pub deity_blessing_ready: bool,
    #[serde(default)]
    pub arena_rank: i8,
    #[serde(default)]
    pub arena_opponent: u8,
    #[serde(default)]
    pub arena_match_active: bool,
    pub quest_state: LegacyQuestState,
    pub total_winner_unlocked: bool,
    pub quest_steps_completed: u8,
    pub ending: EndingKind,
    pub score: i64,
    pub high_score_eligible: bool,
    #[serde(default)]
    pub lunarity: i8,
    #[serde(default)]
    pub quests: QuestProgression,
    #[serde(default)]
    pub main_quest: MainQuestState,
    #[serde(default)]
    pub adept_rank: i8,
    #[serde(default)]
    pub victory_trigger: Option<VictoryTrigger>,
}

impl Default for PlayerProgression {
    fn default() -> Self {
        Self {
            guild_rank: 0,
            priest_rank: 0,
            alignment: Alignment::Neutral,
            law_chaos_score: 0,
            deity_favor: 0,
            patron_deity: 0,
            deity_blessing_ready: false,
            arena_rank: 0,
            arena_opponent: 0,
            arena_match_active: false,
            quest_state: LegacyQuestState::NotStarted,
            total_winner_unlocked: false,
            quest_steps_completed: 0,
            ending: EndingKind::None,
            score: 0,
            high_score_eligible: true,
            lunarity: 0,
            quests: QuestProgression::default(),
            main_quest: MainQuestState::default(),
            adept_rank: 0,
            victory_trigger: None,
        }
    }
}

fn quest_state_order(state: LegacyQuestState) -> u8 {
    match state {
        LegacyQuestState::NotStarted => 0,
        LegacyQuestState::Active => 1,
        LegacyQuestState::ArtifactRecovered => 2,
        LegacyQuestState::ReturnToPatron => 3,
        LegacyQuestState::Completed => 4,
        LegacyQuestState::Failed => 5,
    }
}

fn sync_progression_tracks_from_legacy(progression: &mut PlayerProgression) {
    progression.quests.merc.rank =
        progression.quests.merc.rank.max(i16::from(progression.guild_rank));
    progression.quests.temple.rank =
        progression.quests.temple.rank.max(i16::from(progression.priest_rank));
    progression.quests.arena.rank =
        progression.quests.arena.rank.max(i16::from(progression.arena_rank));

    if quest_state_order(progression.quest_state) > quest_state_order(progression.main_quest.stage)
    {
        progression.main_quest.stage = progression.quest_state;
    } else if quest_state_order(progression.main_quest.stage)
        > quest_state_order(progression.quest_state)
    {
        progression.quest_state = progression.main_quest.stage;
    }

    if progression.adept_rank <= 0 && progression.total_winner_unlocked {
        progression.adept_rank = 1;
    }
    if progression.adept_rank > 0 {
        progression.total_winner_unlocked = true;
    }
}

fn sync_legacy_progression_from_tracks(progression: &mut PlayerProgression) {
    progression.guild_rank = progression.quests.merc.rank.max(0).min(u8::MAX as i16) as u8;
    progression.priest_rank = progression.quests.temple.rank.max(0).min(u8::MAX as i16) as u8;
    progression.arena_rank =
        progression.quests.arena.rank.max(i8::MIN as i16).min(i8::MAX as i16) as i8;
    progression.quest_state = progression.main_quest.stage;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusEffect {
    pub id: String,
    pub remaining_turns: u32,
    pub magnitude: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeOptions {
    pub topinv: bool,
    pub belligerent: bool,
    pub runstop: bool,
    pub jumpmove: bool,
    pub pickup: bool,
    pub confirm: bool,
    pub packadd: bool,
    pub compress: bool,
    pub colour: bool,
    pub verbosity: LegacyVerbosity,
    pub searchnum: u8,
    #[serde(default)]
    pub interactive_sites: bool,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            topinv: false,
            belligerent: false,
            runstop: true,
            jumpmove: false,
            pickup: false,
            confirm: true,
            packadd: false,
            compress: true,
            colour: true,
            verbosity: LegacyVerbosity::Medium,
            searchnum: 1,
            interactive_sites: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SiteInteractionKind {
    Shop,
    Armorer,
    Club,
    Gym,
    Healer,
    Casino,
    Commandant,
    Diner,
    Craps,
    Tavern,
    PawnShop,
    Brothel,
    Condo,
    Bank,
    MercGuild,
    ThievesGuild,
    Temple,
    College,
    Sorcerors,
    Castle,
    Palace,
    Order,
    Charity,
    Monastery,
    Arena,
    Altar { deity_id: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WizardInteraction {
    EnterWizardConfirm { via_backdoor: bool },
    WishTextEntry { blessing: i8 },
    WishAcquisitionKindSelect { cheated: bool, item_hint: Option<String> },
    WishAcquisitionItemSelect { cheated: bool, kind: WishItemKind },
    StatusFlagActionSelect,
    StatusFlagIndexEntry { set_mode: bool },
    StatEditorSelect { slot: u8 },
    StatEditorValueEntry { slot: u8 },
    BashDirectionSelect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InventoryInteraction {
    Control { top_mode: bool, selected_slot: usize },
    TakeFromPackSelect { top_mode: bool, selected_slot: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemPromptContext {
    Quaff,
    Read,
    Eat,
    Drop,
    FireThrow,
    ActivateThing,
    ZapStick,
    ActivateArtifact,
    CallItem,
    Give,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemPromptFilter {
    Any,
    Families(Vec<ItemFamily>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemPromptInteraction {
    pub context: ItemPromptContext,
    pub filter: ItemPromptFilter,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellInteraction {
    SpellSelect { filtered_indices: Vec<usize>, cursor: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationInteraction {
    ChooseKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuitInteraction {
    ConfirmQuit,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TalkDirectionInteraction {
    Talk,
    Tunnel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TargetingInteraction {
    pub origin: Position,
    pub cursor: Position,
    pub mode: ProjectileKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PendingProjectileAction {
    pub source_token: String,
    pub turn_minutes: u64,
    pub mode: ProjectileKind,
    pub item_id: Option<u32>,
    pub item_name: String,
    pub hit_bonus: i32,
    pub damage_bonus: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub damage_type: ProjectileDamageType,
    pub max_range: i32,
    pub allows_drop: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WishItemKind {
    Potion,
    Scroll,
    Ring,
    Stick,
    Armor,
    Shield,
    Weapon,
    Boots,
    Cloak,
    Food,
    Thing,
    Artifact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectileKind {
    ThrownItem,
    Arrow,
    Bolt,
    MagicMissile,
    FireBolt,
    LightningBolt,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectileDamageType {
    Normal,
    Flame,
    Electricity,
    Cold,
    Magic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectileResolution {
    pub final_pos: Position,
    pub hit_monster_id: Option<u64>,
    pub dropped_item: Option<String>,
    pub consumed_item: bool,
    pub log_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiLogClass {
    Timeline,
    Prompt,
    Hint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalInputProfile {
    None,
    TextEntry,
    ChoiceEntry,
    DirectionEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WishIntent {
    Death,
    Power,
    Skill,
    Wealth,
    Balance,
    Chaos,
    Law,
    Location,
    Knowledge,
    Health,
    Destruction,
    Acquisition { item_hint: Option<String> },
    Summoning,
    Stats,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WizardSession {
    pub enabled: bool,
    pub scoring_allowed: bool,
}

impl Default for WizardSession {
    fn default() -> Self {
        Self { enabled: false, scoring_allowed: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrimaryAttributes {
    pub strength: i32,
    pub constitution: i32,
    pub dexterity: i32,
    pub agility: i32,
    pub iq: i32,
    pub power: i32,
}

impl Default for PrimaryAttributes {
    fn default() -> Self {
        Self { strength: 12, constitution: 12, dexterity: 12, agility: 12, iq: 12, power: 12 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResistanceProfile {
    pub fire: i16,
    pub cold: i16,
    pub electricity: i16,
    pub poison: i16,
    pub magic: i16,
}

impl Default for ResistanceProfile {
    fn default() -> Self {
        Self { fire: 0, cold: 0, electricity: 0, poison: 0, magic: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImmunityFlags {
    pub poison: bool,
    pub sleep: bool,
    pub fear: bool,
}

impl Default for ImmunityFlags {
    fn default() -> Self {
        Self { poison: false, sleep: false, fear: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldTopology {
    pub dungeon_level: i16,
    pub city_site_id: u8,
    pub country_region_id: u8,
    pub last_city_position: Option<Position>,
    pub last_country_position: Option<Position>,
    pub country_rampart_position: Option<Position>,
}

impl Default for WorldTopology {
    fn default() -> Self {
        Self {
            dungeon_level: 0,
            city_site_id: 0,
            country_region_id: 0,
            last_city_position: None,
            last_country_position: None,
            country_rampart_position: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TurnScheduler {
    pub player_phase: u64,
    pub monster_phase: u64,
    pub environment_phase: u64,
    pub timed_effect_phase: u64,
}

impl Default for TurnScheduler {
    fn default() -> Self {
        Self { player_phase: 0, monster_phase: 0, environment_phase: 0, timed_effect_phase: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpellState {
    pub id: u8,
    pub known: bool,
    pub power_drain: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpellbookState {
    pub mana: i32,
    pub max_mana: i32,
    #[serde(default = "default_spellbook_spells")]
    pub spells: Vec<SpellState>,
    #[serde(default)]
    pub next_spell_index: u8,
}

impl Default for SpellbookState {
    fn default() -> Self {
        Self { mana: 120, max_mana: 120, spells: default_spellbook_spells(), next_spell_index: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterArchetype {
    pub id: String,
    pub label: String,
    pub stats: Stats,
    pub starting_gold: i32,
    pub starting_mana: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterCreation {
    pub name: String,
    pub archetype_id: String,
    pub alignment: Alignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyQuestionnaireProfile {
    pub strength: i32,
    pub iq: i32,
    pub agility: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub power: i32,
    pub preference: char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyQuestionnaireCreation {
    pub creation: CharacterCreation,
    pub profile: LegacyQuestionnaireProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyQuestionnaireAnswers {
    pub bench_press_lbs: i32,
    pub took_iq_test: bool,
    pub iq_score: i32,
    pub took_undergraduate_exam: bool,
    pub undergraduate_percentile: i32,
    pub took_graduate_exam: bool,
    pub graduate_percentile: i32,
    pub pretty_dumb: bool,
    pub can_dance: bool,
    pub dance_well: bool,
    pub has_martial_training: bool,
    pub has_dan_rank: bool,
    pub plays_field_sport: bool,
    pub good_field_sport: bool,
    pub does_caving_or_mountaineering: bool,
    pub skates_or_skis: bool,
    pub good_at_skating_or_skiing: bool,
    pub physically_handicapped: bool,
    pub accident_prone: bool,
    pub can_ride_bicycle: bool,
    pub plays_video_games: bool,
    pub gets_high_scores: bool,
    pub archer_fencer_marksman: bool,
    pub good_marksman: bool,
    pub picked_lock: bool,
    pub typing_speed_wpm: i32,
    pub hand_shaking: bool,
    pub ambidextrous: bool,
    pub can_cut_deck_one_hand: bool,
    pub can_tie_shoes_blindfolded: bool,
    pub gets_colds: bool,
    pub colds_frequent: bool,
    pub recent_serious_accident_or_illness: bool,
    pub chronic_disease: bool,
    pub overweight_or_underweight_20pct: bool,
    pub high_blood_pressure: bool,
    pub smokes: bool,
    pub aerobics_classes: bool,
    pub miles_can_run: i32,
    pub animals_react_oddly: bool,
    pub can_see_auras: bool,
    pub out_of_body_experience: bool,
    pub cast_spell: bool,
    pub spell_worked: bool,
    pub has_esp: bool,
    pub has_pk: bool,
    pub believes_in_ghosts: bool,
    pub is_irish: bool,
    pub sexual_preference: char,
}

impl Default for LegacyQuestionnaireAnswers {
    fn default() -> Self {
        Self {
            bench_press_lbs: 0,
            took_iq_test: false,
            iq_score: 0,
            took_undergraduate_exam: false,
            undergraduate_percentile: 0,
            took_graduate_exam: false,
            graduate_percentile: 0,
            pretty_dumb: false,
            can_dance: false,
            dance_well: false,
            has_martial_training: false,
            has_dan_rank: false,
            plays_field_sport: false,
            good_field_sport: false,
            does_caving_or_mountaineering: false,
            skates_or_skis: false,
            good_at_skating_or_skiing: false,
            physically_handicapped: false,
            accident_prone: false,
            can_ride_bicycle: true,
            plays_video_games: false,
            gets_high_scores: false,
            archer_fencer_marksman: false,
            good_marksman: false,
            picked_lock: false,
            typing_speed_wpm: 0,
            hand_shaking: false,
            ambidextrous: false,
            can_cut_deck_one_hand: false,
            can_tie_shoes_blindfolded: true,
            gets_colds: false,
            colds_frequent: false,
            recent_serious_accident_or_illness: false,
            chronic_disease: false,
            overweight_or_underweight_20pct: false,
            high_blood_pressure: false,
            smokes: false,
            aerobics_classes: false,
            miles_can_run: 0,
            animals_react_oddly: false,
            can_see_auras: false,
            out_of_body_experience: false,
            cast_spell: false,
            spell_worked: false,
            has_esp: false,
            has_pk: false,
            believes_in_ghosts: false,
            is_irish: false,
            sexual_preference: 'n',
        }
    }
}

pub fn derive_legacy_questionnaire_profile(
    answers: &LegacyQuestionnaireAnswers,
) -> LegacyQuestionnaireProfile {
    let strength = if answers.bench_press_lbs < 30 {
        3
    } else if answers.bench_press_lbs < 90 {
        answers.bench_press_lbs / 10
    } else {
        ((answers.bench_press_lbs - 120) / 30 + 9).clamp(3, 18)
    };

    let mut iq_points = 0i32;
    let mut iq_entries = 0i32;

    if answers.took_iq_test {
        iq_entries += 1;
        iq_points += (answers.iq_score / 10).clamp(0, 18);
    }
    if answers.took_undergraduate_exam {
        iq_entries += 1;
        let pct = answers.undergraduate_percentile.clamp(0, 99);
        iq_points += (9 * (pct - 49) / 50 + 9).clamp(0, 18);
    }
    if answers.took_graduate_exam {
        iq_entries += 1;
        let pct = answers.graduate_percentile.clamp(0, 99);
        iq_points += (9 * (pct - 49) / 50 + 9).clamp(0, 18);
    }

    // Legacy used random_range fallback. We project deterministic midpoint values.
    let iq = if iq_entries == 0 {
        if answers.pretty_dumb { 4 } else { 8 }
    } else {
        iq_points / iq_entries
    }
    .clamp(3, 18);

    let mut agility_points = 0i32;
    agility_points += if !answers.can_dance {
        0
    } else if !answers.dance_well {
        1
    } else {
        3
    };
    agility_points += if !answers.has_martial_training {
        0
    } else if !answers.has_dan_rank {
        2
    } else {
        6
    };
    agility_points += if !answers.plays_field_sport {
        0
    } else if !answers.good_field_sport {
        1
    } else {
        2
    };
    agility_points += if answers.does_caving_or_mountaineering { 3 } else { 0 };
    agility_points += if !answers.skates_or_skis {
        0
    } else if !answers.good_at_skating_or_skiing {
        2
    } else {
        4
    };
    agility_points += if answers.physically_handicapped { -4 } else { 0 };
    agility_points += if answers.accident_prone { -4 } else { 0 };
    agility_points += if answers.can_ride_bicycle { 0 } else { -4 };
    let agility = (9 + agility_points / 2).clamp(3, 18);

    let mut dexterity_points = 0i32;
    dexterity_points += if !answers.plays_video_games {
        0
    } else if !answers.gets_high_scores {
        2
    } else {
        6
    };
    dexterity_points += if !answers.archer_fencer_marksman {
        0
    } else if !answers.good_marksman {
        2
    } else {
        6
    };
    dexterity_points += if answers.picked_lock { 2 } else { 0 };
    dexterity_points += answers.typing_speed_wpm.min(124) / 25;
    dexterity_points += if answers.hand_shaking { -3 } else { 0 };
    dexterity_points += if answers.ambidextrous { 4 } else { 0 };
    dexterity_points += if answers.can_cut_deck_one_hand { 2 } else { 0 };
    dexterity_points += if answers.can_tie_shoes_blindfolded { 0 } else { -3 };
    let dexterity = (6 + dexterity_points / 2).clamp(3, 18);

    let mut constitution_points = 0i32;
    constitution_points += if !answers.gets_colds {
        4
    } else if !answers.colds_frequent {
        0
    } else {
        -4
    };
    constitution_points += if answers.recent_serious_accident_or_illness { -4 } else { 4 };
    constitution_points += if answers.chronic_disease { -4 } else { 0 };
    constitution_points += if answers.overweight_or_underweight_20pct { -2 } else { 0 };
    constitution_points += if answers.high_blood_pressure { -2 } else { 0 };
    constitution_points += if answers.smokes { -3 } else { 0 };
    constitution_points += if answers.aerobics_classes { 2 } else { 0 };
    constitution_points += if answers.miles_can_run < 1 {
        -3
    } else if answers.miles_can_run < 5 {
        2
    } else if answers.miles_can_run < 10 {
        4
    } else {
        8
    };
    let constitution = (12 + constitution_points / 3).clamp(3, 18);

    let mut power_points = 0i32;
    power_points += if answers.animals_react_oddly { 2 } else { 0 };
    power_points += if answers.can_see_auras { 3 } else { 0 };
    power_points += if answers.out_of_body_experience { 3 } else { 0 };
    power_points += if !answers.cast_spell {
        0
    } else if !answers.spell_worked {
        3
    } else {
        7
    };
    power_points += if answers.has_esp { 3 } else { 0 };
    power_points += if answers.has_pk { 6 } else { 0 };
    power_points += if answers.believes_in_ghosts { 2 } else { 0 };
    power_points += if answers.is_irish { 2 } else { 0 };
    let power = (3 + power_points / 2).clamp(3, 18);

    let preference = match answers.sexual_preference.to_ascii_lowercase() {
        'm' | 'f' | 'y' | 'n' => answers.sexual_preference.to_ascii_lowercase(),
        _ => 'n',
    };

    LegacyQuestionnaireProfile { strength, iq, agility, dexterity, constitution, power, preference }
}

pub fn derive_legacy_questionnaire_creation(
    name: String,
    answers: &LegacyQuestionnaireAnswers,
) -> LegacyQuestionnaireCreation {
    let profile = derive_legacy_questionnaire_profile(answers);
    let fighter_score = profile.strength * 3 + profile.constitution * 2 + profile.agility;
    let rogue_score = profile.dexterity * 3 + profile.agility * 2 + profile.iq;
    let mage_score = profile.iq * 3 + profile.power * 3 + profile.dexterity;
    let priest_score = profile.power * 3 + profile.iq * 2 + profile.constitution;

    let mut best_id = "fighter";
    let mut best_score = fighter_score;
    for (id, score) in [("rogue", rogue_score), ("mage", mage_score), ("priest", priest_score)] {
        if score > best_score {
            best_id = id;
            best_score = score;
        }
    }
    let _ = best_score;

    let creation = CharacterCreation {
        name,
        archetype_id: best_id.to_string(),
        alignment: Alignment::Neutral,
    };
    LegacyQuestionnaireCreation { creation, profile }
}

pub fn apply_legacy_questionnaire_profile(
    state: &mut GameState,
    profile: LegacyQuestionnaireProfile,
) {
    state.attributes.strength = profile.strength.clamp(1, 32);
    state.attributes.iq = profile.iq.clamp(1, 32);
    state.attributes.agility = profile.agility.clamp(1, 32);
    state.attributes.dexterity = profile.dexterity.clamp(1, 32);
    state.attributes.constitution = profile.constitution.clamp(1, 32);
    state.attributes.power = profile.power.clamp(1, 32);

    let base_stats = state.player.stats;
    let strength_delta = profile.strength - 12;
    let iq_delta = profile.iq - 12;
    let agility_delta = profile.agility - 12;
    let dexterity_delta = profile.dexterity - 12;
    let constitution_delta = profile.constitution - 12;
    let power_delta = profile.power - 12;

    let max_hp = (base_stats.max_hp + constitution_delta / 2 + strength_delta / 4).clamp(12, 40);
    let attack_min =
        (base_stats.attack_min + strength_delta / 6 + dexterity_delta / 7).clamp(1, 12);
    let attack_max =
        (base_stats.attack_max + strength_delta / 5 + agility_delta / 6 + power_delta / 8)
            .clamp(attack_min + 1, 18);
    let defense =
        (base_stats.defense + agility_delta / 6 + dexterity_delta / 8 + constitution_delta / 10)
            .clamp(0, 10);

    state.player.stats.max_hp = max_hp;
    state.player.stats.hp = max_hp;
    state.player.stats.attack_min = attack_min;
    state.player.stats.attack_max = attack_max;
    state.player.stats.defense = defense;

    let projected_mana = (state.spellbook.max_mana + power_delta * 6 + iq_delta * 3).clamp(40, 260);
    state.spellbook.max_mana = projected_mana;
    state.spellbook.mana = projected_mana;

    state.gold = (state.gold + iq_delta * 6 + dexterity_delta * 4).clamp(80, 600);
    state.progression.alignment = Alignment::Neutral;
    state.progression.law_chaos_score = 0;
}

pub fn default_character_archetypes() -> Vec<CharacterArchetype> {
    vec![
        CharacterArchetype {
            id: "fighter".to_string(),
            label: "Fighter".to_string(),
            stats: Stats { hp: 26, max_hp: 26, attack_min: 3, attack_max: 7, defense: 2 },
            starting_gold: 320,
            starting_mana: 80,
        },
        CharacterArchetype {
            id: "mage".to_string(),
            label: "Mage".to_string(),
            stats: Stats { hp: 18, max_hp: 18, attack_min: 2, attack_max: 5, defense: 1 },
            starting_gold: 220,
            starting_mana: 160,
        },
        CharacterArchetype {
            id: "rogue".to_string(),
            label: "Rogue".to_string(),
            stats: Stats { hp: 22, max_hp: 22, attack_min: 2, attack_max: 6, defense: 1 },
            starting_gold: 260,
            starting_mana: 110,
        },
        CharacterArchetype {
            id: "priest".to_string(),
            label: "Priest".to_string(),
            stats: Stats { hp: 20, max_hp: 20, attack_min: 2, attack_max: 6, defense: 1 },
            starting_gold: 240,
            starting_mana: 140,
        },
    ]
}

pub fn apply_character_creation(state: &mut GameState, creation: &CharacterCreation) {
    let archetypes = default_character_archetypes();
    let selected = archetypes
        .iter()
        .find(|arch| arch.id.eq_ignore_ascii_case(&creation.archetype_id))
        .cloned()
        .unwrap_or_else(|| archetypes.first().cloned().unwrap());

    if !creation.name.trim().is_empty() {
        state.player_name = creation.name.trim().to_string();
    }
    state.player.stats = selected.stats;
    state.gold = selected.starting_gold;
    state.spellbook.max_mana = selected.starting_mana;
    state.spellbook.mana = selected.starting_mana;
    initialize_spell_knowledge_for_archetype(state, &selected.id);
    state.attributes = PrimaryAttributes::default();
    state.progression.alignment = creation.alignment;
    state.progression.law_chaos_score = match creation.alignment {
        Alignment::Lawful => 5,
        Alignment::Neutral => 0,
        Alignment::Chaotic => -5,
    };
}

fn set_spell_known(state: &mut GameState, spell_id: usize, known: bool) {
    if let Some(spell) = state.spellbook.spells.get_mut(spell_id) {
        spell.known = known;
    }
}

fn learn_spell_id(state: &mut GameState, spell_id: usize) -> bool {
    if let Some(spell) = state.spellbook.spells.get_mut(spell_id)
        && !spell.known
    {
        spell.known = true;
        return true;
    }
    false
}

fn teach_first_unknown_from_pool(state: &mut GameState, pool: &[usize]) -> Option<usize> {
    for spell_id in pool {
        if learn_spell_id(state, *spell_id) {
            return Some(*spell_id);
        }
    }
    None
}

fn initialize_spell_knowledge_for_archetype(state: &mut GameState, archetype_id: &str) {
    sync_spellbook_state(state);
    for spell_id in 0..state.spellbook.spells.len() {
        set_spell_known(state, spell_id, false);
    }

    let known_set: &[usize] = match archetype_id {
        "mage" => &[2, 3, 12, 1],
        "priest" => &[17, 10, 19],
        "rogue" => &[1, 38],
        _ => &[],
    };
    for spell_id in known_set {
        set_spell_known(state, *spell_id, true);
    }
}

fn recompute_derived_combat_and_mana_from_attributes(state: &mut GameState) {
    let strength_delta = state.attributes.strength - 12;
    let iq_delta = state.attributes.iq - 12;
    let agility_delta = state.attributes.agility - 12;
    let dexterity_delta = state.attributes.dexterity - 12;
    let constitution_delta = state.attributes.constitution - 12;
    let power_delta = state.attributes.power - 12;

    let max_hp = (20 + constitution_delta / 2 + strength_delta / 4).clamp(12, 60);
    let attack_min = (2 + strength_delta / 6 + dexterity_delta / 7).clamp(1, 14);
    let attack_max =
        (6 + strength_delta / 5 + agility_delta / 6 + power_delta / 8).clamp(attack_min + 1, 24);
    let defense =
        (1 + agility_delta / 6 + dexterity_delta / 8 + constitution_delta / 10).clamp(0, 16);
    let max_mana = (100 + power_delta * 6 + iq_delta * 3).clamp(40, 320);

    state.player.stats.max_hp = max_hp;
    state.player.stats.hp = state.player.stats.hp.clamp(0, max_hp);
    state.player.stats.attack_min = attack_min;
    state.player.stats.attack_max = attack_max;
    state.player.stats.defense = defense;
    state.spellbook.max_mana = max_mana;
    state.spellbook.mana = state.spellbook.mana.clamp(0, max_mana);
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroundItem {
    pub position: Position,
    pub item: Item,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Player {
    pub position: Position,
    pub stats: Stats,
    pub inventory: Vec<Item>,
    pub inventory_capacity: usize,
    #[serde(default = "default_pack_capacity")]
    pub pack_capacity: usize,
    #[serde(default)]
    pub pack_order: Vec<u32>,
    #[serde(default)]
    pub equipment: EquipmentSlots,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MonsterBehavior {
    #[default]
    Brute,
    Skirmisher,
    Caster,
    Social,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Faction {
    Law,
    #[default]
    Neutral,
    Chaos,
    Wild,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Monster {
    pub id: u64,
    pub name: String,
    pub position: Position,
    pub stats: Stats,
    #[serde(default)]
    pub behavior: MonsterBehavior,
    #[serde(default)]
    pub faction: Faction,
    #[serde(default)]
    pub display_glyph: Option<char>,
    #[serde(default)]
    pub on_death_drops: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Trap {
    pub id: u64,
    pub position: Position,
    pub damage: i32,
    pub effect_id: String,
    pub armed: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameClock {
    pub turn: u64,
    pub minutes: u64,
    pub minutes_per_turn: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SessionStatus {
    #[default]
    InProgress,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameState {
    pub bounds: MapBounds,
    #[serde(default)]
    pub mode: GameMode,
    #[serde(default)]
    pub map_rows: Vec<String>,
    #[serde(default)]
    pub city_map_rows: Vec<String>,
    #[serde(default)]
    pub country_map_rows: Vec<String>,
    #[serde(default)]
    pub site_grid: Vec<TileSiteCell>,
    #[serde(default)]
    pub city_site_grid: Vec<TileSiteCell>,
    #[serde(default)]
    pub country_site_grid: Vec<TileSiteCell>,
    #[serde(default)]
    pub country_grid: CountryGrid,
    #[serde(default)]
    pub city_map_id: u16,
    #[serde(default)]
    pub city_level_index: u16,
    #[serde(default)]
    pub city_map_source: String,
    #[serde(default)]
    pub country_map_id: u16,
    #[serde(default)]
    pub country_level_index: u16,
    #[serde(default)]
    pub country_map_source: String,
    #[serde(default)]
    pub site_maps: Vec<SiteMapDefinition>,
    pub clock: GameClock,
    #[serde(default)]
    pub world_mode: WorldMode,
    #[serde(default)]
    pub environment: LegacyEnvironment,
    #[serde(default)]
    pub map_binding: MapBinding,
    pub player: Player,
    #[serde(default)]
    pub progression: PlayerProgression,
    #[serde(default)]
    pub status_effects: Vec<StatusEffect>,
    #[serde(default)]
    pub options: RuntimeOptions,
    #[serde(default)]
    pub pending_wizard_interaction: Option<WizardInteraction>,
    #[serde(default)]
    pub pending_spell_interaction: Option<SpellInteraction>,
    #[serde(default)]
    pub pending_activation_interaction: Option<ActivationInteraction>,
    #[serde(default)]
    pub pending_quit_interaction: Option<QuitInteraction>,
    #[serde(default)]
    pub pending_talk_direction: Option<TalkDirectionInteraction>,
    #[serde(default)]
    pub pending_inventory_interaction: Option<InventoryInteraction>,
    #[serde(default)]
    pub pending_item_prompt: Option<ItemPromptInteraction>,
    #[serde(default)]
    pub pending_targeting_interaction: Option<TargetingInteraction>,
    #[serde(default)]
    pub pending_projectile_action: Option<PendingProjectileAction>,
    #[serde(default)]
    pub transient_projectile_path: Vec<Position>,
    #[serde(default)]
    pub transient_projectile_impact: Option<Position>,
    #[serde(default)]
    pub wizard_input_buffer: String,
    #[serde(default)]
    pub spell_input_buffer: String,
    #[serde(default)]
    pub interaction_buffer: String,
    #[serde(default)]
    pub target_input_buffer: String,
    #[serde(default)]
    pub legacy_status_flags: u64,
    #[serde(default)]
    pub navigation_lost: bool,
    #[serde(default)]
    pub precipitation: i32,
    #[serde(default)]
    pub chaos_attuned: bool,
    #[serde(default)]
    pub chaos_protection_consumed: bool,
    #[serde(default)]
    pub wizard: WizardSession,
    #[serde(default)]
    pub attributes: PrimaryAttributes,
    #[serde(default)]
    pub resistances: ResistanceProfile,
    #[serde(default)]
    pub immunities: ImmunityFlags,
    #[serde(default = "default_encounter_monsters")]
    pub encounter_monsters: Vec<String>,
    #[serde(default)]
    pub topology: WorldTopology,
    #[serde(default)]
    pub scheduler: TurnScheduler,
    #[serde(default)]
    pub spellbook: SpellbookState,
    #[serde(default = "default_player_name")]
    pub player_name: String,
    #[serde(default = "default_gold")]
    pub gold: i32,
    #[serde(default)]
    pub bank_gold: i32,
    #[serde(default = "default_food")]
    pub food: i32,
    #[serde(default)]
    pub legal_heat: i32,
    #[serde(default)]
    pub known_sites: Vec<Position>,
    #[serde(default)]
    pub pending_confirmation: Option<String>,
    #[serde(default)]
    pub pending_site_interaction: Option<SiteInteractionKind>,
    #[serde(default = "default_combat_sequence")]
    pub combat_sequence: Vec<CombatStep>,
    #[serde(default)]
    pub combat_sequence_cursor: usize,
    #[serde(default)]
    pub action_points_spent: u64,
    #[serde(default = "default_action_points_per_turn")]
    pub action_points_per_turn: u16,
    #[serde(default)]
    pub carry_burden: i32,
    #[serde(default)]
    pub traps: Vec<Trap>,
    pub monsters: Vec<Monster>,
    pub ground_items: Vec<GroundItem>,
    pub log: Vec<String>,
    #[serde(default)]
    pub status: SessionStatus,
    #[serde(default)]
    pub death_source: Option<String>,
    #[serde(default)]
    pub monsters_defeated: u64,
    #[serde(default)]
    pub ai_paused: bool,
    pub next_entity_id: u64,
    pub next_item_id: u32,
}

fn default_player_name() -> String {
    "Adventurer".to_string()
}

fn default_gold() -> i32 {
    250
}

fn default_food() -> i32 {
    3
}

fn default_action_points_per_turn() -> u16 {
    100
}

fn default_pack_capacity() -> usize {
    26
}

fn default_combat_sequence() -> Vec<CombatStep> {
    vec![CombatStep::default()]
}

fn default_encounter_monsters() -> Vec<String> {
    vec!["wolf".to_string(), "bandit".to_string(), "goblin".to_string(), "stalker".to_string()]
}

fn default_spellbook_spells() -> Vec<SpellState> {
    LEGACY_SPELL_COSTS
        .iter()
        .enumerate()
        .map(|(id, drain)| SpellState { id: id as u8, known: false, power_drain: *drain })
        .collect()
}

impl GameState {
    pub fn new(bounds: MapBounds) -> Self {
        let start = Position { x: bounds.width / 2, y: bounds.height / 2 };

        Self {
            bounds,
            mode: GameMode::Classic,
            map_rows: default_map_rows(bounds),
            city_map_rows: default_map_rows(bounds),
            country_map_rows: Vec::new(),
            site_grid: Vec::new(),
            city_site_grid: Vec::new(),
            country_site_grid: Vec::new(),
            country_grid: CountryGrid::default(),
            city_map_id: 0,
            city_level_index: 0,
            city_map_source: String::new(),
            country_map_id: 0,
            country_level_index: 0,
            country_map_source: String::new(),
            site_maps: Vec::new(),
            clock: GameClock { turn: 0, minutes: 0, minutes_per_turn: 6 },
            world_mode: WorldMode::DungeonCity,
            environment: LegacyEnvironment::City,
            map_binding: MapBinding {
                semantic: MapSemanticKind::City,
                map_id: 0,
                level_index: 0,
                source: String::new(),
            },
            player: Player {
                position: start,
                stats: Stats { hp: 20, max_hp: 20, attack_min: 2, attack_max: 6, defense: 1 },
                inventory: Vec::new(),
                inventory_capacity: default_pack_capacity(),
                pack_capacity: default_pack_capacity(),
                pack_order: Vec::new(),
                equipment: EquipmentSlots::default(),
            },
            progression: PlayerProgression::default(),
            status_effects: Vec::new(),
            options: RuntimeOptions::default(),
            pending_wizard_interaction: None,
            pending_spell_interaction: None,
            pending_activation_interaction: None,
            pending_quit_interaction: None,
            pending_talk_direction: None,
            pending_inventory_interaction: None,
            pending_item_prompt: None,
            pending_targeting_interaction: None,
            pending_projectile_action: None,
            transient_projectile_path: Vec::new(),
            transient_projectile_impact: None,
            wizard_input_buffer: String::new(),
            spell_input_buffer: String::new(),
            interaction_buffer: String::new(),
            target_input_buffer: String::new(),
            legacy_status_flags: 0,
            navigation_lost: false,
            precipitation: 0,
            chaos_attuned: false,
            chaos_protection_consumed: false,
            wizard: WizardSession::default(),
            attributes: PrimaryAttributes::default(),
            resistances: ResistanceProfile::default(),
            immunities: ImmunityFlags::default(),
            encounter_monsters: default_encounter_monsters(),
            topology: WorldTopology::default(),
            scheduler: TurnScheduler::default(),
            spellbook: SpellbookState::default(),
            player_name: default_player_name(),
            gold: default_gold(),
            bank_gold: 0,
            food: default_food(),
            legal_heat: 0,
            known_sites: Vec::new(),
            pending_confirmation: None,
            pending_site_interaction: None,
            combat_sequence: default_combat_sequence(),
            combat_sequence_cursor: 0,
            action_points_spent: 0,
            action_points_per_turn: default_action_points_per_turn(),
            carry_burden: 0,
            traps: Vec::new(),
            monsters: Vec::new(),
            ground_items: Vec::new(),
            log: Vec::new(),
            status: SessionStatus::InProgress,
            death_source: None,
            monsters_defeated: 0,
            ai_paused: false,
            next_entity_id: 1,
            next_item_id: 1,
        }
    }

    pub fn with_mode(mode: GameMode, bounds: MapBounds) -> Self {
        let mut state = Self::new(bounds);
        state.mode = mode;
        state
    }

    pub fn spawn_monster(
        &mut self,
        name: impl Into<String>,
        position: Position,
        stats: Stats,
    ) -> u64 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        let name = name.into();
        let (behavior, faction) = infer_monster_profile(&name);
        self.monsters.push(Monster {
            id,
            name,
            position,
            stats,
            behavior,
            faction,
            display_glyph: None,
            on_death_drops: Vec::new(),
        });
        id
    }

    pub fn place_item(&mut self, name: impl Into<String>, position: Position) -> u32 {
        let id = self.next_item_id;
        self.next_item_id += 1;
        let requested = name.into();
        let item = instantiate_item_from_name(id, &requested);
        self.ground_items.push(GroundItem { position, item });
        id
    }

    pub fn is_terminal(&self) -> bool {
        self.status != SessionStatus::InProgress
    }

    pub fn set_map_rows(&mut self, rows: Vec<String>) {
        if let Some(first) = rows.first() {
            self.bounds =
                MapBounds { width: first.chars().count() as i32, height: rows.len() as i32 };
        }
        self.map_rows = rows;
    }

    pub fn tile_site_at(&self, pos: Position) -> Option<&TileSiteCell> {
        if !self.bounds.contains(pos) {
            return None;
        }
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        let width = usize::try_from(self.bounds.width).ok()?;
        self.site_grid.get(y.saturating_mul(width).saturating_add(x))
    }

    pub fn tile_site_at_mut(&mut self, pos: Position) -> Option<&mut TileSiteCell> {
        if !self.bounds.contains(pos) {
            return None;
        }
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        let width = usize::try_from(self.bounds.width).ok()?;
        let idx = y.saturating_mul(width).saturating_add(x);
        self.site_grid.get_mut(idx)
    }

    pub fn set_map_glyph_at(&mut self, pos: Position, glyph: char) -> bool {
        if !self.bounds.contains(pos) {
            return false;
        }
        if !set_row_char(&mut self.map_rows, pos, glyph) {
            return false;
        }
        match self.map_binding.semantic {
            MapSemanticKind::City => {
                let _ = set_row_char(&mut self.city_map_rows, pos, glyph);
            }
            MapSemanticKind::Country => {
                let _ = set_row_char(&mut self.country_map_rows, pos, glyph);
            }
            _ => {}
        }
        true
    }

    pub fn country_cell_at(&self, pos: Position) -> Option<&CountryCell> {
        let width = usize::try_from(self.country_grid.width).ok()?;
        let height = usize::try_from(self.country_grid.height).ok()?;
        let x = usize::try_from(pos.x).ok()?;
        let y = usize::try_from(pos.y).ok()?;
        if x >= width || y >= height {
            return None;
        }
        self.country_grid.cells.get(y.saturating_mul(width).saturating_add(x))
    }

    pub fn map_glyph_at(&self, pos: Position) -> char {
        if !self.bounds.contains(pos) {
            return '#';
        }
        let y = usize::try_from(pos.y).ok();
        let x = usize::try_from(pos.x).ok();
        match (y, x) {
            (Some(y), Some(x)) => {
                self.map_rows.get(y).and_then(|row| row.chars().nth(x)).unwrap_or('.')
            }
            _ => '#',
        }
    }

    pub fn tile_is_walkable(&self, pos: Position) -> bool {
        if !self.bounds.contains(pos) {
            return false;
        }
        let glyph = self.map_glyph_at(pos);
        if glyph == '#' {
            return false;
        }
        if self.world_mode == WorldMode::DungeonCity && glyph == '=' {
            return false;
        }
        if self.world_mode == WorldMode::DungeonCity && matches!(glyph, '-' | 'D' | 'J') {
            return false;
        }
        if let Some(site) = self.tile_site_at(pos)
            && (site.flags & TILE_FLAG_BLOCK_MOVE) != 0
        {
            return false;
        }
        true
    }

    pub fn activate_city_view(&mut self) {
        if !self.city_map_rows.is_empty() {
            self.set_map_rows(self.city_map_rows.clone());
        }
        self.site_grid = self.city_site_grid.clone();
        self.monsters.clear();
        self.pending_site_interaction = None;
        self.pending_spell_interaction = None;
        self.pending_activation_interaction = None;
        self.pending_quit_interaction = None;
        self.pending_talk_direction = None;
        self.pending_inventory_interaction = None;
        self.pending_item_prompt = None;
        self.pending_targeting_interaction = None;
        self.pending_projectile_action = None;
        self.transient_projectile_path.clear();
        self.transient_projectile_impact = None;
        self.spell_input_buffer.clear();
        self.interaction_buffer.clear();
        self.target_input_buffer.clear();
        self.navigation_lost = false;
        self.world_mode = WorldMode::DungeonCity;
        self.environment = LegacyEnvironment::City;
        self.map_binding = MapBinding {
            semantic: MapSemanticKind::City,
            map_id: self.city_map_id,
            level_index: self.city_level_index,
            source: self.city_map_source.clone(),
        };
        let _ = self.spawn_guard_monsters_from_markers();
    }

    pub fn activate_country_view(&mut self) {
        if !self.country_map_rows.is_empty() {
            self.set_map_rows(self.country_map_rows.clone());
        }
        self.site_grid = self.country_site_grid.clone();
        self.monsters.clear();
        self.pending_site_interaction = None;
        self.pending_spell_interaction = None;
        self.pending_activation_interaction = None;
        self.pending_quit_interaction = None;
        self.pending_talk_direction = None;
        self.pending_inventory_interaction = None;
        self.pending_item_prompt = None;
        self.pending_targeting_interaction = None;
        self.pending_projectile_action = None;
        self.transient_projectile_path.clear();
        self.transient_projectile_impact = None;
        self.spell_input_buffer.clear();
        self.interaction_buffer.clear();
        self.target_input_buffer.clear();
        self.world_mode = WorldMode::Countryside;
        self.environment = LegacyEnvironment::Countryside;
        self.map_binding = MapBinding {
            semantic: MapSemanticKind::Country,
            map_id: self.country_map_id,
            level_index: self.country_level_index,
            source: self.country_map_source.clone(),
        };
    }

    pub fn activate_site_map_by_id(
        &mut self,
        map_id: u16,
        spawn_override: Option<Position>,
    ) -> bool {
        let Some(site_map) =
            self.site_maps.iter().find(|candidate| candidate.map_id == map_id).cloned()
        else {
            return false;
        };

        self.set_map_rows(site_map.rows.clone());
        if site_map.site_grid.is_empty() {
            let cell_count =
                usize::try_from(self.bounds.width.saturating_mul(self.bounds.height)).unwrap_or(0);
            self.site_grid = vec![TileSiteCell::default(); cell_count];
        } else {
            self.site_grid = site_map.site_grid.clone();
        }
        self.monsters.clear();
        self.pending_site_interaction = None;
        self.pending_spell_interaction = None;
        self.pending_activation_interaction = None;
        self.pending_quit_interaction = None;
        self.pending_talk_direction = None;
        self.pending_inventory_interaction = None;
        self.pending_item_prompt = None;
        self.pending_targeting_interaction = None;
        self.pending_projectile_action = None;
        self.transient_projectile_path.clear();
        self.transient_projectile_impact = None;
        self.spell_input_buffer.clear();
        self.interaction_buffer.clear();
        self.target_input_buffer.clear();
        self.navigation_lost = false;
        self.world_mode = WorldMode::DungeonCity;
        self.environment = site_map.environment;
        self.map_binding = MapBinding {
            semantic: site_map.semantic,
            map_id: site_map.map_id,
            level_index: site_map.level_index,
            source: site_map.source.clone(),
        };

        let requested_spawn = spawn_override.unwrap_or(site_map.spawn);
        if let Some(spawn) = sanitize_spawn(self, requested_spawn) {
            self.player.position = spawn;
        }
        let _ = self.spawn_guard_monsters_from_markers();

        true
    }

    pub fn spawn_guard_monsters_from_markers(&mut self) -> usize {
        let markers = guard_marker_positions(&self.map_rows, self.bounds);
        let mut spawned = 0usize;
        for pos in markers {
            let _ = set_row_char(&mut self.map_rows, pos, '.');
            if let Some(cell) = self.tile_site_at_mut(pos) {
                cell.glyph = '.';
                cell.flags &= !TILE_FLAG_BLOCK_MOVE;
            }
            if self.player.position == pos
                || self.monsters.iter().any(|monster| monster.position == pos)
            {
                continue;
            }
            let guard_id = self.spawn_monster("city guard", pos, guard_marker_stats());
            if let Some(guard) = self.monsters.iter_mut().find(|monster| monster.id == guard_id) {
                guard.behavior = MonsterBehavior::Social;
                guard.faction = Faction::Neutral;
                guard.display_glyph = Some('G');
            }
            spawned += 1;
        }
        spawned
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(MapBounds { width: 80, height: 25 })
    }
}

fn default_map_rows(bounds: MapBounds) -> Vec<String> {
    let width = usize::try_from(bounds.width).unwrap_or(0);
    let height = usize::try_from(bounds.height).unwrap_or(0);
    let row = ".".repeat(width);
    vec![row; height]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn random_cardinal_direction<R: RandomSource>(rng: &mut R) -> Direction {
    match rng.range_inclusive_i32(0, 3) {
        0 => Direction::North,
        1 => Direction::South,
        2 => Direction::East,
        _ => Direction::West,
    }
}

const LEGACY_INVENTORY_KEYMAP: [char; 16] =
    ['-', 'a', 'b', 'c', 'f', 'g', 'h', 'i', 'm', 'n', 'o', 'q', 'r', 'u', 'v', 'w'];

const INVENTORY_SLOT_COUNT: usize = 16;
const SLOT_UP_IN_AIR: usize = 0;
const SLOT_READY_HAND: usize = 1;
const SLOT_WEAPON_HAND: usize = 2;
const SLOT_LEFT_SHOULDER: usize = 3;
const SLOT_RIGHT_SHOULDER: usize = 4;
const SLOT_BELT_1: usize = 5;
const SLOT_BELT_2: usize = 6;
const SLOT_BELT_3: usize = 7;
const SLOT_SHIELD: usize = 8;
const SLOT_ARMOR: usize = 9;
const SLOT_BOOTS: usize = 10;
const SLOT_CLOAK: usize = 11;
const SLOT_RING_1: usize = 12;
const SLOT_RING_2: usize = 13;
const SLOT_RING_3: usize = 14;
const SLOT_RING_4: usize = 15;

pub fn legacy_inventory_key_to_slot(key: char) -> Option<usize> {
    LEGACY_INVENTORY_KEYMAP.iter().position(|candidate| *candidate == key.to_ascii_lowercase())
}

pub fn legacy_inventory_slot_to_key(slot: usize) -> Option<char> {
    LEGACY_INVENTORY_KEYMAP.get(slot).copied()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Command {
    Wait,
    Move(Direction),
    Attack(Direction),
    Pickup,
    Drop { slot: usize },
    Legacy { token: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Event {
    Waited,
    Moved { from: Position, to: Position },
    MoveBlocked { target: Position },
    AttackMissed { target: Position },
    Attacked { monster_id: u64, damage: i32, remaining_hp: i32 },
    MonsterMoved { monster_id: u64, from: Position, to: Position },
    MonsterAttacked { monster_id: u64, damage: i32, remaining_hp: i32 },
    MonsterDefeated { monster_id: u64 },
    PlayerDefeated,
    VictoryAchieved,
    CommandIgnoredTerminal { status: SessionStatus },
    PickedUp { item_id: u32, name: String },
    Dropped { item_id: u32, name: String },
    InventoryFull { capacity: usize },
    NoItemToPickUp,
    InvalidDropSlot { slot: usize },
    LegacyHandled { token: String, note: String, fully_modeled: bool },
    ConfirmationRequired { token: String },
    EconomyUpdated { source: String, gold: i32, bank_gold: i32 },
    DialogueAdvanced { speaker: String, quest_state: LegacyQuestState },
    QuestAdvanced { state: LegacyQuestState, steps_completed: u8 },
    ProgressionUpdated { guild_rank: u8, priest_rank: u8, alignment: Alignment },
    EndingResolved { ending: EndingKind, score: i64, high_score_eligible: bool },
    ActionPointsSpent { cost: u16, budget_per_turn: u16, total_spent: u64 },
    StatusTick { effect_id: String, magnitude: i32, remaining_turns: u32 },
    StatusExpired { effect_id: String },
    TurnAdvanced { turn: u64, minutes: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Outcome {
    pub turn: u64,
    pub minutes: u64,
    pub status: SessionStatus,
    pub events: Vec<Event>,
}

pub trait RandomSource {
    fn range_inclusive_i32(&mut self, min: i32, max: i32) -> i32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub fn seeded(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 32) as u32
    }
}

impl Default for DeterministicRng {
    fn default() -> Self {
        Self::seeded(0xD1CE_5EED)
    }
}

impl RandomSource for DeterministicRng {
    fn range_inclusive_i32(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        let span = (max - min + 1) as u32;
        min + (self.next_u32() % span) as i32
    }
}

pub fn step<R: RandomSource>(state: &mut GameState, command: Command, rng: &mut R) -> Outcome {
    let mut events = Vec::new();
    let mut turn_minutes =
        estimate_turn_minutes(&command, state.world_mode, state.options.searchnum);
    let mut command_for_accounting = command.clone();
    let mut bonus_minutes = 0u64;
    let mut freeze_world_progression = false;
    let mut command_consumed = false;

    if state.is_terminal() {
        events.push(Event::CommandIgnoredTerminal { status: state.status });
        return Outcome {
            turn: state.clock.turn,
            minutes: state.clock.minutes,
            status: state.status,
            events,
        };
    }

    let mode_policies = core::mode::policy_set_for(state.mode);
    core::mode::apply_before_command(mode_policies, state, &command, &mut events);

    sync_pack_order(state);
    sync_progression_tracks_from_legacy(&mut state.progression);
    sync_wizard_flag_with_legacy_bits(state);
    sync_spellbook_state(state);
    state.transient_projectile_path.clear();
    state.transient_projectile_impact = None;
    state.scheduler.player_phase = state.scheduler.player_phase.saturating_add(1);

    if let Some(wizard_resolution) =
        resolve_pending_wizard_interaction(state, &command, &mut events, &mut bonus_minutes)
    {
        command_consumed = true;
        freeze_world_progression = wizard_resolution.freeze_world_progression;
        command_for_accounting = wizard_resolution.command_for_accounting;
        turn_minutes = wizard_resolution.turn_minutes;
    }

    if !command_consumed {
        if let Some(quit_resolution) =
            resolve_pending_quit_interaction(state, &command, &mut events)
        {
            command_consumed = true;
            freeze_world_progression = quit_resolution.freeze_world_progression;
            command_for_accounting = quit_resolution.command_for_accounting;
            turn_minutes = quit_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(talk_resolution) =
            resolve_pending_talk_direction_interaction(state, &command, &mut events)
        {
            command_consumed = true;
            freeze_world_progression = talk_resolution.freeze_world_progression;
            command_for_accounting = talk_resolution.command_for_accounting;
            turn_minutes = talk_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(spell_resolution) =
            resolve_pending_spell_interaction(state, &command, &mut events)
        {
            command_consumed = true;
            freeze_world_progression = spell_resolution.freeze_world_progression;
            command_for_accounting = spell_resolution.command_for_accounting;
            turn_minutes = spell_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(activation_resolution) =
            resolve_pending_activation_interaction(state, &command, &mut events)
        {
            command_consumed = true;
            freeze_world_progression = activation_resolution.freeze_world_progression;
            command_for_accounting = activation_resolution.command_for_accounting;
            turn_minutes = activation_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(inventory_resolution) =
            resolve_pending_inventory_interaction(state, &command, &mut events)
        {
            command_consumed = true;
            freeze_world_progression = inventory_resolution.freeze_world_progression;
            command_for_accounting = inventory_resolution.command_for_accounting;
            turn_minutes = inventory_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(item_prompt_resolution) = resolve_pending_item_prompt_interaction(
            state,
            &command,
            &mut events,
            rng,
            &mut bonus_minutes,
        ) {
            command_consumed = true;
            freeze_world_progression = item_prompt_resolution.freeze_world_progression;
            command_for_accounting = item_prompt_resolution.command_for_accounting;
            turn_minutes = item_prompt_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        if let Some(targeting_resolution) =
            resolve_pending_targeting_interaction(state, &command, &mut events, rng)
        {
            command_consumed = true;
            freeze_world_progression = targeting_resolution.freeze_world_progression;
            command_for_accounting = targeting_resolution.command_for_accounting;
            turn_minutes = targeting_resolution.turn_minutes;
        }
    }

    if !command_consumed {
        let interaction_consumed = resolve_pending_site_interaction(state, &command, &mut events);
        if interaction_consumed {
            command_consumed = true;
            // Site menus are modal interactions; keep world/time frozen until
            // the player exits the interaction, matching prompt-driven behavior.
            freeze_world_progression = true;
            turn_minutes = 0;
            command_for_accounting = Command::Legacy { token: "F".to_string() };
        }
    }

    if !command_consumed {
        match command {
            Command::Wait => {
                state.log.push("You wait.".to_string());
                events.push(Event::Waited);
            }
            Command::Move(direction) => {
                let from = state.player.position;
                let move_direction =
                    apply_lost_navigation_direction(state, direction, rng, &mut events);
                let target = from.offset(move_direction);
                if try_bump_attack_on_move(state, move_direction, rng, &mut events) {
                    // Legacy parity: walking into an occupied tile resolves melee instead of block.
                } else {
                    let burden_limit = (effective_inventory_capacity(state) as i32) * 12;
                    let overburdened = state.carry_burden > burden_limit;
                    if overburdened {
                        state.log.push("You are too burdened to move.".to_string());
                        events.push(Event::MoveBlocked { target: from });
                    } else if !state.tile_is_walkable(target) {
                        if !try_bump_interaction_on_blocked_move(
                            state,
                            from,
                            target,
                            rng,
                            &mut events,
                            &mut bonus_minutes,
                        ) {
                            state.log.push("Movement blocked.".to_string());
                            events.push(Event::MoveBlocked { target });
                        }
                    } else {
                        state.player.position = target;
                        state.log.push("You move.".to_string());
                        events.push(Event::Moved { from, to: target });
                        apply_post_move_effects(state, rng, &mut events, &mut bonus_minutes);
                    }
                }
            }
            Command::Attack(direction) => {
                resolve_attack_command(state, direction, rng, &mut events);
            }
            Command::Pickup => {
                try_pickup_at_player(state, &mut events);
            }
            Command::Drop { slot } => {
                if slot >= state.player.inventory.len() {
                    state.log.push("Invalid inventory slot.".to_string());
                    events.push(Event::InvalidDropSlot { slot });
                } else {
                    let item = state.player.inventory.remove(slot);
                    unequip_item_id(&mut state.player.equipment, item.id);
                    remove_item_from_pack_order(state, item.id);
                    state.carry_burden =
                        state.carry_burden.saturating_sub(item_burden(&item)).max(0);
                    state.log.push(format!("Dropped {}.", item.name));
                    events.push(Event::Dropped { item_id: item.id, name: item.name.clone() });
                    state.ground_items.push(GroundItem { position: state.player.position, item });
                }
            }
            Command::Legacy { token } => {
                apply_legacy_command(state, &token, &mut events, rng, &mut bonus_minutes);
            }
        }
    }

    if !freeze_world_progression && let Command::Legacy { token } = &command_for_accounting {
        let trimmed = token.trim();
        let opened_wizard_prompt = state.pending_wizard_interaction.is_some()
            && matches!(trimmed, "^g" | "^x" | "^k" | "#" | "z");
        let opened_spell_prompt = state.pending_spell_interaction.is_some() && trimmed == "m";
        let opened_activation_prompt =
            state.pending_activation_interaction.is_some() && trimmed == "a";
        let opened_quit_prompt = state.pending_quit_interaction.is_some() && trimmed == "Q";
        let opened_talk_prompt =
            state.pending_talk_direction.is_some() && matches!(trimmed, "t" | "T");
        let opened_inventory_prompt =
            state.pending_inventory_interaction.is_some() && matches!(trimmed, "i" | "I");
        let opened_item_prompt = state.pending_item_prompt.is_some()
            && matches!(trimmed, "q" | "r" | "e" | "d" | "f" | "z" | "A" | "C" | "G");
        let opened_targeting_prompt =
            state.pending_targeting_interaction.is_some() && matches!(trimmed, "f" | "m" | "z");
        let non_advancing_wizard_token = matches!(trimmed, "^g" | "^w" | "^k" | "#");
        if opened_wizard_prompt
            || opened_spell_prompt
            || opened_activation_prompt
            || opened_quit_prompt
            || opened_talk_prompt
            || opened_inventory_prompt
            || opened_item_prompt
            || opened_targeting_prompt
            || non_advancing_wizard_token
        {
            freeze_world_progression = true;
            turn_minutes = 0;
            command_for_accounting = Command::Legacy { token: "F".to_string() };
        }
    }

    if !freeze_world_progression && state.status == SessionStatus::InProgress {
        apply_environment_effects(state, rng, &mut events);
    }

    if !freeze_world_progression && state.status == SessionStatus::InProgress {
        apply_status_effects(state, &mut events);
    }

    if !freeze_world_progression && state.status == SessionStatus::InProgress && !state.ai_paused {
        run_monster_turn(state, rng, &mut events);
    }

    if !freeze_world_progression && state.status == SessionStatus::InProgress {
        resolve_arena_round(state, &mut events);
    }

    if !freeze_world_progression {
        update_progression_from_combat(state, &mut events);
        apply_action_points(state, &command_for_accounting, &mut events);
        resolve_session_outcome(state, &mut events);
        advance_time(state, turn_minutes.saturating_add(bonus_minutes), &mut events);
    } else {
        sync_wizard_flag_with_legacy_bits(state);
    }

    sync_progression_tracks_from_legacy(&mut state.progression);
    sync_legacy_progression_from_tracks(&mut state.progression);
    sync_pack_order(state);
    core::mode::apply_after_command(mode_policies, state, &command_for_accounting, &mut events);

    Outcome { turn: state.clock.turn, minutes: state.clock.minutes, status: state.status, events }
}

fn estimate_turn_minutes(command: &Command, world_mode: WorldMode, searchnum: u8) -> u64 {
    match command {
        Command::Wait => 6,
        Command::Move(_) => match world_mode {
            WorldMode::DungeonCity => 5,
            WorldMode::Countryside => 60,
        },
        Command::Attack(_) => 10,
        Command::Pickup => 10,
        Command::Drop { .. } => 5,
        Command::Legacy { token } => estimate_legacy_turn_minutes(token, world_mode, searchnum),
    }
}

fn estimate_legacy_turn_minutes(token: &str, world_mode: WorldMode, searchnum: u8) -> u64 {
    match token.trim() {
        "." => 10,
        "," => 60,
        "M" => 45,
        "a" => 0,
        "A" => 10,
        "c" => 2,
        "e" => 30,
        "f" => 5,
        "p" => 20,
        "r" => 30,
        "v" => 10,
        "z" | "Z" => 10,
        "T" => 30,
        "H" => 180,
        "s" => match world_mode {
            WorldMode::Countryside => 60,
            WorldMode::DungeonCity => 20,
        },
        ">" => match world_mode {
            WorldMode::Countryside => 30,
            WorldMode::DungeonCity => 0,
        },
        "<" => 0,
        "g" => 10,
        "d" => 5,
        "q" => 10,
        "m" => 20,
        "t" => 10,
        "b" | "n" | "u" | "y" => 5,
        "G" => 15,
        "D" => 30,
        "F" => 0,
        "S" => 0,
        "Q" => 0,
        "i" | "I" => 0,
        "^p" | "^o" | "^r" | "^l" | "?" | "/" | "P" | "V" => 0,
        "^g" | "^w" | "^k" | "#" => 0,
        "^x" => 5,
        "^f" | "^i" | "C" | "R" => 5,
        "O" => 0,
        "@" => 5,
        _ => {
            let _ = searchnum;
            0
        }
    }
}

fn apply_legacy_command<R: RandomSource>(
    state: &mut GameState,
    token: &str,
    events: &mut Vec<Event>,
    rng: &mut R,
    bonus_minutes: &mut u64,
) {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        let note = "empty legacy token ignored".to_string();
        events.push(Event::LegacyHandled {
            token: trimmed.to_string(),
            note: note.clone(),
            fully_modeled: true,
        });
        push_timeline_line(state, note);
        return;
    }

    if state.options.confirm && requires_confirmation(trimmed) {
        let confirmed = state.pending_confirmation.as_deref() == Some(trimmed);
        if !confirmed {
            state.pending_confirmation = Some(trimmed.to_string());
            let note = "confirmation required; repeat command to proceed".to_string();
            events.push(Event::ConfirmationRequired { token: trimmed.to_string() });
            events.push(Event::LegacyHandled {
                token: trimmed.to_string(),
                note: note.clone(),
                fully_modeled: true,
            });
            push_timeline_line(state, note);
            return;
        }
        state.pending_confirmation = None;
    } else if state.pending_confirmation.is_some() {
        state.pending_confirmation = None;
    }

    let (note, fully_modeled) = match trimmed {
        "." | "@" => {
            state.log.push("You wait.".to_string());
            events.push(Event::Waited);
            ("wait action resolved".to_string(), true)
        }
        "," => {
            state.log.push("You rest for an extended period.".to_string());
            events.push(Event::Waited);
            if state.player.stats.hp < state.player.stats.max_hp {
                state.player.stats.hp += 1;
            }
            ("sleep resolved with minor recovery".to_string(), true)
        }
        "<" => {
            state.topology.last_city_position = Some(state.player.position);
            ensure_country_bootstrap(state);
            state.activate_country_view();
            ensure_known_site(state, state.player.position);
            state.topology.country_region_id = state.topology.country_region_id.wrapping_add(1);
            let fallback = Position { x: state.bounds.width / 2, y: state.bounds.height / 2 };
            let target = state
                .topology
                .last_country_position
                .or(state.topology.country_rampart_position)
                .unwrap_or(fallback);
            if state.tile_is_walkable(target) {
                state.player.position = target;
            }
            ("entered countryside mode".to_string(), true)
        }
        ">" => resolve_enter_command(state, events),
        "M" => {
            if state.world_mode == WorldMode::DungeonCity
                && state
                    .tile_site_at(state.player.position)
                    .is_some_and(|site| (site.flags & TILE_FLAG_NO_CITY_MOVE) != 0)
            {
                ("cannot use city movement from this location (NOCITYMOVE)".to_string(), true)
            } else {
                if state.known_sites.is_empty() {
                    let center = Position { x: state.bounds.width / 2, y: state.bounds.height / 2 };
                    state.known_sites.push(center);
                }
                let idx = (state.clock.turn as usize) % state.known_sites.len();
                state.player.position = state.known_sites[idx];
                if state.gold >= 5 {
                    state.gold -= 5;
                }
                events.push(Event::EconomyUpdated {
                    source: "fast_travel".to_string(),
                    gold: state.gold,
                    bank_gold: state.bank_gold,
                });
                ("fast travel to discovered site".to_string(), true)
            }
        }
        "H" => {
            if state.world_mode == WorldMode::Countryside {
                let (note, bonus) = resolve_countryside_hunt(state, rng, events);
                *bonus_minutes = bonus_minutes.saturating_add(bonus);
                (note, true)
            } else {
                state.food += 1;
                let item_name = format!("foraged ration {}", state.next_item_id);
                state.place_item(item_name, state.player.position);
                ("hunt completed; food stock increased".to_string(), true)
            }
        }
        "s" => {
            if state.world_mode == WorldMode::Countryside {
                let discovered = Position {
                    x: (state.player.position.x + i32::from(state.options.searchnum))
                        .clamp(0, state.bounds.width.saturating_sub(1)),
                    y: (state.player.position.y + 1)
                        .clamp(0, state.bounds.height.saturating_sub(1)),
                };
                ensure_known_site(state, discovered);
                let bonus = apply_countryside_search(state, rng, events);
                *bonus_minutes = bonus_minutes.saturating_add(bonus);
                ("countryside search discovered a new trace".to_string(), true)
            } else {
                let loops = state.options.searchnum.max(1);
                for i in 0..loops {
                    let item_name = format!("cache provision {}-{}", state.next_item_id, i + 1);
                    state.place_item(item_name, state.player.position);
                }
                ("search resolved and revealed hidden cache(s)".to_string(), true)
            }
        }
        "q" => begin_item_prompt(
            state,
            ItemPromptContext::Quaff,
            ItemPromptFilter::Families(vec![ItemFamily::Potion]),
            "Quaff which potion?".to_string(),
        ),
        "r" => begin_item_prompt(
            state,
            ItemPromptContext::Read,
            ItemPromptFilter::Families(vec![ItemFamily::Scroll]),
            "Read which scroll?".to_string(),
        ),
        "m" => begin_spell_interaction(state),
        "a" => begin_activation_interaction(state),
        "A" => begin_item_prompt(
            state,
            ItemPromptContext::ActivateArtifact,
            ItemPromptFilter::Families(vec![ItemFamily::Artifact]),
            "Activate which artifact?".to_string(),
        ),
        "t" => begin_talk_direction_interaction(state, TalkDirectionInteraction::Talk),
        "G" => {
            if state.player.inventory.is_empty() {
                if state.gold >= 10 {
                    state.gold -= 10;
                    state.progression.deity_favor += 3;
                    state.progression.law_chaos_score += 2;
                }
                events.push(Event::EconomyUpdated {
                    source: "donation".to_string(),
                    gold: state.gold,
                    bank_gold: state.bank_gold,
                });
                ("gift resolved; favor and alignment adjusted".to_string(), true)
            } else {
                begin_item_prompt(
                    state,
                    ItemPromptContext::Give,
                    ItemPromptFilter::Any,
                    "Give which item?".to_string(),
                )
            }
        }
        "D" => disarm_adjacent_trap(state, events),
        "F" => {
            rotate_combat_sequence(state);
            ("combat sequence preset updated".to_string(), true)
        }
        "O" => {
            cycle_runtime_options(state);
            ("runtime options cycled".to_string(), true)
        }
        "d" => begin_item_prompt(
            state,
            ItemPromptContext::Drop,
            ItemPromptFilter::Any,
            "Drop which item?".to_string(),
        ),
        "e" => {
            if state.player.inventory.iter().any(|item| item.family == ItemFamily::Food) {
                begin_item_prompt(
                    state,
                    ItemPromptContext::Eat,
                    ItemPromptFilter::Families(vec![ItemFamily::Food]),
                    "Eat which item?".to_string(),
                )
            } else if state.food > 0 {
                state.food -= 1;
                state.player.stats.hp = (state.player.stats.hp + 2).min(state.player.stats.max_hp);
                ("ate rations and recovered health".to_string(), true)
            } else {
                ("eat requested but no food available".to_string(), true)
            }
        }
        "i" => begin_inventory_interaction(state, false),
        "I" => begin_inventory_interaction(state, true),
        "?" => (
            format!(
                "context help displayed for {:?} (verbosity {:?})",
                state.world_mode, state.options.verbosity
            ),
            true,
        ),
        "/" => {
            let marker = if has_adjacent_monster(state) {
                "monster-nearby"
            } else if ground_item_index_at(state, state.player.position).is_some() {
                "item-on-tile"
            } else if state
                .traps
                .iter()
                .any(|trap| trap.armed && trap.position == state.player.position)
            {
                "trap-signature"
            } else {
                "terrain"
            };
            (format!("identify resolved: {marker}"), true)
        }
        "x" => {
            let trap_here = state
                .traps
                .iter()
                .find(|trap| trap.armed && trap.position == state.player.position)
                .map(|trap| trap.id);
            (
                format!(
                    "examine: pos=({}, {}), trap={:?}, known_sites={}",
                    state.player.position.x,
                    state.player.position.y,
                    trap_here,
                    state.known_sites.len()
                ),
                true,
            )
        }
        "C" => begin_item_prompt(
            state,
            ItemPromptContext::CallItem,
            ItemPromptFilter::Any,
            "Name which item?".to_string(),
        ),
        "R" => {
            state.player_name = format!("{}-{}", state.player_name, state.clock.turn + 1);
            ("character renamed".to_string(), true)
        }
        "P" => ("public license information displayed".to_string(), true),
        "V" => ("version information displayed".to_string(), true),
        "^p" | "^o" => ("previous message replayed".to_string(), true),
        "^r" | "^l" => ("redraw command acknowledged".to_string(), true),
        "^f" => {
            state.status_effects.retain(|effect| effect.id != "shadow_form");
            ("shadow form aborted".to_string(), true)
        }
        "^g" => {
            if state.wizard.enabled || has_legacy_status_flag(state, LEGACY_STATUS_CHEATED) {
                ("You're already in wizard mode!".to_string(), true)
            } else {
                begin_wizard_interaction(
                    state,
                    WizardInteraction::EnterWizardConfirm { via_backdoor: false },
                    "You just asked to enter wizard mode. [y/n]".to_string(),
                )
            }
        }
        "^w" => {
            if state.wizard.enabled {
                reveal_map_for_wizard(state);
                ("wizard map revealed the full current environment".to_string(), true)
            } else {
                ("wizard-only command denied".to_string(), true)
            }
        }
        "^x" => {
            if state.wizard.enabled || state.progression.guild_rank >= 4 {
                begin_wizard_interaction(
                    state,
                    WizardInteraction::WishTextEntry { blessing: 1 },
                    "What do you wish for?".to_string(),
                )
            } else {
                ("wish denied: insufficient privileges".to_string(), true)
            }
        }
        "^k" => {
            if state.wizard.enabled {
                begin_wizard_interaction(
                    state,
                    WizardInteraction::StatusFlagActionSelect,
                    "Set or Reset or Forget it [s/r/ESCAPE]:".to_string(),
                )
            } else {
                ("wizard-only command denied".to_string(), true)
            }
        }
        "#" => {
            if state.wizard.enabled {
                begin_wizard_interaction(
                    state,
                    WizardInteraction::StatEditorSelect { slot: 1 },
                    "Stat editor active: j/k or </> to move, SPACE to edit, ESCAPE to quit."
                        .to_string(),
                )
            } else {
                ("wizard-only command denied".to_string(), true)
            }
        }
        "o" => (apply_door_interaction(state, false), true),
        "c" => (apply_door_interaction(state, true), true),
        "E" => ("mount/dismount action resolved".to_string(), true),
        "p" => {
            if has_adjacent_monster(state) {
                state.gold += 20;
                state.progression.law_chaos_score -= 2;
                events.push(Event::EconomyUpdated {
                    source: "pickpocket".to_string(),
                    gold: state.gold,
                    bank_gold: state.bank_gold,
                });
                ("pickpocket succeeded".to_string(), true)
            } else {
                state.legal_heat += 1;
                ("pickpocket failed; legal heat increased".to_string(), true)
            }
        }
        "f" => begin_item_prompt(
            state,
            ItemPromptContext::FireThrow,
            ItemPromptFilter::Any,
            "Fire/Throw --".to_string(),
        ),
        "v" => {
            let target = Position {
                x: (state.player.position.x + 2).clamp(0, state.bounds.width.saturating_sub(1)),
                y: state.player.position.y,
            };
            if state.tile_is_walkable(target) && !is_occupied(state, target) {
                state.player.position = target;
            }
            ("vault movement resolved".to_string(), true)
        }
        "z" => begin_item_prompt(
            state,
            ItemPromptContext::ZapStick,
            ItemPromptFilter::Families(vec![ItemFamily::Stick]),
            "Zap which stick?".to_string(),
        ),
        "T" => begin_talk_direction_interaction(state, TalkDirectionInteraction::Tunnel),
        "Z" => {
            if trimmed == "Z" && state.environment == LegacyEnvironment::City {
                begin_wizard_interaction(
                    state,
                    WizardInteraction::BashDirectionSelect,
                    "Bashing -- choose direction (hjklyubn or keypad; ESCAPE aborts).".to_string(),
                )
            } else {
                apply_destructive_action(state)
            }
        }
        "Q" => begin_quit_interaction(state),
        "S" => (
            "save-and-quit command is frontend-driven; no core world mutation applied".to_string(),
            false,
        ),
        "b" => {
            let target =
                Position { x: state.player.position.x - 1, y: state.player.position.y + 1 };
            if state.tile_is_walkable(target) && !is_occupied(state, target) {
                state.player.position = target;
            }
            ("diagonal movement `b` resolved".to_string(), true)
        }
        "n" => {
            let target =
                Position { x: state.player.position.x + 1, y: state.player.position.y + 1 };
            if state.tile_is_walkable(target) && !is_occupied(state, target) {
                state.player.position = target;
            }
            ("diagonal movement `n` resolved".to_string(), true)
        }
        "u" => {
            let target =
                Position { x: state.player.position.x + 1, y: state.player.position.y - 1 };
            if state.tile_is_walkable(target) && !is_occupied(state, target) {
                state.player.position = target;
            }
            ("diagonal movement `u` resolved".to_string(), true)
        }
        "y" => {
            let target =
                Position { x: state.player.position.x - 1, y: state.player.position.y - 1 };
            if state.tile_is_walkable(target) && !is_occupied(state, target) {
                state.player.position = target;
            }
            ("diagonal movement `y` resolved".to_string(), true)
        }
        _ => (format!("unsupported legacy command `{trimmed}`"), false),
    };

    let class = classify_note_against_active_interactions(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled { token: trimmed.to_string(), note, fully_modeled });
}

fn resolve_enter_command(state: &mut GameState, events: &mut Vec<Event>) -> (String, bool) {
    if state.world_mode == WorldMode::Countryside {
        return resolve_enter_country_site(state);
    }
    resolve_enter_local_site(state, events)
}

fn ensure_country_bootstrap(state: &mut GameState) {
    if !state.country_map_rows.is_empty() && !state.country_grid.cells.is_empty() {
        return;
    }

    if state.country_map_rows.is_empty() {
        let width = state.bounds.width.max(1);
        let height = state.bounds.height.max(1);
        state.country_map_rows = default_map_rows(MapBounds { width, height });
    }

    let width = state
        .country_map_rows
        .first()
        .map(|row| row.chars().count())
        .unwrap_or(usize::try_from(state.bounds.width.max(1)).unwrap_or(1))
        .max(1);
    for row in &mut state.country_map_rows {
        let count = row.chars().count();
        if count < width {
            row.push_str(&".".repeat(width - count));
        } else if count > width {
            *row = row.chars().take(width).collect();
        }
    }

    let height = state.country_map_rows.len().max(1);
    if state.country_map_rows.len() < height {
        state
            .country_map_rows
            .extend((state.country_map_rows.len()..height).map(|_| ".".repeat(width)));
    }

    let px = usize::try_from(
        state.player.position.x.clamp(0, i32::try_from(width.saturating_sub(1)).unwrap_or(0)),
    )
    .unwrap_or(0);
    let py = usize::try_from(
        state.player.position.y.clamp(0, i32::try_from(height.saturating_sub(1)).unwrap_or(0)),
    )
    .unwrap_or(0);
    if let Some(row) = state.country_map_rows.get_mut(py) {
        let mut chars: Vec<char> = row.chars().collect();
        if let Some(slot) = chars.get_mut(px) {
            *slot = 'O';
            *row = chars.into_iter().collect();
        }
    }

    let width_i32 = i32::try_from(width).unwrap_or(1);
    let height_i32 = i32::try_from(height).unwrap_or(1);
    let mut cells = Vec::with_capacity(width.saturating_mul(height));
    for y in 0..height_i32 {
        for x in 0..width_i32 {
            let pos = Position { x, y };
            let cell = fallback_country_cell_from_rows(state, pos).unwrap_or(CountryCell {
                glyph: '.',
                base_terrain: CountryTerrainKind::Road,
                current_terrain: CountryTerrainKind::Road,
                aux: 0,
                status: 0,
            });
            cells.push(cell);
        }
    }
    state.country_grid = CountryGrid { width: width_i32, height: height_i32, cells };

    let site_len = width.saturating_mul(height);
    if state.country_site_grid.len() != site_len {
        state.country_site_grid = vec![TileSiteCell::default(); site_len];
    }

    if state.topology.country_rampart_position.is_none() {
        state.topology.country_rampart_position =
            Some(Position { x: i32::try_from(px).unwrap_or(0), y: i32::try_from(py).unwrap_or(0) });
    }
}

fn resolve_enter_country_site(state: &mut GameState) -> (String, bool) {
    let origin = state.player.position;
    let Some(cell) = state
        .country_cell_at(origin)
        .cloned()
        .or_else(|| fallback_country_cell_from_rows(state, origin))
    else {
        return ("no country cell at current position".to_string(), true);
    };
    state.topology.last_country_position = Some(origin);

    match cell.base_terrain {
        CountryTerrainKind::City => {
            state.activate_city_view();
            state.topology.city_site_id = state.topology.city_site_id.wrapping_add(1);
            state.topology.dungeon_level = 0;
            if let Some(city_pos) = state.topology.last_city_position
                && let Some(spawn) = sanitize_spawn(state, city_pos)
            {
                state.player.position = spawn;
            }
            ("entered Rampart city from countryside".to_string(), true)
        }
        CountryTerrainKind::Village => {
            let Some((map_id, spawn, village_name)) = village_map_for_aux(cell.aux) else {
                return ("village entry failed: unknown village identifier".to_string(), true);
            };
            if state.activate_site_map_by_id(map_id, Some(spawn)) {
                state.topology.dungeon_level = 0;
                (format!("entered village {village_name}"), true)
            } else {
                ("village map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::Temple => {
            if state.activate_site_map_by_id(16, Some(Position { x: 32, y: 15 })) {
                state.topology.dungeon_level = 0;
                (format!("entered temple {}", cell.aux), true)
            } else {
                ("temple map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::Castle => {
            if state.activate_site_map_by_id(5, Some(Position { x: 32, y: 2 })) {
                state.topology.dungeon_level = 0;
                remap_active_site_aux(state, SITE_AUX_SERVICE_PALACE, SITE_AUX_SERVICE_CASTLE);
                ("entered the royal court".to_string(), true)
            } else {
                ("court map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::Palace => {
            if state.activate_site_map_by_id(5, Some(Position { x: 32, y: 2 })) {
                state.topology.dungeon_level = 0;
                state.environment = LegacyEnvironment::Palace;
                remap_active_site_aux(state, SITE_AUX_SERVICE_CASTLE, SITE_AUX_SERVICE_PALACE);
                ("entered the imperial palace".to_string(), true)
            } else {
                ("court map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::Caves => {
            if state.activate_site_map_by_id(2, Some(Position { x: 2, y: 2 })) {
                state.topology.dungeon_level = 0;
                ("entered the caves".to_string(), true)
            } else {
                ("caves map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::Volcano => {
            if state.activate_site_map_by_id(4, Some(Position { x: 32, y: 8 })) {
                state.topology.dungeon_level = 0;
                ("entered the volcano".to_string(), true)
            } else {
                ("volcano map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::DragonLair => {
            if state.activate_site_map_by_id(6, Some(Position { x: 8, y: 0 })) {
                state.topology.dungeon_level = 0;
                ("entered dragon lair".to_string(), true)
            } else {
                ("dragon lair map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::StarPeak => {
            if state.activate_site_map_by_id(13, Some(Position { x: 2, y: 9 })) {
                state.topology.dungeon_level = 0;
                ("entered Star Peak".to_string(), true)
            } else {
                ("star peak map missing from loaded content".to_string(), true)
            }
        }
        CountryTerrainKind::MagicIsle => {
            if state.activate_site_map_by_id(11, Some(Position { x: 62, y: 14 })) {
                state.topology.dungeon_level = 0;
                ("entered Magic Isle".to_string(), true)
            } else {
                ("magic isle map missing from loaded content".to_string(), true)
            }
        }
        _ => ("there is nothing to enter here".to_string(), true),
    }
}

fn remap_active_site_aux(state: &mut GameState, from_aux: i32, to_aux: i32) {
    for cell in &mut state.site_grid {
        if cell.aux == from_aux {
            cell.aux = to_aux;
        }
    }
}

fn open_arena_gateway_exit_target(state: &GameState, pos: Position) -> Option<Position> {
    if state.environment != LegacyEnvironment::Arena {
        return None;
    }
    let site = state.tile_site_at(pos)?;
    if (site.flags & TILE_FLAG_PORTCULLIS) == 0 || (site.flags & TILE_FLAG_BLOCK_MOVE) != 0 {
        return None;
    }
    let west = pos.offset(Direction::West);
    if !state.bounds.contains(west) {
        return Some(pos);
    }
    state.tile_site_at(west).filter(|neighbor| neighbor.aux == SITE_AUX_EXIT_ARENA).map(|_| west)
}

fn apply_garden_local_interaction(
    state: &mut GameState,
    events: &mut Vec<Event>,
) -> Option<String> {
    let site_id = state.tile_site_at(state.player.position)?.site_id;
    let note = match site_id {
        CITY_SITE_GARDEN => {
            "The Garden paths are quiet. Rumors point to a sewer entrance beneath the hedges."
                .to_string()
        }
        CITY_SITE_CEMETARY => {
            "The cemetary stones stand in silence. The dead are rarely at rest in Rampart."
                .to_string()
        }
        _ => return None,
    };
    events.push(Event::LegacyHandled {
        token: "local_site".to_string(),
        note: format!("local site interaction {}", site_id),
        fully_modeled: true,
    });
    Some(note)
}

fn resolve_enter_local_site(state: &mut GameState, events: &mut Vec<Event>) -> (String, bool) {
    let site_aux = state.tile_site_at(state.player.position).map(|site| site.aux).unwrap_or(0);

    if site_aux == SITE_AUX_EXIT_ARENA {
        state.activate_city_view();
        if let Some(city_pos) = state.topology.last_city_position
            && let Some(spawn) = sanitize_spawn(state, city_pos)
        {
            state.player.position = spawn;
        }
        return ("left the arena and returned to Rampart".to_string(), true);
    }

    if open_arena_gateway_exit_target(state, state.player.position).is_some() {
        state.activate_city_view();
        if let Some(city_pos) = state.topology.last_city_position
            && let Some(spawn) = sanitize_spawn(state, city_pos)
        {
            state.player.position = spawn;
        }
        return ("left the arena through the raised portcullis".to_string(), true);
    }

    if site_aux == SITE_AUX_EXIT_COUNTRYSIDE {
        state.topology.last_city_position = Some(state.player.position);
        ensure_country_bootstrap(state);
        state.activate_country_view();
        ensure_known_site(state, state.player.position);
        state.topology.country_region_id = state.topology.country_region_id.wrapping_add(1);
        let fallback = Position { x: state.bounds.width / 2, y: state.bounds.height / 2 };
        let target = state
            .topology
            .last_country_position
            .or(state.topology.country_rampart_position)
            .unwrap_or(fallback);
        if state.tile_is_walkable(target) {
            state.player.position = target;
        }
        return ("returned to countryside".to_string(), true);
    }

    if let Some(note) = apply_garden_local_interaction(state, events) {
        return (note, true);
    }

    if state.options.interactive_sites
        && let Some(kind) = interaction_kind_for_site_aux(state, site_aux)
    {
        let note = begin_site_interaction(state, kind, events, "enter");
        return (note, true);
    }

    if let Some(note) = apply_site_service(state, site_aux, events) {
        return (note, true);
    }

    if matches!(
        state.environment,
        LegacyEnvironment::City
            | LegacyEnvironment::Village
            | LegacyEnvironment::Temple
            | LegacyEnvironment::Castle
            | LegacyEnvironment::MagicIsle
            | LegacyEnvironment::StarPeak
            | LegacyEnvironment::DragonLair
            | LegacyEnvironment::Hovel
            | LegacyEnvironment::House
            | LegacyEnvironment::Mansion
    ) {
        return ("there is nothing to enter on this tile".to_string(), true);
    }

    state.topology.dungeon_level = state.topology.dungeon_level.saturating_add(1);
    ("descended to deeper area".to_string(), true)
}

fn arena_rival_profile(opponent: u8, arena_rank: i8) -> (String, Stats) {
    let challenger_names = [
        "Ari", "Borek", "Cira", "Dolan", "Edda", "Fenn", "Garth", "Hale", "Iris", "Jorek", "Kara",
        "Lorn", "Mira", "Nox", "Orin", "Pax",
    ];
    let roster = [
        "pencil-necked geek",
        "hornet",
        "hyena",
        "goblin",
        "grunt",
        "tove",
        "apprentice ninja",
        "salamander",
        "ant",
        "manticore",
        "spectre",
        "bandersnatch",
        "liche",
        "auto major",
        "jabberwock",
        "jotun",
    ];

    if let Some(base_name) = roster.get(usize::from(opponent)) {
        let power = i32::from(opponent);
        let alias = challenger_names[usize::from(opponent) % challenger_names.len()];
        return (
            format!("{alias} the {base_name}"),
            Stats {
                hp: 12 + power * 4,
                max_hp: 12 + power * 4,
                attack_min: 2 + power / 3,
                attack_max: 5 + power / 2,
                defense: 1 + power / 4,
            },
        );
    }

    if arena_rank > 0 {
        (
            "the arena champion".to_string(),
            Stats { hp: 120, max_hp: 120, attack_min: 12, attack_max: 20, defense: 10 },
        )
    } else {
        (
            "a veteran challenger".to_string(),
            Stats { hp: 72, max_hp: 72, attack_min: 8, attack_max: 13, defense: 7 },
        )
    }
}

fn arena_portcullis_opener_item(state: &mut GameState) -> Item {
    let item_id = state.next_item_id;
    state.next_item_id = state.next_item_id.saturating_add(1);
    let mut item = instantiate_item_from_name(item_id, "disposeable garage door opener");
    item.known = true;
    if item.family == ItemFamily::Unknown {
        item.family = ItemFamily::Thing;
    }
    if item.usef.is_empty() {
        item.usef = "I_RAISE_PORTCULLIS".to_string();
    }
    item
}

fn arm_arena_challenger_with_opener(state: &mut GameState, monster_id: u64) {
    let opener = arena_portcullis_opener_item(state);
    if let Some(monster) = state.monsters.iter_mut().find(|candidate| candidate.id == monster_id) {
        monster.on_death_drops.clear();
        monster.on_death_drops.push(opener);
    }
}

fn try_spawn_local_arena_rival(state: &mut GameState) -> bool {
    let spawn = [
        Position { x: state.player.position.x + 1, y: state.player.position.y + 1 },
        Position { x: state.player.position.x - 1, y: state.player.position.y + 1 },
        Position { x: state.player.position.x + 1, y: state.player.position.y - 1 },
        Position { x: state.player.position.x - 1, y: state.player.position.y - 1 },
        Position { x: state.player.position.x + 1, y: state.player.position.y },
        Position { x: state.player.position.x - 1, y: state.player.position.y },
        Position { x: state.player.position.x, y: state.player.position.y + 1 },
        Position { x: state.player.position.x, y: state.player.position.y - 1 },
    ]
    .into_iter()
    .find(|candidate| {
        state.bounds.contains(*candidate)
            && state.tile_is_walkable(*candidate)
            && !is_occupied(state, *candidate)
    });

    let Some(pos) = spawn else {
        return false;
    };
    let (name, stats) =
        arena_rival_profile(state.progression.arena_opponent, state.progression.arena_rank);
    let challenger_id = state.spawn_monster(name, pos, stats);
    arm_arena_challenger_with_opener(state, challenger_id);
    state.progression.arena_match_active = true;
    true
}

fn start_arena_challenge(state: &mut GameState) -> String {
    let city_pos_before = state.player.position;
    let (name, stats) =
        arena_rival_profile(state.progression.arena_opponent, state.progression.arena_rank);

    if state.activate_site_map_by_id(1, Some(Position { x: 2, y: 7 })) {
        let _ = drop_all_portcullises(state);
        let arena_spawn = sanitize_spawn(state, Position { x: 60, y: 7 }).or_else(|| {
            sanitize_spawn(
                state,
                Position { x: state.bounds.width - 4, y: state.bounds.height / 2 },
            )
        });
        if let Some(spawn) = arena_spawn {
            let challenger_id = state.spawn_monster(name.clone(), spawn, stats);
            arm_arena_challenger_with_opener(state, challenger_id);
            state.progression.arena_match_active = true;
            state.topology.last_city_position = Some(city_pos_before);
            return format!(
                "OK, we're arranging a match.... You have a challenger: {name}. Let the battle begin.... The portcullis slams shut."
            );
        }
        return "The games are delayed: no valid challenger spawn tile exists.".to_string();
    }

    if try_spawn_local_arena_rival(state) {
        return format!("You have a challenger: {name}.");
    }

    "The arena challenge cannot begin: no free tile for challenger.".to_string()
}

fn apply_site_service(
    state: &mut GameState,
    site_aux: i32,
    events: &mut Vec<Event>,
) -> Option<String> {
    let kind = interaction_kind_for_site_aux(state, site_aux)?;
    let choice = match site_aux {
        SITE_AUX_SERVICE_SHOP => {
            if state.gold >= 12 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_ARMORER => {
            if state.gold >= 70 {
                1
            } else if state.gold >= 65 {
                2
            } else if state.gold >= 30 {
                3
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_CLUB => {
            if state.legal_heat > 0 && state.gold >= 20 {
                2
            } else if state.gold >= 20 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_GYM => {
            if state.gold >= 35 {
                2
            } else if state.gold >= 30 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_HEALER => {
            if state.player.stats.hp < state.player.stats.max_hp && state.gold >= 18 {
                1
            } else if state.status_effects.iter().any(|effect| effect.id == "poison")
                && state.gold >= 25
            {
                2
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_CASINO => {
            if state.gold >= 25 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_COMMANDANT => {
            if state.gold >= 20 {
                1
            } else if state.legal_heat > 0 {
                2
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_DINER => {
            if state.food <= 3 && state.gold >= 8 {
                1
            } else if state.gold >= 6 {
                2
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_CRAPS => {
            if state.gold >= 15 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_TAVERN => {
            if state.food <= 2 && state.gold >= 10 {
                2
            } else if state.gold >= 6 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_PAWN_SHOP => {
            if !state.player.inventory.is_empty() {
                2
            } else if state.gold >= 15 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_BROTHEL => {
            if state.gold >= 25 {
                1
            } else if state.gold >= 10 {
                2
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_CONDO => {
            if state.gold >= 40 {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_BANK => {
            if state.legal_heat > 0 && state.gold >= 25 {
                3
            } else if state.gold >= 80 {
                1
            } else if state.bank_gold >= 50 {
                2
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_MERC_GUILD => {
            if state.progression.guild_rank > 0
                && state.gold >= 60
                && state.monsters_defeated >= u64::from(state.progression.guild_rank) * 3
            {
                3
            } else if state.progression.quest_state == LegacyQuestState::NotStarted
                && state.gold >= 40
            {
                2
            } else if state.gold >= 40 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_THIEVES => {
            let thieves_rank = state.progression.quests.thieves.rank.max(0) as u8;
            if thieves_rank > 0 && state.gold >= 55 {
                3
            } else if state.gold >= 25 {
                2
            } else if state.gold >= 30 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_TEMPLE => {
            if state.gold >= 35 && state.progression.deity_favor >= 3 {
                3
            } else if state.gold >= 15 {
                1
            } else if state.legal_heat > 0 {
                4
            } else {
                2
            }
        }
        SITE_AUX_SERVICE_COLLEGE => {
            if state.gold >= 25 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_SORCERORS => {
            if state.gold >= 30 {
                1
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_CASTLE => {
            if matches!(
                state.progression.quest_state,
                LegacyQuestState::ArtifactRecovered
                    | LegacyQuestState::ReturnToPatron
                    | LegacyQuestState::Completed
            ) {
                3
            } else if state.legal_heat > 0 {
                1
            } else {
                2
            }
        }
        SITE_AUX_SERVICE_PALACE => {
            if state.progression.main_quest.palace_access
                && state.progression.main_quest.stage == LegacyQuestState::ArtifactRecovered
            {
                2
            } else if state.progression.main_quest.palace_access {
                1
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_ORDER => {
            if state.progression.alignment != Alignment::Lawful {
                1
            } else if state.legal_heat > 0 && state.gold >= 25 {
                2
            } else {
                3
            }
        }
        SITE_AUX_SERVICE_CHARITY => {
            if state.player.stats.hp < state.player.stats.max_hp
                || state.food <= 0
                || (state.food <= 3 && state.status_effects.is_empty())
            {
                1
            } else if !state.status_effects.is_empty() {
                2
            } else if state.legal_heat > 0 {
                3
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_MONASTERY => {
            if state.gold >= 20 {
                1
            } else if !state.player.inventory.is_empty() {
                2
            } else if state.progression.alignment != Alignment::Lawful {
                3
            } else {
                4
            }
        }
        SITE_AUX_SERVICE_ARENA => {
            if state.progression.arena_rank <= 0 {
                1
            } else if !state.progression.arena_match_active {
                1
            } else {
                2
            }
        }
        SITE_AUX_ALTAR_ODIN
        | SITE_AUX_ALTAR_SET
        | SITE_AUX_ALTAR_ATHENA
        | SITE_AUX_ALTAR_HECATE
        | SITE_AUX_ALTAR_DESTINY => {
            if state.progression.patron_deity == 0 {
                1
            } else if state.gold >= 50 && state.progression.deity_favor < 8 {
                2
            } else {
                3
            }
        }
        _ => 1,
    };

    Some(apply_site_interaction_choice(state, kind, choice, events, false))
}

fn interaction_kind_for_site_aux(state: &GameState, site_aux: i32) -> Option<SiteInteractionKind> {
    match site_aux {
        SITE_AUX_SERVICE_SHOP => Some(SiteInteractionKind::Shop),
        SITE_AUX_SERVICE_ARMORER => Some(SiteInteractionKind::Armorer),
        SITE_AUX_SERVICE_CLUB => Some(SiteInteractionKind::Club),
        SITE_AUX_SERVICE_GYM => Some(SiteInteractionKind::Gym),
        SITE_AUX_SERVICE_HEALER => Some(SiteInteractionKind::Healer),
        SITE_AUX_SERVICE_CASINO => Some(SiteInteractionKind::Casino),
        SITE_AUX_SERVICE_COMMANDANT => Some(SiteInteractionKind::Commandant),
        SITE_AUX_SERVICE_DINER => Some(SiteInteractionKind::Diner),
        SITE_AUX_SERVICE_CRAPS => Some(SiteInteractionKind::Craps),
        SITE_AUX_SERVICE_TAVERN => Some(SiteInteractionKind::Tavern),
        SITE_AUX_SERVICE_PAWN_SHOP => Some(SiteInteractionKind::PawnShop),
        SITE_AUX_SERVICE_BROTHEL => Some(SiteInteractionKind::Brothel),
        SITE_AUX_SERVICE_CONDO => Some(SiteInteractionKind::Condo),
        SITE_AUX_SERVICE_BANK => Some(SiteInteractionKind::Bank),
        SITE_AUX_SERVICE_MERC_GUILD => Some(SiteInteractionKind::MercGuild),
        SITE_AUX_SERVICE_THIEVES => Some(SiteInteractionKind::ThievesGuild),
        SITE_AUX_SERVICE_TEMPLE => Some(SiteInteractionKind::Temple),
        SITE_AUX_SERVICE_COLLEGE => Some(SiteInteractionKind::College),
        SITE_AUX_SERVICE_SORCERORS => Some(SiteInteractionKind::Sorcerors),
        SITE_AUX_SERVICE_CASTLE => {
            if state.environment == LegacyEnvironment::Palace {
                Some(SiteInteractionKind::Palace)
            } else {
                Some(SiteInteractionKind::Castle)
            }
        }
        SITE_AUX_SERVICE_PALACE => Some(SiteInteractionKind::Palace),
        SITE_AUX_SERVICE_ORDER => Some(SiteInteractionKind::Order),
        SITE_AUX_SERVICE_CHARITY => Some(SiteInteractionKind::Charity),
        SITE_AUX_SERVICE_MONASTERY => Some(SiteInteractionKind::Monastery),
        SITE_AUX_SERVICE_ARENA => Some(SiteInteractionKind::Arena),
        SITE_AUX_ALTAR_ODIN => Some(SiteInteractionKind::Altar { deity_id: DEITY_ID_ODIN }),
        SITE_AUX_ALTAR_SET => Some(SiteInteractionKind::Altar { deity_id: DEITY_ID_SET }),
        SITE_AUX_ALTAR_ATHENA => Some(SiteInteractionKind::Altar { deity_id: DEITY_ID_ATHENA }),
        SITE_AUX_ALTAR_HECATE => Some(SiteInteractionKind::Altar { deity_id: DEITY_ID_HECATE }),
        SITE_AUX_ALTAR_DESTINY => Some(SiteInteractionKind::Altar { deity_id: DEITY_ID_DESTINY }),
        _ => None,
    }
}

fn deity_name(deity_id: u8) -> &'static str {
    match deity_id {
        DEITY_ID_ODIN => "Odin",
        DEITY_ID_SET => "Set",
        DEITY_ID_ATHENA => "Athena",
        DEITY_ID_HECATE => "Hecate",
        DEITY_ID_DESTINY => "Destiny",
        _ => "Unknown",
    }
}

fn altar_description(deity_id: u8) -> &'static str {
    match deity_id {
        DEITY_ID_ODIN => "This granite altar is graven with a gallows.",
        DEITY_ID_SET => "This sandstone altar has a black hand drawn on it.",
        DEITY_ID_HECATE => "This silver altar is inlaid with a black crescent moon.",
        DEITY_ID_ATHENA => "This golden altar is inscribed with an owl.",
        DEITY_ID_DESTINY => "This crystal altar is in the form of an omega.",
        _ => "This rude altar has no markings.",
    }
}

fn altar_needs_initial_worship(state: &GameState) -> bool {
    state.progression.patron_deity == 0 || state.progression.priest_rank == 0
}

fn is_friendly_deity_pair(left: u8, right: u8) -> bool {
    matches!(
        (left, right),
        (DEITY_ID_ODIN, DEITY_ID_ATHENA)
            | (DEITY_ID_ATHENA, DEITY_ID_ODIN)
            | (DEITY_ID_SET, DEITY_ID_HECATE)
            | (DEITY_ID_HECATE, DEITY_ID_SET)
    )
}

fn deity_allows_alignment(deity_id: u8, alignment: Alignment) -> bool {
    match deity_id {
        DEITY_ID_ODIN | DEITY_ID_ATHENA => alignment == Alignment::Lawful,
        DEITY_ID_SET | DEITY_ID_HECATE => alignment == Alignment::Chaotic,
        DEITY_ID_DESTINY => true,
        _ => alignment == Alignment::Neutral,
    }
}

fn sacrilege_penalty(state: &mut GameState, deity_id: u8) -> String {
    let patron = state.progression.patron_deity;
    state.progression.patron_deity = 0;
    state.progression.priest_rank = 0;
    state.progression.quests.temple.rank = 0;
    state.progression.quests.temple.quest_flags |= 0x8000;
    state.progression.deity_favor = 0;
    state.progression.deity_blessing_ready = false;
    state.spellbook.max_mana = (state.spellbook.max_mana - 12).max(24);
    state.spellbook.mana = state.spellbook.mana.min(state.spellbook.max_mana);
    state.player.stats.hp = (state.player.stats.hp - 6).max(1);
    format!(
        "Sacrilege! {} strips your patronage as you pray to {}.",
        deity_name(patron),
        deity_name(deity_id)
    )
}

fn apply_altar_prayer(state: &mut GameState, deity_id: u8, events: &mut Vec<Event>) -> String {
    if state.progression.patron_deity == 0 || state.progression.priest_rank == 0 {
        if deity_allows_alignment(deity_id, state.progression.alignment) {
            state.progression.patron_deity = deity_id;
            state.progression.priest_rank = state.progression.priest_rank.max(1);
            state.progression.deity_favor = state.progression.deity_favor.max(1).saturating_add(2);
            events.push(Event::ProgressionUpdated {
                guild_rank: state.progression.guild_rank,
                priest_rank: state.progression.priest_rank,
                alignment: state.progression.alignment,
            });
            return format!("{} accepts your devotion.", deity_name(deity_id));
        }
        return format!("{} ignores your prayer.", deity_name(deity_id));
    }

    let patron = state.progression.patron_deity;
    if patron != deity_id {
        if patron == DEITY_ID_DESTINY || is_friendly_deity_pair(patron, deity_id) {
            state.progression.deity_favor = state.progression.deity_favor.saturating_add(1);
            return format!(
                "{} tolerates your prayer at the altar of {}.",
                deity_name(patron),
                deity_name(deity_id)
            );
        }
        let note = sacrilege_penalty(state, deity_id);
        events.push(Event::ProgressionUpdated {
            guild_rank: state.progression.guild_rank,
            priest_rank: state.progression.priest_rank,
            alignment: state.progression.alignment,
        });
        return note;
    }

    if !deity_allows_alignment(patron, state.progression.alignment) {
        state.progression.deity_favor = (state.progression.deity_favor - 2).max(0);
        return "You have strayed from your deity's path.".to_string();
    }

    let gain = if state.progression.deity_blessing_ready { 5 } else { 2 };
    state.progression.deity_favor = state.progression.deity_favor.saturating_add(gain);
    state.progression.deity_blessing_ready = false;
    format!("{} hears your prayer.", deity_name(deity_id))
}

fn apply_altar_sacrifice(state: &mut GameState, deity_id: u8, events: &mut Vec<Event>) -> String {
    if state.gold < 50 {
        return "Not enough gold for a sacrifice.".to_string();
    }
    state.gold -= 50;
    events.push(Event::EconomyUpdated {
        source: "altar".to_string(),
        gold: state.gold,
        bank_gold: state.bank_gold,
    });

    if state.progression.patron_deity == 0 {
        return "A sacrifice alone is not enough; establish devotion first.".to_string();
    }

    if state.progression.patron_deity != deity_id
        && state.progression.patron_deity != DEITY_ID_DESTINY
        && !is_friendly_deity_pair(state.progression.patron_deity, deity_id)
    {
        let note = sacrilege_penalty(state, deity_id);
        events.push(Event::ProgressionUpdated {
            guild_rank: state.progression.guild_rank,
            priest_rank: state.progression.priest_rank,
            alignment: state.progression.alignment,
        });
        return note;
    }

    state.progression.deity_favor = state.progression.deity_favor.saturating_add(6);
    state.progression.deity_blessing_ready = true;
    format!("{} accepts your sacrifice.", deity_name(deity_id))
}

fn apply_altar_blessing(state: &mut GameState, deity_id: u8, events: &mut Vec<Event>) -> String {
    let patron = state.progression.patron_deity;
    if patron == 0 {
        return "Your ardent plea is ignored.".to_string();
    }
    if patron != deity_id && patron != DEITY_ID_DESTINY && !is_friendly_deity_pair(patron, deity_id)
    {
        let note = sacrilege_penalty(state, deity_id);
        events.push(Event::ProgressionUpdated {
            guild_rank: state.progression.guild_rank,
            priest_rank: state.progression.priest_rank,
            alignment: state.progression.alignment,
        });
        return note;
    }

    if state.progression.deity_blessing_ready || state.progression.deity_favor >= 8 {
        state.progression.deity_blessing_ready = false;
        state.progression.deity_favor = state.progression.deity_favor.saturating_sub(8);
        state.player.stats.hp = (state.player.stats.hp + 10).min(state.player.stats.max_hp);
        state.spellbook.mana = (state.spellbook.mana + 20).min(state.spellbook.max_mana);
        return "A shaft of lucent radiance lances down from the heavens! You feel uplifted...."
            .to_string();
    }

    "Your ardent plea is ignored. You feel ashamed.".to_string()
}

fn site_interaction_prompt(state: &GameState, kind: &SiteInteractionKind) -> String {
    match kind {
        SiteInteractionKind::Shop => format!(
            "Shop: [1/r] ration (12g) [2/p] healing potion (30g) [3/i] identify scroll (40g) [4/x] leave | gold={}",
            state.gold
        ),
        SiteInteractionKind::Armorer => format!(
            "Armorer: [1/a] chain mail (70g) [2/w] long sword (65g) [3/r] refit (30g) [4/x] leave | gold={}",
            state.gold
        ),
        SiteInteractionKind::Club => format!(
            "Club: [1/m] membership drink (20g) [2/l] legal favor (20g) [3/x] leave | gold={} heat={}",
            state.gold, state.legal_heat
        ),
        SiteInteractionKind::Gym => format!(
            "Gym: [1/d] drills (30g) [2/s] spar contract (35g) [3/x] leave | gold={} hp={}/{}",
            state.gold, state.player.stats.hp, state.player.stats.max_hp
        ),
        SiteInteractionKind::Healer => format!(
            "Healer: [1/h] heal wounds (18g) [2/c] cure poison (25g) [3/x] leave | gold={} hp={}/{}",
            state.gold, state.player.stats.hp, state.player.stats.max_hp
        ),
        SiteInteractionKind::Casino => format!(
            "Casino: [1/b] buy chips (25g) [2/p] play table [3/x] leave | gold={}",
            state.gold
        ),
        SiteInteractionKind::Commandant => format!(
            "Commandant: [1/b] buy a bucket! (20g) [2/r] report patrol [3/x] leave | gold={} food={} heat={}",
            state.gold, state.food, state.legal_heat
        ),
        SiteInteractionKind::Diner => format!(
            "Diner: [1/m] meal (8g) [2/c] coffee (6g) [3/x] leave | gold={} food={}",
            state.gold, state.food
        ),
        SiteInteractionKind::Craps => format!(
            "Craps: [1/b] buy into dice (15g) [2/r] cash out [3/x] leave | gold={}",
            state.gold
        ),
        SiteInteractionKind::Tavern => format!(
            "Tavern: [1/a] ale (6g) [2/m] stew (10g) [3/r] rumor (8g) [4/x] leave | gold={} food={} heat={}",
            state.gold, state.food, state.legal_heat
        ),
        SiteInteractionKind::PawnShop => format!(
            "Pawn shop: [1/b] buy oddity (15g) [2/s] sell first item [3/x] leave | gold={} pack={}",
            state.gold,
            state.player.inventory.len()
        ),
        SiteInteractionKind::Brothel => format!(
            "Brothel: [1/r] rent room (25g) [2/g] pay for gossip (10g) [3/x] leave | gold={} hp={}/{}",
            state.gold, state.player.stats.hp, state.player.stats.max_hp
        ),
        SiteInteractionKind::Condo => format!(
            "Condo: [1/r] rent room (40g) [2/s] secure stash (15g) [3/x] leave | gold={} bank={}",
            state.gold, state.bank_gold
        ),
        SiteInteractionKind::Bank => format!(
            "Bank: [1/d] deposit 50 [2/w] withdraw 50 [3/s] post surety 25 [4/x] leave | gold={} bank={} legal_heat={}",
            state.gold, state.bank_gold, state.legal_heat
        ),
        SiteInteractionKind::MercGuild => format!(
            "Merc guild: [1/t] train arms (40g) [2/c] take contract (40g) [3/p] promotion board (60g) [4/x] leave | gold={} rank={}",
            state.gold, state.progression.guild_rank
        ),
        SiteInteractionKind::ThievesGuild => format!(
            "Thieves guild: [1/j] join (30g) [2/h] take heist (25g) [3/p] promotion board (55g) [4/x] leave | gold={} rank={} heat={}",
            state.gold,
            state.progression.quests.thieves.rank.max(0),
            state.legal_heat
        ),
        SiteInteractionKind::Temple => format!(
            "Temple: [1/t] tithe (15g) [2/p] pray [3/b] blessing (35g) [4/s] sanctuary [5/x] leave | favor={} gold={}",
            state.progression.deity_favor, state.gold
        ),
        SiteInteractionKind::College => format!(
            "College: [1/m] mana training (25g) [2/l] learn spell (40g) [3/i] identify item (30g) [4/x] leave | gold={}",
            state.gold
        ),
        SiteInteractionKind::Sorcerors => {
            format!(
                "Sorcerors: [1/r] recharge (30g) [2/d] deep lore (50g) [3/t] transmute focus (45g) [4/x] leave | gold={}",
                state.gold
            )
        }
        SiteInteractionKind::Castle => {
            format!(
                "Castle: [1/f] settle legal matters [2/a] audience [3/p] petition reward [4/x] leave | legal_heat={} quest={:?}",
                state.legal_heat, state.progression.quest_state
            )
        }
        SiteInteractionKind::Palace => format!(
            "Palace: [1/a] audience [2/p] petition crown [3/x] leave | access={} stage={:?}",
            state.progression.main_quest.palace_access, state.progression.main_quest.stage
        ),
        SiteInteractionKind::Order => format!(
            "Order: [1/v] lawful vow [2/a] absolution (25g) [3/u] audience [4/x] leave | alignment={:?} legal_heat={}",
            state.progression.alignment, state.legal_heat
        ),
        SiteInteractionKind::Charity => format!(
            "Charity: [1/m] meal+shelter [2/c] cleansing [3/v] volunteer [4/x] leave | hp={}/{} food={} legal_heat={}",
            state.player.stats.hp, state.player.stats.max_hp, state.food, state.legal_heat
        ),
        SiteInteractionKind::Monastery => format!(
            "Monastery: [1/m] meditate (20g) [2/d] donate item [3/v] vow discipline [4/x] leave | gold={} rank={} align={:?}",
            state.gold,
            state.progression.quests.monastery.rank.max(0),
            state.progression.alignment
        ),
        SiteInteractionKind::Arena => {
            if state.progression.arena_rank > 0 {
                format!(
                    "Rampart Coliseum: Enter the games? [1/y]es [2/n]o | rank={} opponent={} active={}",
                    state.progression.arena_rank,
                    state.progression.arena_opponent,
                    state.progression.arena_match_active
                )
            } else {
                "Rampart Coliseum: Enter the games, or Register as a Gladiator? [1/e]nter [2/r]egister [3/x]leave".to_string()
            }
        }
        SiteInteractionKind::Altar { deity_id } => {
            let patron = if state.progression.patron_deity == 0 {
                "none".to_string()
            } else {
                deity_name(state.progression.patron_deity).to_string()
            };
            if altar_needs_initial_worship(state) {
                format!(
                    "{} Worship at this altar? [1/y]es [2/n]o [x]leave | patron={} favor={} gold={}",
                    altar_description(*deity_id),
                    patron,
                    state.progression.deity_favor,
                    state.gold
                )
            } else {
                format!(
                    "{} Request a Blessing, Sacrifice an offering, or just Pray [1/b]lessing [2/s]acrifice [3/p]ray [4/x]leave | patron={} favor={} gold={}",
                    altar_description(*deity_id),
                    patron,
                    state.progression.deity_favor,
                    state.gold
                )
            }
        }
    }
}

fn site_interaction_help_hint(state: &GameState, kind: &SiteInteractionKind) -> String {
    match kind {
        SiteInteractionKind::Arena if state.progression.arena_rank > 0 => {
            "Rampart Coliseum prompt: choose 1/y to enter or 2/n to decline (q/x closes)."
                .to_string()
        }
        SiteInteractionKind::Arena => {
            "Rampart Coliseum prompt: choose 1/e to enter or 2/r to register (q/x closes)."
                .to_string()
        }
        SiteInteractionKind::Temple => {
            "Temple prompt active: choose 1-5 or letter aliases shown in brackets.".to_string()
        }
        SiteInteractionKind::ThievesGuild => {
            "Thieves guild prompt active: choose 1-4 or letter aliases shown in brackets."
                .to_string()
        }
        SiteInteractionKind::Palace => {
            "Palace prompt active: choose audience/petition, or q/x to close.".to_string()
        }
        SiteInteractionKind::Monastery => {
            "Monastery prompt active: choose meditation/donation/vow, or q/x to close.".to_string()
        }
        SiteInteractionKind::Altar { .. } if altar_needs_initial_worship(state) => {
            "Altar prompt: choose 1/y to worship or 2/n to step away (q/x closes).".to_string()
        }
        SiteInteractionKind::Altar { .. } => {
            "Altar prompt: choose blessing/sacrifice/pray, or q/x to close.".to_string()
        }
        _ => "Site prompt active: choose a bracketed option, or press q/x to close.".to_string(),
    }
}

pub fn active_site_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_site_interaction.as_ref().map(|kind| site_interaction_prompt(state, kind))
}

pub fn active_site_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_site_interaction.as_ref().map(|kind| site_interaction_help_hint(state, kind))
}

fn inventory_slot_snapshot(state: &GameState, slot: usize) -> String {
    let key = legacy_inventory_slot_to_key(slot).unwrap_or('?');
    let label = inventory_slot_name(slot);
    let value = inventory_slot_item_id(state, slot)
        .and_then(|item_id| state.player.inventory.iter().find(|item| item.id == item_id))
        .map(|item| item.name.clone())
        .unwrap_or_else(|| "(vacant)".to_string());
    format!("{key}) {label}: {value}")
}

fn inventory_interaction_prompt(state: &GameState, interaction: &InventoryInteraction) -> String {
    match interaction {
        InventoryInteraction::Control { selected_slot, .. } => format!(
            "Inventory action [d,e,l,p,s,t,x,>,<,?,ESCAPE] | {}",
            inventory_slot_snapshot(state, *selected_slot)
        ),
        InventoryInteraction::TakeFromPackSelect { .. } => {
            "Take from pack: choose item letter [a-z], ? lists pack, ESC cancels.".to_string()
        }
    }
}

fn inventory_interaction_help_hint(interaction: &InventoryInteraction) -> String {
    match interaction {
        InventoryInteraction::Control { .. } => {
            "Inventory active: d drop, e exchange, l look, p put to pack, s show pack, t take, x exchange+exit, </> move slot."
                .to_string()
        }
        InventoryInteraction::TakeFromPackSelect { .. } => {
            "Pack selection active: type pack letter, ? to list pack entries, ESC to return."
                .to_string()
        }
    }
}

pub fn active_inventory_interaction_prompt(state: &GameState) -> Option<String> {
    state
        .pending_inventory_interaction
        .as_ref()
        .map(|interaction| inventory_interaction_prompt(state, interaction))
}

pub fn active_inventory_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_inventory_interaction.as_ref().map(inventory_interaction_help_hint)
}

fn item_prompt_filter_allows(item: &Item, filter: &ItemPromptFilter) -> bool {
    match filter {
        ItemPromptFilter::Any => true,
        ItemPromptFilter::Families(families) => {
            families.iter().any(|family| *family == item.family)
        }
    }
}

fn item_prompt_candidate_item_ids(
    state: &GameState,
    interaction: &ItemPromptInteraction,
) -> Vec<u32> {
    let mut ids = Vec::new();

    for slot in 0..INVENTORY_SLOT_COUNT {
        if let Some(item_id) = inventory_slot_item_id(state, slot) {
            if ids.contains(&item_id) {
                continue;
            }
            if let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id)
                && item_prompt_filter_allows(item, &interaction.filter)
            {
                ids.push(item_id);
            }
        }
    }

    for item_id in &state.player.pack_order {
        if ids.contains(item_id) {
            continue;
        }
        if let Some(item) = state.player.inventory.iter().find(|entry| entry.id == *item_id)
            && item_prompt_filter_allows(item, &interaction.filter)
        {
            ids.push(*item_id);
        }
    }

    for item in &state.player.inventory {
        if ids.contains(&item.id) {
            continue;
        }
        if item_prompt_filter_allows(item, &interaction.filter) {
            ids.push(item.id);
        }
    }

    ids
}

fn item_prompt_choice_key(index: usize) -> Option<char> {
    const LETTER_KEYS: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    LETTER_KEYS.get(index).copied()
}

fn item_prompt_choice_keys(state: &GameState, interaction: &ItemPromptInteraction) -> Vec<char> {
    let count = item_prompt_candidate_item_ids(state, interaction).len();
    (0..count).filter_map(item_prompt_choice_key).collect()
}

fn item_prompt_prompt(state: &GameState, interaction: &ItemPromptInteraction) -> String {
    let choice_keys = item_prompt_choice_keys(state, interaction);
    if choice_keys.is_empty() {
        format!("{} (no valid items)", interaction.prompt)
    } else {
        format!(
            "{} [{}] (? lists, ESC cancels)",
            interaction.prompt,
            choice_keys.into_iter().collect::<String>()
        )
    }
}

fn item_prompt_help_hint(interaction: &ItemPromptInteraction) -> String {
    format!(
        "{}: choose listed letter/number key, ? lists options, ESC cancels.",
        interaction.prompt
    )
}

pub fn active_item_prompt(state: &GameState) -> Option<String> {
    state.pending_item_prompt.as_ref().map(|interaction| item_prompt_prompt(state, interaction))
}

pub fn active_item_prompt_help_hint(state: &GameState) -> Option<String> {
    state.pending_item_prompt.as_ref().map(item_prompt_help_hint)
}

fn activation_interaction_prompt(interaction: &ActivationInteraction) -> String {
    match interaction {
        ActivationInteraction::ChooseKind => {
            "Activate -- item [i] or artifact [a] or quit [ESCAPE]?".to_string()
        }
    }
}

fn activation_interaction_help_hint(interaction: &ActivationInteraction) -> String {
    match interaction {
        ActivationInteraction::ChooseKind => {
            "Activation prompt active: choose i for thing item, a for artifact, q/esc to cancel."
                .to_string()
        }
    }
}

pub fn active_activation_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_activation_interaction.as_ref().map(activation_interaction_prompt)
}

pub fn active_activation_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_activation_interaction.as_ref().map(activation_interaction_help_hint)
}

fn quit_interaction_prompt(interaction: &QuitInteraction) -> String {
    match interaction {
        QuitInteraction::ConfirmQuit => "Quit: Are you sure? [y/n]".to_string(),
    }
}

fn quit_interaction_help_hint(interaction: &QuitInteraction) -> String {
    match interaction {
        QuitInteraction::ConfirmQuit => {
            "Quit prompt active: press y to retire now, n or q/esc to cancel.".to_string()
        }
    }
}

pub fn active_quit_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_quit_interaction.as_ref().map(quit_interaction_prompt)
}

pub fn active_quit_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_quit_interaction.as_ref().map(quit_interaction_help_hint)
}

fn talk_direction_interaction_prompt(interaction: TalkDirectionInteraction) -> String {
    match interaction {
        TalkDirectionInteraction::Talk => {
            "Talk -- choose direction (hjklyubn or keypad; ESCAPE aborts).".to_string()
        }
        TalkDirectionInteraction::Tunnel => {
            "Tunnel -- choose direction (hjklyubn or keypad; ESCAPE aborts).".to_string()
        }
    }
}

fn talk_direction_interaction_help_hint(interaction: TalkDirectionInteraction) -> String {
    match interaction {
        TalkDirectionInteraction::Talk => {
            "Talk prompt active: choose direction (hjklyubn or arrows), q/esc cancels."
                .to_string()
        }
        TalkDirectionInteraction::Tunnel => {
            "Tunnel prompt active: choose direction (hjklyubn or arrows), q/esc cancels."
                .to_string()
        }
    }
}

pub fn active_talk_direction_prompt(state: &GameState) -> Option<String> {
    state.pending_talk_direction.map(talk_direction_interaction_prompt)
}

pub fn active_talk_direction_help_hint(state: &GameState) -> Option<String> {
    state.pending_talk_direction.map(talk_direction_interaction_help_hint)
}

fn targeting_interaction_prompt(_state: &GameState, interaction: &TargetingInteraction) -> String {
    let mode = match interaction.mode {
        ProjectileKind::ThrownItem => "Throw",
        ProjectileKind::Arrow => "Fire arrow",
        ProjectileKind::Bolt => "Fire bolt",
        ProjectileKind::MagicMissile => "Cast magic missile",
        ProjectileKind::FireBolt => "Cast firebolt",
        ProjectileKind::LightningBolt => "Cast lightning bolt",
    };
    format!(
        "Targeting... {mode} at ({}, {}). Move cursor, '.' confirm, '?' help, ESC cancel.",
        interaction.cursor.x, interaction.cursor.y
    )
}

fn targeting_interaction_help_hint(
    _state: &GameState,
    interaction: &TargetingInteraction,
) -> String {
    let mode = match interaction.mode {
        ProjectileKind::ThrownItem => "throw",
        ProjectileKind::Arrow | ProjectileKind::Bolt => "fire",
        ProjectileKind::MagicMissile | ProjectileKind::FireBolt | ProjectileKind::LightningBolt => {
            "cast"
        }
    };
    format!("Targeting active: use hjklyubn or arrows, '.' to {mode}, '?' for help, ESC to cancel.")
}

pub fn active_targeting_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_targeting_interaction.as_ref().map(|it| targeting_interaction_prompt(state, it))
}

pub fn active_targeting_interaction_help_hint(state: &GameState) -> Option<String> {
    state
        .pending_targeting_interaction
        .as_ref()
        .map(|it| targeting_interaction_help_hint(state, it))
}

fn spell_name_by_id(spell_id: usize) -> &'static str {
    LEGACY_SPELL_NAMES.get(spell_id).copied().unwrap_or("unknown spell")
}

fn spell_drain_by_id(state: &GameState, spell_id: usize) -> i32 {
    state
        .spellbook
        .spells
        .get(spell_id)
        .map(|spell| spell.power_drain)
        .unwrap_or_else(|| LEGACY_SPELL_COSTS.get(spell_id).copied().unwrap_or(0))
}

fn spell_interaction_prompt(state: &GameState, interaction: &SpellInteraction) -> String {
    match interaction {
        SpellInteraction::SpellSelect { .. } => {
            format!("Cast Spell: [type spell abbrev, ?, or ESCAPE]: {}_", state.spell_input_buffer)
        }
    }
}

fn spell_interaction_help_hint(state: &GameState, interaction: &SpellInteraction) -> String {
    let _ = state;
    match interaction {
        SpellInteraction::SpellSelect { .. } => {
            "Spell prompt active: type abbreviation, ? lists known spells, Enter casts, Esc cancels."
                .to_string()
        }
    }
}

pub fn active_spell_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_spell_interaction.as_ref().map(|it| spell_interaction_prompt(state, it))
}

pub fn active_spell_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_spell_interaction.as_ref().map(|it| spell_interaction_help_hint(state, it))
}

fn wizard_interaction_prompt(state: &GameState, interaction: &WizardInteraction) -> String {
    match interaction {
        WizardInteraction::EnterWizardConfirm { via_backdoor } => {
            if *via_backdoor {
                "A hidden panel shifts. Enter wizard mode? [y/n]".to_string()
            } else {
                "You just asked to enter wizard mode. [y/n]".to_string()
            }
        }
        WizardInteraction::WishTextEntry { .. } => {
            format!("Wish text: {}_", state.wizard_input_buffer)
        }
        WizardInteraction::WishAcquisitionKindSelect { cheated, item_hint } => {
            let mut prompt = if *cheated {
                "Acquire which kind of item: !?][}{)/=%%\\& ".to_string()
            } else {
                "Acquire which kind of item: !?][}{)/=%%\\ ".to_string()
            };
            if let Some(hint) = item_hint.as_ref()
                && !hint.trim().is_empty()
            {
                prompt.push_str(&format!("(hint: {hint})"));
            }
            prompt
        }
        WizardInteraction::WishAcquisitionItemSelect { kind, .. } => {
            format!(
                "Choose {} by number or name: {}_",
                wish_item_kind_label(*kind),
                state.wizard_input_buffer
            )
        }
        WizardInteraction::StatusFlagActionSelect => {
            "Set or Reset or Forget it [s/r/ESCAPE]:".to_string()
        }
        WizardInteraction::StatusFlagIndexEntry { set_mode } => {
            let verb = if *set_mode { "set" } else { "reset" };
            format!("Choose status bit index to {verb} (0-63): {}_", state.wizard_input_buffer)
        }
        WizardInteraction::StatEditorSelect { slot } => {
            format!(
                "Stat editor: [{}] {}={} (j/k or </> move, space/enter edit, esc quit).",
                slot,
                stat_slot_name(*slot),
                stat_slot_value(state, *slot)
            )
        }
        WizardInteraction::StatEditorValueEntry { slot } => {
            format!(
                "New value for {} (current {}): {}_",
                stat_slot_name(*slot),
                stat_slot_value(state, *slot),
                state.wizard_input_buffer
            )
        }
        WizardInteraction::BashDirectionSelect => {
            "Bashing -- choose direction (hjklyubn or keypad; ESCAPE aborts).".to_string()
        }
    }
}

fn wizard_interaction_help_hint(state: &GameState, interaction: &WizardInteraction) -> String {
    match interaction {
        WizardInteraction::EnterWizardConfirm { .. } => {
            "Wizard prompt active: press y to confirm, n or q/esc to cancel.".to_string()
        }
        WizardInteraction::WishTextEntry { .. } => {
            "Wizard wish prompt: type text, Backspace edits, Enter commits, q/esc cancels."
                .to_string()
        }
        WizardInteraction::WishAcquisitionKindSelect { cheated, .. } => {
            if *cheated {
                "Wizard acquisition: choose ! ? ] [ } { ) / = % \\ or & (q/esc cancels)."
                    .to_string()
            } else {
                "Wizard acquisition: choose ! ? ] [ } { ) / = % or \\ (q/esc cancels)."
                    .to_string()
            }
        }
        WizardInteraction::WishAcquisitionItemSelect { kind, .. } => {
            format!(
                "Wizard acquisition: enter {} number/name, Enter commits, Backspace edits, q/esc cancels.",
                wish_item_kind_label(*kind)
            )
        }
        WizardInteraction::StatusFlagActionSelect => {
            "Wizard status editor: choose s=set, r=reset, or q/esc to exit.".to_string()
        }
        WizardInteraction::StatusFlagIndexEntry { set_mode } => {
            let verb = if *set_mode { "set" } else { "reset" };
            format!(
                "Wizard status editor: enter a bit index (0-63) to {verb}, Enter applies, q/esc cancels."
            )
        }
        WizardInteraction::StatEditorSelect { .. } => {
            "Wizard stat editor: j/k or </> changes stat, space/enter edits value, q/esc exits."
                .to_string()
        }
        WizardInteraction::StatEditorValueEntry { .. } => {
            "Wizard stat editor: enter numeric value, Backspace edits, Enter applies, q/esc cancels."
                .to_string()
        }
        WizardInteraction::BashDirectionSelect => {
            let _ = state;
            "Wizard backdoor check: choose direction (hjklyubn or keypad), q/esc aborts."
                .to_string()
        }
    }
}

pub fn active_wizard_interaction_prompt(state: &GameState) -> Option<String> {
    state.pending_wizard_interaction.as_ref().map(|it| wizard_interaction_prompt(state, it))
}

pub fn active_wizard_interaction_help_hint(state: &GameState) -> Option<String> {
    state.pending_wizard_interaction.as_ref().map(|it| wizard_interaction_help_hint(state, it))
}

pub fn modal_input_profile(state: &GameState) -> ModalInputProfile {
    if let Some(interaction) = state.pending_wizard_interaction.as_ref() {
        return wizard_modal_input_profile(interaction);
    }
    if let Some(interaction) = state.pending_spell_interaction.as_ref() {
        return spell_modal_input_profile(interaction);
    }
    if state.pending_quit_interaction.is_some() {
        return ModalInputProfile::ChoiceEntry;
    }
    if state.pending_activation_interaction.is_some() {
        return ModalInputProfile::ChoiceEntry;
    }
    if state.pending_talk_direction.is_some() {
        return ModalInputProfile::DirectionEntry;
    }
    if state.pending_targeting_interaction.is_some() {
        return ModalInputProfile::DirectionEntry;
    }
    if state.pending_inventory_interaction.is_some()
        || state.pending_item_prompt.is_some()
        || state.pending_site_interaction.is_some()
    {
        return ModalInputProfile::ChoiceEntry;
    }
    ModalInputProfile::None
}

fn wizard_modal_input_profile(interaction: &WizardInteraction) -> ModalInputProfile {
    match interaction {
        WizardInteraction::WishTextEntry { .. }
        | WizardInteraction::WishAcquisitionItemSelect { .. }
        | WizardInteraction::StatusFlagIndexEntry { .. }
        | WizardInteraction::StatEditorValueEntry { .. } => ModalInputProfile::TextEntry,
        WizardInteraction::BashDirectionSelect => ModalInputProfile::DirectionEntry,
        WizardInteraction::EnterWizardConfirm { .. }
        | WizardInteraction::WishAcquisitionKindSelect { .. }
        | WizardInteraction::StatusFlagActionSelect
        | WizardInteraction::StatEditorSelect { .. } => ModalInputProfile::ChoiceEntry,
    }
}

fn spell_modal_input_profile(interaction: &SpellInteraction) -> ModalInputProfile {
    match interaction {
        SpellInteraction::SpellSelect { .. } => ModalInputProfile::TextEntry,
    }
}

fn begin_wizard_interaction(
    state: &mut GameState,
    interaction: WizardInteraction,
    prompt: String,
) -> (String, bool) {
    state.pending_wizard_interaction = Some(interaction);
    state.wizard_input_buffer.clear();
    (prompt, true)
}

#[derive(Debug, Clone)]
struct WizardInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct SpellInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct QuitInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct TalkDirectionInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct ActivationInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct InventoryInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct ItemPromptInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone)]
struct TargetingInteractionResolution {
    freeze_world_progression: bool,
    command_for_accounting: Command,
    turn_minutes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WizardInputToken {
    Cancel,
    Enter,
    Backspace,
    DirectionDelta { dx: i32, dy: i32 },
    Text(String),
    None,
}

fn direction_delta_from_char(ch: char) -> Option<(i32, i32)> {
    match ch.to_ascii_lowercase() {
        'h' | '4' => Some((-1, 0)),
        'j' | '2' => Some((0, 1)),
        'k' | '8' => Some((0, -1)),
        'l' | '6' => Some((1, 0)),
        'y' | '7' => Some((-1, -1)),
        'u' | '9' => Some((1, -1)),
        'b' | '1' => Some((-1, 1)),
        'n' | '3' => Some((1, 1)),
        _ => None,
    }
}

fn parse_wizard_input_token(command: &Command) -> WizardInputToken {
    match command {
        Command::Legacy { token } => {
            let raw = token.as_str();
            let trimmed = raw.trim();
            if trimmed.eq_ignore_ascii_case("<esc>")
                || trimmed.eq_ignore_ascii_case("esc")
                || trimmed.eq_ignore_ascii_case("escape")
                || trimmed.eq_ignore_ascii_case("q")
            {
                return WizardInputToken::Cancel;
            }
            if trimmed.eq_ignore_ascii_case("<enter>")
                || trimmed.eq_ignore_ascii_case("enter")
                || raw == "\n"
                || raw == "\r"
            {
                return WizardInputToken::Enter;
            }
            if trimmed.eq_ignore_ascii_case("<backspace>")
                || trimmed.eq_ignore_ascii_case("backspace")
                || raw == "\u{8}"
                || raw == "\u{7f}"
            {
                return WizardInputToken::Backspace;
            }

            if raw == " " {
                return WizardInputToken::Text(" ".to_string());
            }

            if !raw.is_empty() {
                return WizardInputToken::Text(raw.to_string());
            }

            WizardInputToken::None
        }
        Command::Drop { slot } => WizardInputToken::Text(slot.saturating_add(1).to_string()),
        Command::Move(dir) | Command::Attack(dir) => match dir {
            Direction::North => WizardInputToken::DirectionDelta { dx: 0, dy: -1 },
            Direction::South => WizardInputToken::DirectionDelta { dx: 0, dy: 1 },
            Direction::East => WizardInputToken::DirectionDelta { dx: 1, dy: 0 },
            Direction::West => WizardInputToken::DirectionDelta { dx: -1, dy: 0 },
        },
        Command::Wait => WizardInputToken::Text(" ".to_string()),
        Command::Pickup => WizardInputToken::None,
    }
}

fn append_wizard_buffer(state: &mut GameState, text: &str, max_len: usize) {
    if state.wizard_input_buffer.len() >= max_len {
        return;
    }
    let room = max_len.saturating_sub(state.wizard_input_buffer.len());
    let snippet: String = text.chars().take(room).collect();
    state.wizard_input_buffer.push_str(&snippet);
}

fn classify_wizard_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_wizard_interaction.as_ref() {
        if wizard_interaction_prompt(state, interaction) == note {
            return UiLogClass::Prompt;
        }
        if wizard_interaction_help_hint(state, interaction) == note {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_wizard_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_wizard_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled { token: "wizard".to_string(), note, fully_modeled: true });
}

fn sync_spellbook_state(state: &mut GameState) {
    if state.spellbook.spells.len() != LEGACY_SPELL_NAMES.len() {
        let mut rebuilt = default_spellbook_spells();
        for spell in &state.spellbook.spells {
            let idx = usize::from(spell.id);
            if let Some(slot) = rebuilt.get_mut(idx) {
                slot.known = spell.known;
                if spell.power_drain > 0 {
                    slot.power_drain = spell.power_drain;
                }
            }
        }
        state.spellbook.spells = rebuilt;
    }

    for (idx, spell) in state.spellbook.spells.iter_mut().enumerate() {
        spell.id = idx as u8;
        if spell.power_drain <= 0 {
            spell.power_drain = LEGACY_SPELL_COSTS[idx];
        }
    }
}

fn spell_known_indices_sorted(state: &GameState) -> Vec<usize> {
    LEGACY_SPELL_SORTED_IDS
        .iter()
        .copied()
        .filter(|idx| state.spellbook.spells.get(*idx).map(|spell| spell.known).unwrap_or(false))
        .collect()
}

fn spell_prefix_matches(name: &str, prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    name.to_ascii_lowercase().starts_with(&prefix.to_ascii_lowercase())
}

fn spell_candidates_for_prefix(state: &GameState, prefix: &str) -> Vec<usize> {
    spell_known_indices_sorted(state)
        .into_iter()
        .filter(|idx| spell_prefix_matches(spell_name_by_id(*idx), prefix))
        .collect()
}

fn begin_quit_interaction(state: &mut GameState) -> (String, bool) {
    state.pending_quit_interaction = Some(QuitInteraction::ConfirmQuit);
    ("Quit: Are you sure? [y/n]".to_string(), true)
}

fn resolve_pending_quit_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> Option<QuitInteractionResolution> {
    let Some(interaction) = state.pending_quit_interaction.clone() else {
        return None;
    };

    let mut resolution = QuitInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    match interaction {
        QuitInteraction::ConfirmQuit => {
            let input = parse_wizard_input_token(command);
            match input {
                WizardInputToken::Cancel => {
                    state.pending_quit_interaction = None;
                    push_timeline_line(state, "Quit canceled.".to_string());
                    events.push(Event::LegacyHandled {
                        token: "Q".to_string(),
                        note: "quit canceled".to_string(),
                        fully_modeled: true,
                    });
                }
                WizardInputToken::Text(text) => {
                    let trimmed = text.trim();
                    if trimmed.eq_ignore_ascii_case("y") {
                        state.pending_quit_interaction = None;
                        apply_explicit_victory_trigger(
                            state,
                            VictoryTrigger::QuitConfirmed,
                            events,
                        );
                        push_timeline_line(
                            state,
                            "You settle down from adventuring and retire.".to_string(),
                        );
                        events.push(Event::LegacyHandled {
                            token: "Q".to_string(),
                            note: "quit confirmed".to_string(),
                            fully_modeled: true,
                        });
                        resolution.freeze_world_progression = false;
                        resolution.command_for_accounting =
                            Command::Legacy { token: "Q".to_string() };
                    } else if trimmed.eq_ignore_ascii_case("n") {
                        state.pending_quit_interaction = None;
                        push_timeline_line(state, "Quit canceled.".to_string());
                        events.push(Event::LegacyHandled {
                            token: "Q".to_string(),
                            note: "quit canceled".to_string(),
                            fully_modeled: true,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    Some(resolution)
}

fn begin_talk_direction_interaction(
    state: &mut GameState,
    interaction: TalkDirectionInteraction,
) -> (String, bool) {
    state.pending_talk_direction = Some(interaction);
    (talk_direction_interaction_prompt(interaction), true)
}

fn resolve_pending_talk_direction_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> Option<TalkDirectionInteractionResolution> {
    let Some(interaction) = state.pending_talk_direction else {
        return None;
    };

    let mut resolution = TalkDirectionInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    let input = parse_wizard_input_token(command);
    let mut delta = None;
    match input {
        WizardInputToken::Cancel => {
            state.pending_talk_direction = None;
            let token = if interaction == TalkDirectionInteraction::Talk { "t" } else { "T" };
            let note = if interaction == TalkDirectionInteraction::Talk {
                "talk canceled".to_string()
            } else {
                "tunnel canceled".to_string()
            };
            events.push(Event::LegacyHandled { token: token.to_string(), note, fully_modeled: true });
            return Some(resolution);
        }
        WizardInputToken::DirectionDelta { dx, dy } => {
            delta = Some((dx, dy));
        }
        WizardInputToken::Text(text) => {
            delta = parse_direction_delta_from_text(&text);
        }
        WizardInputToken::Enter | WizardInputToken::Backspace | WizardInputToken::None => {}
    }

    let Some((dx, dy)) = delta else {
        return Some(resolution);
    };

    let target =
        Position { x: state.player.position.x + dx, y: state.player.position.y + dy };
    state.pending_talk_direction = None;
    let (token, note, fully_modeled) = match interaction {
        TalkDirectionInteraction::Talk => {
            let (note, fully_modeled) = resolve_talk_direction(state, target, events);
            ("t".to_string(), note, fully_modeled)
        }
        TalkDirectionInteraction::Tunnel => {
            let (note, fully_modeled) = resolve_tunnel_direction(state, target);
            ("T".to_string(), note, fully_modeled)
        }
    };

    push_timeline_line(state, note.clone());
    events.push(Event::LegacyHandled { token: token.clone(), note, fully_modeled });
    resolution.freeze_world_progression = false;
    resolution.command_for_accounting = Command::Legacy { token: token.clone() };
    resolution.turn_minutes =
        estimate_legacy_turn_minutes(&token, state.world_mode, state.options.searchnum);
    Some(resolution)
}

fn refresh_spell_interaction_filter(state: &mut GameState) {
    let prefix = state.spell_input_buffer.clone();
    let filtered = spell_candidates_for_prefix(state, &prefix);
    if let Some(SpellInteraction::SpellSelect { filtered_indices, cursor }) =
        state.pending_spell_interaction.as_mut()
    {
        *filtered_indices = filtered;
        if filtered_indices.is_empty() {
            *cursor = 0;
        } else if *cursor >= filtered_indices.len() {
            *cursor = filtered_indices.len() - 1;
        }
    }
}

fn has_active_fear(state: &GameState) -> bool {
    state.status_effects.iter().any(|effect| effect.id == "fear" && effect.remaining_turns > 0)
}

fn compute_spell_drain(state: &GameState, spell_id: usize) -> i32 {
    let base = spell_drain_by_id(state, spell_id).max(1);
    match state.progression.lunarity {
        1 => (base / 2).max(1),
        -1 => base.saturating_mul(2),
        _ => base,
    }
}

fn begin_spell_interaction(state: &mut GameState) -> (String, bool) {
    sync_spellbook_state(state);
    if has_active_fear(state) {
        return ("You are too afraid to concentrate on a spell!".to_string(), true);
    }
    let known = spell_known_indices_sorted(state);
    if known.is_empty() {
        return ("You don't know any spells!".to_string(), true);
    }
    let interaction = SpellInteraction::SpellSelect { filtered_indices: known, cursor: 0 };
    state.pending_spell_interaction = Some(interaction.clone());
    state.spell_input_buffer.clear();
    (spell_interaction_prompt(state, &interaction), true)
}

fn classify_spell_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_spell_interaction.as_ref() {
        if spell_interaction_prompt(state, interaction) == note {
            return UiLogClass::Prompt;
        }
        if spell_interaction_help_hint(state, interaction) == note {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_spell_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_spell_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled { token: "m".to_string(), note, fully_modeled: true });
}

fn show_known_spells(state: &GameState, filtered: &[usize]) -> String {
    if filtered.is_empty() {
        return "No spells match that prefix!".to_string();
    }
    let mut parts = Vec::new();
    for spell_id in filtered {
        let name = spell_name_by_id(*spell_id);
        let drain = compute_spell_drain(state, *spell_id);
        parts.push(format!("{name} ({drain} mana)"));
    }
    format!("Possible spells: {}", parts.join(", "))
}

fn resolve_pending_spell_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> Option<SpellInteractionResolution> {
    let Some(interaction) = state.pending_spell_interaction.clone() else {
        return None;
    };
    let mut resolution = SpellInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    match interaction {
        SpellInteraction::SpellSelect { .. } => {
            let input = parse_wizard_input_token(command);
            match input {
                WizardInputToken::Cancel => {
                    state.pending_spell_interaction = None;
                    state.spell_input_buffer.clear();
                    record_spell_note(state, events, "Spell casting canceled.".to_string());
                }
                WizardInputToken::Backspace => {
                    state.spell_input_buffer.pop();
                    refresh_spell_interaction_filter(state);
                }
                WizardInputToken::Enter => {
                    let prefix = state.spell_input_buffer.trim().to_string();
                    let filtered = spell_candidates_for_prefix(state, &prefix);
                    if prefix.is_empty() || filtered.len() != 1 {
                        record_spell_note(
                            state,
                            events,
                            "That is an ambiguous abbreviation!".to_string(),
                        );
                    } else {
                        let spell_id = filtered[0];
                        let (note, _modeled) = cast_spell_by_id(state, events, spell_id);
                        state.pending_spell_interaction = None;
                        state.spell_input_buffer.clear();
                        record_spell_note(state, events, note);
                        if state.pending_targeting_interaction.is_some() {
                            resolution.freeze_world_progression = true;
                            resolution.command_for_accounting =
                                Command::Legacy { token: "F".to_string() };
                            resolution.turn_minutes = 0;
                        } else {
                            resolution.freeze_world_progression = false;
                            resolution.command_for_accounting =
                                Command::Legacy { token: "m".to_string() };
                            resolution.turn_minutes = estimate_legacy_turn_minutes(
                                "m",
                                state.world_mode,
                                state.options.searchnum,
                            );
                        }
                    }
                }
                WizardInputToken::Text(text) => {
                    let mut showed_list = false;
                    for ch in text.chars() {
                        if ch == '?' {
                            let prefix = state.spell_input_buffer.trim().to_string();
                            let filtered = spell_candidates_for_prefix(state, &prefix);
                            record_spell_note(state, events, show_known_spells(state, &filtered));
                            showed_list = true;
                            continue;
                        }
                        let normalized =
                            if ch.is_ascii_uppercase() { ch.to_ascii_lowercase() } else { ch };
                        if !(normalized.is_ascii_lowercase() || normalized == ' ') {
                            continue;
                        }
                        let mut candidate = state.spell_input_buffer.clone();
                        if candidate.len() >= 80 {
                            continue;
                        }
                        candidate.push(normalized);
                        if !spell_candidates_for_prefix(state, candidate.trim()).is_empty() {
                            state.spell_input_buffer = candidate;
                            refresh_spell_interaction_filter(state);
                        }
                    }
                    if !showed_list {
                        refresh_spell_interaction_filter(state);
                    }
                }
                _ => {
                    let note = spell_interaction_help_hint(
                        state,
                        &SpellInteraction::SpellSelect { filtered_indices: Vec::new(), cursor: 0 },
                    );
                    record_spell_note(state, events, note);
                }
            }
        }
    }

    Some(resolution)
}

fn resolve_pending_wizard_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
    bonus_minutes: &mut u64,
) -> Option<WizardInteractionResolution> {
    let Some(interaction) = state.pending_wizard_interaction.clone() else {
        return None;
    };
    let _ = bonus_minutes;
    let input = parse_wizard_input_token(command);
    let mut resolution = WizardInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    match interaction {
        WizardInteraction::EnterWizardConfirm { via_backdoor } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wizard mode request canceled.".to_string());
            }
            WizardInputToken::Text(text) => {
                let key = text.chars().next().map(|ch| ch.to_ascii_lowercase());
                match key {
                    Some('y') => {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        state.wizard.enabled = true;
                        state.wizard.scoring_allowed = false;
                        set_legacy_status_flag(state, LEGACY_STATUS_CHEATED);
                        sync_wizard_flag_with_legacy_bits(state);
                        let note = if via_backdoor {
                            "Backdoor accepted. Wizard mode enabled; this run is score-ineligible."
                                .to_string()
                        } else {
                            "Wizard mode enabled; this run is score-ineligible.".to_string()
                        };
                        record_wizard_note(state, events, note);
                    }
                    Some('n') => {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        record_wizard_note(
                            state,
                            events,
                            "Wizard mode request canceled.".to_string(),
                        );
                    }
                    _ => {
                        let note = wizard_interaction_help_hint(
                            state,
                            &WizardInteraction::EnterWizardConfirm { via_backdoor },
                        );
                        record_wizard_note(state, events, note);
                    }
                }
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::EnterWizardConfirm { via_backdoor },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::WishTextEntry { blessing } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wish canceled.".to_string());
            }
            WizardInputToken::Backspace => {
                state.wizard_input_buffer.pop();
            }
            WizardInputToken::Enter => {
                let raw_wish = state.wizard_input_buffer.trim().to_string();
                state.wizard_input_buffer.clear();
                if raw_wish.is_empty() {
                    state.pending_wizard_interaction = None;
                    record_wizard_note(state, events, "Wish canceled.".to_string());
                } else {
                    let result = resolve_wish_request(state, events, blessing, &raw_wish);
                    record_wizard_note(state, events, result.note);
                    if result.committed {
                        resolution.freeze_world_progression = false;
                        resolution.command_for_accounting =
                            Command::Legacy { token: "^x".to_string() };
                        resolution.turn_minutes = 5;
                    }
                }
            }
            WizardInputToken::Text(text) => {
                append_wizard_buffer(state, &text, 80);
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::WishTextEntry { blessing },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::WishAcquisitionKindSelect { cheated, item_hint } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wish canceled.".to_string());
            }
            WizardInputToken::Text(text) => {
                if let Some(kind) = resolve_item_kind_from_choice_token(&text) {
                    let mut committed = false;
                    if kind == WishItemKind::Artifact && !cheated {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        record_wizard_note(state, events, "You feel stupid.".to_string());
                        committed = true;
                    } else if cheated {
                        let interaction =
                            WizardInteraction::WishAcquisitionItemSelect { cheated, kind };
                        state.pending_wizard_interaction = Some(interaction.clone());
                        state.wizard_input_buffer.clear();
                        let note = wizard_interaction_prompt(state, &interaction);
                        record_wizard_note(state, events, note);
                    } else if let Some(item_name) = random_item_from_kind(state, kind) {
                        let result = add_item_to_inventory_or_ground(state, item_name, events);
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        record_wizard_note(
                            state,
                            events,
                            format!("Acquisition resolved ({result})."),
                        );
                        committed = true;
                    } else {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        record_wizard_note(state, events, "You feel stupid.".to_string());
                        committed = true;
                    }
                    if committed {
                        resolution.freeze_world_progression = false;
                        resolution.command_for_accounting =
                            Command::Legacy { token: "^x".to_string() };
                        resolution.turn_minutes = 5;
                    }
                } else {
                    let note = wizard_interaction_help_hint(
                        state,
                        &WizardInteraction::WishAcquisitionKindSelect { cheated, item_hint },
                    );
                    record_wizard_note(state, events, note);
                }
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::WishAcquisitionKindSelect { cheated, item_hint },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::WishAcquisitionItemSelect { cheated, kind } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wish canceled.".to_string());
            }
            WizardInputToken::Backspace => {
                state.wizard_input_buffer.pop();
            }
            WizardInputToken::Enter => {
                let request = state.wizard_input_buffer.trim().to_string();
                if request.is_empty() {
                    let note = wizard_interaction_help_hint(
                        state,
                        &WizardInteraction::WishAcquisitionItemSelect { cheated, kind },
                    );
                    record_wizard_note(state, events, note);
                } else if let Some(item_name) =
                    resolve_item_selection_by_number_or_name(kind, &request)
                {
                    let result = add_item_to_inventory_or_ground(state, item_name, events);
                    state.pending_wizard_interaction = None;
                    state.wizard_input_buffer.clear();
                    record_wizard_note(state, events, format!("Acquisition resolved ({result})."));
                    resolution.freeze_world_progression = false;
                    resolution.command_for_accounting = Command::Legacy { token: "^x".to_string() };
                    resolution.turn_minutes = 5;
                } else {
                    state.wizard_input_buffer.clear();
                    let note = format!("No {} matched that request.", wish_item_kind_label(kind));
                    record_wizard_note(state, events, note);
                    let next = wizard_interaction_prompt(
                        state,
                        &WizardInteraction::WishAcquisitionItemSelect { cheated, kind },
                    );
                    record_wizard_note(state, events, next);
                }
            }
            WizardInputToken::Text(text) => {
                append_wizard_buffer(state, &text, 80);
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::WishAcquisitionItemSelect { cheated, kind },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::StatusFlagActionSelect => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wizard status editor closed.".to_string());
            }
            WizardInputToken::Text(text) => {
                let key = text.chars().next().map(|ch| ch.to_ascii_lowercase());
                match key {
                    Some('s') => {
                        state.pending_wizard_interaction =
                            Some(WizardInteraction::StatusFlagIndexEntry { set_mode: true });
                        state.wizard_input_buffer.clear();
                        let note = wizard_interaction_prompt(
                            state,
                            &WizardInteraction::StatusFlagIndexEntry { set_mode: true },
                        );
                        record_wizard_note(state, events, note);
                    }
                    Some('r') => {
                        state.pending_wizard_interaction =
                            Some(WizardInteraction::StatusFlagIndexEntry { set_mode: false });
                        state.wizard_input_buffer.clear();
                        let note = wizard_interaction_prompt(
                            state,
                            &WizardInteraction::StatusFlagIndexEntry { set_mode: false },
                        );
                        record_wizard_note(state, events, note);
                    }
                    Some('x') => {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        record_wizard_note(
                            state,
                            events,
                            "Wizard status editor closed.".to_string(),
                        );
                    }
                    _ => {
                        let note = wizard_interaction_help_hint(
                            state,
                            &WizardInteraction::StatusFlagActionSelect,
                        );
                        record_wizard_note(state, events, note);
                    }
                }
            }
            _ => {
                let note =
                    wizard_interaction_help_hint(state, &WizardInteraction::StatusFlagActionSelect);
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::StatusFlagIndexEntry { set_mode } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = Some(WizardInteraction::StatusFlagActionSelect);
                state.wizard_input_buffer.clear();
                let note =
                    wizard_interaction_prompt(state, &WizardInteraction::StatusFlagActionSelect);
                record_wizard_note(state, events, note);
            }
            WizardInputToken::Backspace => {
                state.wizard_input_buffer.pop();
            }
            WizardInputToken::Enter => {
                let parsed = state.wizard_input_buffer.trim().parse::<u8>().ok();
                if let Some(index) = parsed {
                    if index < 64 {
                        let bit = 1u64 << index;
                        if bit == LEGACY_STATUS_CHEATED {
                            record_wizard_note(
                                state,
                                events,
                                "CHEATED bit cannot be modified from the status editor."
                                    .to_string(),
                            );
                        } else {
                            if set_mode {
                                set_legacy_status_flag(state, bit);
                            } else {
                                clear_legacy_status_flag(state, bit);
                            }
                            sync_wizard_flag_with_legacy_bits(state);
                            record_wizard_note(
                                state,
                                events,
                                format!(
                                    "Status bit {} {}.",
                                    index,
                                    if set_mode { "set" } else { "cleared" }
                                ),
                            );
                            state.pending_wizard_interaction =
                                Some(WizardInteraction::StatusFlagActionSelect);
                            state.wizard_input_buffer.clear();
                            return Some(resolution);
                        }
                    } else {
                        record_wizard_note(
                            state,
                            events,
                            "Invalid bit index. Choose a value between 0 and 63.".to_string(),
                        );
                    }
                } else {
                    record_wizard_note(
                        state,
                        events,
                        "Invalid bit index. Enter digits and press Enter.".to_string(),
                    );
                }
                state.wizard_input_buffer.clear();
                let note = wizard_interaction_prompt(
                    state,
                    &WizardInteraction::StatusFlagIndexEntry { set_mode },
                );
                record_wizard_note(state, events, note);
            }
            WizardInputToken::Text(text) => {
                for ch in text.chars() {
                    if ch.is_ascii_digit() && state.wizard_input_buffer.len() < 3 {
                        state.wizard_input_buffer.push(ch);
                    }
                }
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::StatusFlagIndexEntry { set_mode },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::StatEditorSelect { mut slot } => {
            let mut open_value_editor = false;
            let mut close_editor = false;
            match input {
                WizardInputToken::Cancel => {
                    close_editor = true;
                }
                WizardInputToken::Enter => {
                    open_value_editor = true;
                }
                WizardInputToken::DirectionDelta { dx, dy } => {
                    if dx < 0 || dy < 0 {
                        slot = if slot <= 1 { 11 } else { slot - 1 };
                    } else if dx > 0 || dy > 0 {
                        slot = if slot >= 11 { 1 } else { slot + 1 };
                    }
                }
                WizardInputToken::Text(text) => {
                    if text.trim().is_empty() {
                        open_value_editor = true;
                    } else if let Some(ch) = text.chars().next() {
                        match ch.to_ascii_lowercase() {
                            'j' | '<' => {
                                slot = if slot <= 1 { 11 } else { slot - 1 };
                            }
                            'k' | '>' => {
                                slot = if slot >= 11 { 1 } else { slot + 1 };
                            }
                            'x' => {
                                close_editor = true;
                            }
                            '1'..='9' => {
                                slot = ch as u8 - b'0';
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            if close_editor {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Wizard stat editor closed.".to_string());
            } else if open_value_editor {
                state.pending_wizard_interaction =
                    Some(WizardInteraction::StatEditorValueEntry { slot });
                state.wizard_input_buffer.clear();
                let note = wizard_interaction_prompt(
                    state,
                    &WizardInteraction::StatEditorValueEntry { slot },
                );
                record_wizard_note(state, events, note);
            } else {
                state.pending_wizard_interaction =
                    Some(WizardInteraction::StatEditorSelect { slot });
                let note =
                    wizard_interaction_prompt(state, &WizardInteraction::StatEditorSelect { slot });
                record_wizard_note(state, events, note);
            }
        }
        WizardInteraction::StatEditorValueEntry { slot } => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction =
                    Some(WizardInteraction::StatEditorSelect { slot });
                state.wizard_input_buffer.clear();
                let note =
                    wizard_interaction_prompt(state, &WizardInteraction::StatEditorSelect { slot });
                record_wizard_note(state, events, note);
            }
            WizardInputToken::Backspace => {
                state.wizard_input_buffer.pop();
            }
            WizardInputToken::Enter => {
                let value = state.wizard_input_buffer.trim().parse::<i32>();
                let note = match value {
                    Ok(parsed) => apply_stat_slot_value(state, slot, parsed),
                    Err(_) => {
                        "Invalid value; please enter digits and optional leading '-'.".to_string()
                    }
                };
                state.pending_wizard_interaction =
                    Some(WizardInteraction::StatEditorSelect { slot });
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, note);
                let next =
                    wizard_interaction_prompt(state, &WizardInteraction::StatEditorSelect { slot });
                record_wizard_note(state, events, next);
            }
            WizardInputToken::Text(text) => {
                for ch in text.chars() {
                    if ch.is_ascii_digit() {
                        append_wizard_buffer(state, &ch.to_string(), 8);
                    } else if ch == '-' && state.wizard_input_buffer.is_empty() {
                        append_wizard_buffer(state, "-", 8);
                    }
                }
            }
            _ => {
                let note = wizard_interaction_help_hint(
                    state,
                    &WizardInteraction::StatEditorValueEntry { slot },
                );
                record_wizard_note(state, events, note);
            }
        },
        WizardInteraction::BashDirectionSelect => match input {
            WizardInputToken::Cancel => {
                state.pending_wizard_interaction = None;
                state.wizard_input_buffer.clear();
                record_wizard_note(state, events, "Bash canceled.".to_string());
            }
            WizardInputToken::DirectionDelta { dx, dy } => {
                let target =
                    Position { x: state.player.position.x + dx, y: state.player.position.y + dy };
                if state.environment == LegacyEnvironment::City && target.x == 0 && target.y == 0 {
                    state.pending_wizard_interaction =
                        Some(WizardInteraction::EnterWizardConfirm { via_backdoor: true });
                    state.wizard_input_buffer.clear();
                    let note = wizard_interaction_prompt(
                        state,
                        &WizardInteraction::EnterWizardConfirm { via_backdoor: true },
                    );
                    record_wizard_note(state, events, note);
                } else {
                    state.pending_wizard_interaction = None;
                    state.wizard_input_buffer.clear();
                    let (note, _modeled) = apply_destructive_action(state);
                    record_wizard_note(state, events, note);
                }
            }
            WizardInputToken::Text(text) => {
                let parsed = text.chars().next().and_then(direction_delta_from_char);
                if let Some((dx, dy)) = parsed {
                    let target = Position {
                        x: state.player.position.x + dx,
                        y: state.player.position.y + dy,
                    };
                    if state.environment == LegacyEnvironment::City
                        && target.x == 0
                        && target.y == 0
                    {
                        state.pending_wizard_interaction =
                            Some(WizardInteraction::EnterWizardConfirm { via_backdoor: true });
                        state.wizard_input_buffer.clear();
                        let note = wizard_interaction_prompt(
                            state,
                            &WizardInteraction::EnterWizardConfirm { via_backdoor: true },
                        );
                        record_wizard_note(state, events, note);
                    } else {
                        state.pending_wizard_interaction = None;
                        state.wizard_input_buffer.clear();
                        let (note, _modeled) = apply_destructive_action(state);
                        record_wizard_note(state, events, note);
                    }
                } else {
                    let note = wizard_interaction_help_hint(
                        state,
                        &WizardInteraction::BashDirectionSelect,
                    );
                    record_wizard_note(state, events, note);
                }
            }
            _ => {
                let note =
                    wizard_interaction_help_hint(state, &WizardInteraction::BashDirectionSelect);
                record_wizard_note(state, events, note);
            }
        },
    }

    sync_wizard_flag_with_legacy_bits(state);
    Some(resolution)
}

fn begin_inventory_interaction(state: &mut GameState, top_mode: bool) -> (String, bool) {
    let selected_slot = SLOT_READY_HAND;
    let interaction = InventoryInteraction::Control { top_mode, selected_slot };
    state.pending_inventory_interaction = Some(interaction.clone());
    state.interaction_buffer.clear();
    (inventory_interaction_prompt(state, &interaction), true)
}

fn begin_activation_interaction(state: &mut GameState) -> (String, bool) {
    let interaction = ActivationInteraction::ChooseKind;
    state.pending_activation_interaction = Some(interaction.clone());
    state.interaction_buffer.clear();
    (activation_interaction_prompt(&interaction), true)
}

fn begin_item_prompt(
    state: &mut GameState,
    context: ItemPromptContext,
    filter: ItemPromptFilter,
    prompt: String,
) -> (String, bool) {
    let interaction = ItemPromptInteraction { context, filter, prompt };
    if item_prompt_candidate_item_ids(state, &interaction).is_empty() {
        return (format!("{} (no valid items)", interaction.prompt), true);
    }
    state.pending_item_prompt = Some(interaction.clone());
    state.interaction_buffer.clear();
    (item_prompt_prompt(state, &interaction), true)
}

fn classify_inventory_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_inventory_interaction.as_ref() {
        if inventory_interaction_prompt(state, interaction) == note {
            return UiLogClass::Prompt;
        }
        if inventory_interaction_help_hint(interaction) == note {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_inventory_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_inventory_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled { token: "inventory".to_string(), note, fully_modeled: true });
}

fn classify_item_prompt_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_item_prompt.as_ref() {
        if item_prompt_prompt(state, interaction) == note {
            return UiLogClass::Prompt;
        }
        if item_prompt_help_hint(interaction) == note
            || item_prompt_list_choices(state, interaction) == note
        {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_item_prompt_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_item_prompt_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled {
        token: "item_prompt".to_string(),
        note,
        fully_modeled: true,
    });
}

fn classify_activation_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_activation_interaction.as_ref() {
        if activation_interaction_prompt(interaction) == note {
            return UiLogClass::Prompt;
        }
        if activation_interaction_help_hint(interaction) == note {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_activation_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_activation_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled {
        token: "activation".to_string(),
        note,
        fully_modeled: true,
    });
}

fn classify_targeting_note(state: &GameState, note: &str) -> UiLogClass {
    if let Some(interaction) = state.pending_targeting_interaction.as_ref() {
        if targeting_interaction_prompt(state, interaction) == note {
            return UiLogClass::Prompt;
        }
        if targeting_interaction_help_hint(state, interaction) == note {
            return UiLogClass::Hint;
        }
    }
    UiLogClass::Timeline
}

fn record_targeting_note(state: &mut GameState, events: &mut Vec<Event>, note: String) {
    let class = classify_targeting_note(state, &note);
    push_ui_log(state, class, note.clone());
    events.push(Event::LegacyHandled { token: "targeting".to_string(), note, fully_modeled: true });
}

fn statmod(stat: i32) -> i32 {
    (stat - 10) / 2
}

fn projectile_distance(origin: Position, target: Position) -> i32 {
    (origin.x - target.x).abs().max((origin.y - target.y).abs())
}

fn default_target_cursor(state: &GameState, origin: Position, max_range: i32) -> Position {
    if let Some(monster) = state
        .monsters
        .iter()
        .filter(|monster| projectile_distance(origin, monster.position) <= max_range.max(1))
        .min_by_key(|monster| projectile_distance(origin, monster.position))
    {
        return monster.position;
    }
    let fallback =
        Position { x: (origin.x + 1).clamp(0, state.bounds.width.saturating_sub(1)), y: origin.y };
    if state.bounds.contains(fallback) { fallback } else { origin }
}

fn begin_targeting_interaction(state: &mut GameState, action: PendingProjectileAction) -> String {
    let origin = state.player.position;
    let cursor = default_target_cursor(state, origin, action.max_range);
    let interaction = TargetingInteraction { origin, cursor, mode: action.mode };
    state.pending_projectile_action = Some(action);
    state.pending_targeting_interaction = Some(interaction.clone());
    state.target_input_buffer.clear();
    targeting_interaction_prompt(state, &interaction)
}

fn inventory_step_slot(current: usize, delta: i32) -> usize {
    if INVENTORY_SLOT_COUNT == 0 {
        return 0;
    }
    if delta < 0 {
        if current == 0 { INVENTORY_SLOT_COUNT - 1 } else { current - 1 }
    } else if delta > 0 {
        (current + 1) % INVENTORY_SLOT_COUNT
    } else {
        current
    }
}

fn pack_item_ids(state: &GameState) -> Vec<u32> {
    let mut ids = Vec::new();
    for item_id in &state.player.pack_order {
        if ids.contains(item_id) {
            continue;
        }
        if state.player.inventory.iter().any(|item| item.id == *item_id) {
            ids.push(*item_id);
        }
    }
    for item in &state.player.inventory {
        if is_item_equipped(state, item.id) {
            continue;
        }
        if !ids.contains(&item.id) {
            ids.push(item.id);
        }
    }
    ids
}

fn pack_listing(state: &GameState) -> String {
    let mut entries = Vec::new();
    for (idx, item_id) in pack_item_ids(state).into_iter().enumerate() {
        let Some(choice) = item_prompt_choice_key(idx) else {
            break;
        };
        let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id) else {
            continue;
        };
        entries.push(format!("{choice}) {}", item.name));
    }
    if entries.is_empty() {
        "Pack: (empty)".to_string()
    } else {
        format!("Pack: {}", entries.join(", "))
    }
}

fn inventory_look_slot_item(state: &GameState, slot: usize) -> String {
    let Some(item_id) = inventory_slot_item_id(state, slot) else {
        return "Nothing in selected slot!".to_string();
    };
    let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id) else {
        return "Nothing in selected slot!".to_string();
    };
    if !item.known {
        return "You notice nothing new about it.".to_string();
    }
    if !item.truename.is_empty() {
        return format!("It's {}.", item.truename);
    }
    if !item.objstr.is_empty() {
        return format!("It's {}.", item.objstr);
    }
    format!("It's {}.", item.name)
}

fn pack_item_id_from_choice(state: &GameState, choice: char) -> Option<u32> {
    let lowered = choice.to_ascii_lowercase();
    if lowered.is_ascii_digit() {
        let idx = lowered.to_digit(10).unwrap_or(0) as usize;
        if idx == 0 {
            return None;
        }
        return pack_item_ids(state).get(idx - 1).copied();
    }
    let idx = item_prompt_choice_key_index(lowered).unwrap_or(usize::MAX);
    if idx == usize::MAX {
        return None;
    }
    pack_item_ids(state).get(idx).copied()
}

fn item_prompt_choice_key_index(choice: char) -> Option<usize> {
    let lowered = choice.to_ascii_lowercase();
    for idx in 0..26 {
        if item_prompt_choice_key(idx) == Some(lowered) {
            return Some(idx);
        }
    }
    None
}

fn remove_inventory_item_by_id(state: &mut GameState, item_id: u32) -> Option<Item> {
    let idx = state.player.inventory.iter().position(|entry| entry.id == item_id)?;
    let item = state.player.inventory.remove(idx);
    unequip_item_id(&mut state.player.equipment, item.id);
    remove_item_from_pack_order(state, item.id);
    state.carry_burden = state.carry_burden.saturating_sub(item_burden(&item)).max(0);
    Some(item)
}

fn inventory_put_slot_item_to_pack(state: &mut GameState, slot: usize) -> String {
    let Some(item_id) = inventory_slot_item_id(state, slot) else {
        return format!("No item in {} slot.", inventory_slot_name(slot));
    };
    let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id).cloned()
    else {
        return "Selected slot item is missing.".to_string();
    };
    if item_is_cursed_in_use(&item, slot) {
        return format!("{} is cursed and cannot be removed.", item.name);
    }
    unequip_item_id(&mut state.player.equipment, item_id);
    push_item_to_pack_front(state, item_id);
    sync_pack_order(state);
    format!("{} moved to pack.", item.name)
}

fn inventory_drop_slot_item(state: &mut GameState, slot: usize, events: &mut Vec<Event>) -> String {
    let Some(item_id) = inventory_slot_item_id(state, slot) else {
        return format!("No item in {} slot.", inventory_slot_name(slot));
    };
    let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id).cloned()
    else {
        return "Selected slot item is missing.".to_string();
    };
    if item_is_cursed_in_use(&item, slot) {
        return format!("{} is cursed and cannot be dropped.", item.name);
    }
    let Some(item) = remove_inventory_item_by_id(state, item_id) else {
        return "Unable to drop selected item.".to_string();
    };
    let name = item.name.clone();
    if state.world_mode == WorldMode::Countryside {
        events.push(Event::Dropped { item_id: item.id, name: name.clone() });
        format!("Dropped {} in the wilderness.", name)
    } else {
        let position = state.player.position;
        events.push(Event::Dropped { item_id: item.id, name: name.clone() });
        state.ground_items.push(GroundItem { position, item });
        format!("Dropped {}.", name)
    }
}

fn inventory_equip_pack_item_to_slot(state: &mut GameState, slot: usize, item_id: u32) -> String {
    let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id).cloned()
    else {
        return "That pack entry is no longer available.".to_string();
    };
    if !slot_accepts_item(slot, &item) {
        return format!("{} cannot be equipped in {} slot.", item.name, inventory_slot_name(slot));
    }
    if slot == SLOT_SHIELD && equipped_weapon_is_two_handed(state) {
        return "Cannot equip a shield while wielding a two-handed weapon.".to_string();
    }
    if let Some(existing_id) = inventory_slot_item_id(state, slot)
        && existing_id != item_id
    {
        let existing = state.player.inventory.iter().find(|entry| entry.id == existing_id).cloned();
        if let Some(existing) = existing
            && item_is_cursed_in_use(&existing, slot)
        {
            return format!("{} is cursed and cannot be removed.", existing.name);
        }
    }

    let displaced = inventory_slot_item_id(state, slot);
    if displaced == Some(item_id) {
        remove_item_from_pack_order(state, item_id);
        sync_pack_order(state);
        return format!("{} is already in {} slot.", item.name, inventory_slot_name(slot));
    }

    if let Some(existing_id) = displaced {
        set_inventory_slot_item_id(state, slot, None);
        push_item_to_pack_front(state, existing_id);
    }
    unequip_item_id(&mut state.player.equipment, item_id);
    let _ = set_inventory_slot_item_id(state, slot, Some(item_id));
    remove_item_from_pack_order(state, item_id);

    if is_two_handed_weapon(&item) && matches!(slot, SLOT_READY_HAND | SLOT_WEAPON_HAND) {
        state.player.equipment.weapon_hand = Some(item_id);
        state.player.equipment.ready_hand = Some(item_id);
        if let Some(shield_id) = state.player.equipment.shield.take() {
            push_item_to_pack_front(state, shield_id);
        }
    }

    if slot == SLOT_SHIELD
        && let Some(weapon_id) = state.player.equipment.weapon_hand
        && let Some(weapon) = state.player.inventory.iter().find(|entry| entry.id == weapon_id)
        && is_two_handed_weapon(weapon)
    {
        state.player.equipment.shield = None;
        push_item_to_pack_front(state, item_id);
        return "Cannot equip a shield while wielding a two-handed weapon.".to_string();
    }

    sync_pack_order(state);
    format!("{} equipped to {} slot.", item.name, inventory_slot_name(slot))
}

fn resolve_pending_activation_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> Option<ActivationInteractionResolution> {
    let Some(interaction) = state.pending_activation_interaction.clone() else {
        return None;
    };
    let resolution = ActivationInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    match interaction {
        ActivationInteraction::ChooseKind => {
            let input = parse_wizard_input_token(command);
            match input {
                WizardInputToken::Cancel => {
                    state.pending_activation_interaction = None;
                    state.interaction_buffer.clear();
                    record_activation_note(state, events, "Activate canceled.".to_string());
                }
                WizardInputToken::Text(text) => {
                    let key = text.chars().next().map(|ch| ch.to_ascii_lowercase());
                    match key {
                        Some('i') => {
                            state.pending_activation_interaction = None;
                            state.interaction_buffer.clear();
                            let (note, _modeled) = begin_item_prompt(
                                state,
                                ItemPromptContext::ActivateThing,
                                ItemPromptFilter::Families(vec![ItemFamily::Thing]),
                                "Activate --".to_string(),
                            );
                            record_item_prompt_note(state, events, note);
                        }
                        Some('a') => {
                            state.pending_activation_interaction = None;
                            state.interaction_buffer.clear();
                            let (note, _modeled) = begin_item_prompt(
                                state,
                                ItemPromptContext::ActivateArtifact,
                                ItemPromptFilter::Families(vec![ItemFamily::Artifact]),
                                "Activate --".to_string(),
                            );
                            record_item_prompt_note(state, events, note);
                        }
                        _ => {
                            let note = activation_interaction_help_hint(
                                &ActivationInteraction::ChooseKind,
                            );
                            record_activation_note(state, events, note);
                        }
                    }
                }
                _ => {
                    let note = activation_interaction_help_hint(&ActivationInteraction::ChooseKind);
                    record_activation_note(state, events, note);
                }
            }
        }
    }

    Some(resolution)
}

fn resolve_pending_inventory_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> Option<InventoryInteractionResolution> {
    let Some(interaction) = state.pending_inventory_interaction.clone() else {
        return None;
    };
    let mut resolution = InventoryInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    match interaction {
        InventoryInteraction::Control { top_mode, mut selected_slot } => {
            if let Command::Drop { slot } = command {
                selected_slot = (*slot).min(INVENTORY_SLOT_COUNT.saturating_sub(1));
                state.pending_inventory_interaction =
                    Some(InventoryInteraction::Control { top_mode, selected_slot });
                let note = inventory_interaction_prompt(
                    state,
                    &InventoryInteraction::Control { top_mode, selected_slot },
                );
                record_inventory_note(state, events, note);
                return Some(resolution);
            }

            let input = parse_wizard_input_token(command);
            match input {
                WizardInputToken::Cancel => {
                    state.pending_inventory_interaction = None;
                    state.interaction_buffer.clear();
                    record_inventory_note(
                        state,
                        events,
                        "Inventory interaction closed.".to_string(),
                    );
                }
                WizardInputToken::Text(text) => {
                    let key = text.chars().next().map(|ch| ch.to_ascii_lowercase());
                    match key {
                        Some('?') => {
                            record_inventory_note(state, events, pack_listing(state));
                        }
                        Some('<') | Some('j') => {
                            selected_slot = inventory_step_slot(selected_slot, -1);
                            state.pending_inventory_interaction =
                                Some(InventoryInteraction::Control { top_mode, selected_slot });
                            let note = inventory_interaction_prompt(
                                state,
                                &InventoryInteraction::Control { top_mode, selected_slot },
                            );
                            record_inventory_note(state, events, note);
                        }
                        Some('>') | Some('k') => {
                            selected_slot = inventory_step_slot(selected_slot, 1);
                            state.pending_inventory_interaction =
                                Some(InventoryInteraction::Control { top_mode, selected_slot });
                            let note = inventory_interaction_prompt(
                                state,
                                &InventoryInteraction::Control { top_mode, selected_slot },
                            );
                            record_inventory_note(state, events, note);
                        }
                        Some('d') => {
                            let note = inventory_drop_slot_item(state, selected_slot, events);
                            state.pending_inventory_interaction =
                                Some(InventoryInteraction::Control { top_mode, selected_slot });
                            record_inventory_note(state, events, note);
                            resolution.freeze_world_progression = false;
                            resolution.command_for_accounting =
                                Command::Legacy { token: "d".to_string() };
                            resolution.turn_minutes = 1;
                        }
                        Some('p') => {
                            let note = inventory_put_slot_item_to_pack(state, selected_slot);
                            state.pending_inventory_interaction =
                                Some(InventoryInteraction::Control { top_mode, selected_slot });
                            record_inventory_note(state, events, note);
                            resolution.freeze_world_progression = false;
                            resolution.command_for_accounting =
                                Command::Legacy { token: "p".to_string() };
                            resolution.turn_minutes = 5;
                        }
                        Some('s') => {
                            let note = pack_listing(state);
                            record_inventory_note(state, events, note);
                        }
                        Some('l') => {
                            let note = inventory_look_slot_item(state, selected_slot);
                            record_inventory_note(state, events, note);
                        }
                        Some('t') | Some('e') => {
                            state.pending_inventory_interaction =
                                Some(InventoryInteraction::TakeFromPackSelect {
                                    top_mode,
                                    selected_slot,
                                });
                            let note = inventory_interaction_prompt(
                                state,
                                &InventoryInteraction::TakeFromPackSelect {
                                    top_mode,
                                    selected_slot,
                                },
                            );
                            record_inventory_note(state, events, note);
                        }
                        Some('x') => {
                            state.pending_inventory_interaction = None;
                            state.interaction_buffer.clear();
                            record_inventory_note(
                                state,
                                events,
                                "Inventory interaction closed.".to_string(),
                            );
                        }
                        Some(letter) => {
                            if let Some(slot) = legacy_inventory_key_to_slot(letter) {
                                selected_slot = slot;
                                state.pending_inventory_interaction =
                                    Some(InventoryInteraction::Control { top_mode, selected_slot });
                                let note = inventory_interaction_prompt(
                                    state,
                                    &InventoryInteraction::Control { top_mode, selected_slot },
                                );
                                record_inventory_note(state, events, note);
                            } else {
                                let note = inventory_interaction_help_hint(
                                    &InventoryInteraction::Control { top_mode, selected_slot },
                                );
                                record_inventory_note(state, events, note);
                            }
                        }
                        None => {
                            let note =
                                inventory_interaction_help_hint(&InventoryInteraction::Control {
                                    top_mode,
                                    selected_slot,
                                });
                            record_inventory_note(state, events, note);
                        }
                    }
                }
                _ => {
                    let note = inventory_interaction_help_hint(&InventoryInteraction::Control {
                        top_mode,
                        selected_slot,
                    });
                    record_inventory_note(state, events, note);
                }
            }
        }
        InventoryInteraction::TakeFromPackSelect { top_mode, selected_slot } => {
            let mut selected_item_id = if let Command::Drop { slot } = command {
                pack_item_ids(state).get(*slot).copied()
            } else {
                None
            };
            if selected_item_id.is_none() {
                let input = parse_wizard_input_token(command);
                match input {
                    WizardInputToken::Cancel => {
                        state.pending_inventory_interaction =
                            Some(InventoryInteraction::Control { top_mode, selected_slot });
                        let note = inventory_interaction_prompt(
                            state,
                            &InventoryInteraction::Control { top_mode, selected_slot },
                        );
                        record_inventory_note(state, events, note);
                        return Some(resolution);
                    }
                    WizardInputToken::Text(text) => {
                        if text.trim() == "?" {
                            record_inventory_note(state, events, pack_listing(state));
                            return Some(resolution);
                        }
                        if let Some(ch) = text.chars().next() {
                            selected_item_id = pack_item_id_from_choice(state, ch);
                        }
                    }
                    _ => {
                        let note = inventory_interaction_help_hint(
                            &InventoryInteraction::TakeFromPackSelect { top_mode, selected_slot },
                        );
                        record_inventory_note(state, events, note);
                        return Some(resolution);
                    }
                }
            }

            if let Some(item_id) = selected_item_id {
                let note = inventory_equip_pack_item_to_slot(state, selected_slot, item_id);
                state.pending_inventory_interaction =
                    Some(InventoryInteraction::Control { top_mode, selected_slot });
                record_inventory_note(state, events, note);
                resolution.freeze_world_progression = false;
                resolution.command_for_accounting = Command::Legacy { token: "t".to_string() };
                resolution.turn_minutes = 5;
            } else {
                let note =
                    inventory_interaction_help_hint(&InventoryInteraction::TakeFromPackSelect {
                        top_mode,
                        selected_slot,
                    });
                record_inventory_note(state, events, note);
            }
        }
    }

    Some(resolution)
}

fn item_prompt_context_token(context: &ItemPromptContext) -> &'static str {
    match context {
        ItemPromptContext::Quaff => "q",
        ItemPromptContext::Read => "r",
        ItemPromptContext::Eat => "e",
        ItemPromptContext::Drop => "d",
        ItemPromptContext::FireThrow => "f",
        ItemPromptContext::ActivateThing => "a",
        ItemPromptContext::ZapStick => "z",
        ItemPromptContext::ActivateArtifact => "A",
        ItemPromptContext::CallItem => "C",
        ItemPromptContext::Give => "G",
    }
}

fn item_prompt_turn_minutes(context: &ItemPromptContext) -> u64 {
    match context {
        ItemPromptContext::Drop => 1,
        ItemPromptContext::Eat => 2,
        ItemPromptContext::CallItem => 0,
        ItemPromptContext::Give => 5,
        ItemPromptContext::Quaff
        | ItemPromptContext::Read
        | ItemPromptContext::FireThrow
        | ItemPromptContext::ActivateThing
        | ItemPromptContext::ZapStick
        | ItemPromptContext::ActivateArtifact => 5,
    }
}

fn item_prompt_choice_pairs(
    state: &GameState,
    interaction: &ItemPromptInteraction,
) -> Vec<(char, u32)> {
    item_prompt_candidate_item_ids(state, interaction)
        .into_iter()
        .enumerate()
        .filter_map(|(idx, item_id)| item_prompt_choice_key(idx).map(|key| (key, item_id)))
        .collect()
}

fn item_prompt_selection_from_key(
    state: &GameState,
    interaction: &ItemPromptInteraction,
    key: char,
) -> Option<u32> {
    let lowered = key.to_ascii_lowercase();
    if lowered.is_ascii_digit() {
        let idx = lowered.to_digit(10).unwrap_or(0) as usize;
        if idx == 0 {
            return None;
        }
        return item_prompt_choice_pairs(state, interaction)
            .get(idx - 1)
            .map(|(_, item_id)| *item_id);
    }
    item_prompt_choice_pairs(state, interaction)
        .into_iter()
        .find_map(|(choice, item_id)| (choice == lowered).then_some(item_id))
}

fn item_prompt_selection_from_index(
    state: &GameState,
    interaction: &ItemPromptInteraction,
    index: usize,
) -> Option<u32> {
    item_prompt_choice_pairs(state, interaction).get(index).map(|(_, item_id)| *item_id)
}

fn item_prompt_list_choices(state: &GameState, interaction: &ItemPromptInteraction) -> String {
    let mut parts = Vec::new();
    for (choice, item_id) in item_prompt_choice_pairs(state, interaction) {
        let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id) else {
            continue;
        };
        parts.push(format!("{choice}) {}", item.name));
    }
    if parts.is_empty() {
        format!("{}: no valid items.", interaction.prompt)
    } else {
        format!("{} {}", interaction.prompt, parts.join(", "))
    }
}

fn is_arrow_item(item: &Item) -> bool {
    let contract = legacy_projectile_contract();
    item.legacy_id == contract.ob_arrow
        || item.aux == contract.i_arrow
        || normalize_item_lookup(&item.name).contains("arrow")
}

fn is_bolt_item(item: &Item) -> bool {
    let contract = legacy_projectile_contract();
    item.legacy_id == contract.ob_bolt
        || item.aux == contract.i_bolt
        || normalize_item_lookup(&item.name).contains("bolt")
}

fn is_scythe_throw(item: &Item) -> bool {
    let contract = legacy_projectile_contract();
    item.aux == contract.i_scythe || normalize_item_lookup(&item.name).contains("scythe")
}

fn projectile_kind_for_item(item: &Item) -> ProjectileKind {
    if is_arrow_item(item) {
        ProjectileKind::Arrow
    } else if is_bolt_item(item) {
        ProjectileKind::Bolt
    } else {
        ProjectileKind::ThrownItem
    }
}

fn remove_single_inventory_unit_by_id(state: &mut GameState, item_id: u32) -> Option<Item> {
    let idx = state.player.inventory.iter().position(|entry| entry.id == item_id)?;
    if state.player.inventory[idx].number > 1 {
        state.player.inventory[idx].number -= 1;
        let mut unit = state.player.inventory[idx].clone();
        unit.number = 1;
        unit.id = state.next_item_id;
        state.next_item_id = state.next_item_id.wrapping_add(1);
        return Some(unit);
    }
    remove_inventory_item_by_id(state, item_id)
}

fn begin_fire_throw_for_item(
    state: &mut GameState,
    item_id: u32,
    _events: &mut Vec<Event>,
) -> String {
    let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id).cloned()
    else {
        return "That item is no longer available.".to_string();
    };
    if item.family == ItemFamily::Cash {
        return "Can't fire money at something!".to_string();
    }
    if item.blessing < 0 && item.used {
        return "You can't seem to get rid of it!".to_string();
    }
    if weapon_hand_is_crossbow(state) && !weapon_hand_crossbow_loaded(state) && is_bolt_item(&item)
    {
        set_weapon_hand_crossbow_loaded(state, true);
        return "You crank back the crossbow and load a bolt.".to_string();
    }

    let profile = equipment_effect_profile(state);
    let mode = projectile_kind_for_item(&item);
    let mut damage_min = item.dmg.max(1);
    let mut damage_max = (item.dmg + item.plus.max(0) + 2).max(damage_min + 1);
    if mode == ProjectileKind::ThrownItem {
        let throw_mod = 2 * statmod(state.attributes.strength.max(1));
        damage_min = (damage_min + throw_mod).max(1);
        damage_max = (damage_max + throw_mod).max(damage_min + 1);
    }
    let action = PendingProjectileAction {
        source_token: "f".to_string(),
        turn_minutes: estimate_legacy_turn_minutes("f", state.world_mode, state.options.searchnum),
        mode,
        item_id: Some(item.id),
        item_name: item.name.clone(),
        hit_bonus: profile.to_hit_bonus + item.hit + statmod(state.attributes.dexterity.max(1)),
        damage_bonus: item.plus.max(0),
        damage_min,
        damage_max,
        damage_type: ProjectileDamageType::Normal,
        max_range: 12,
        allows_drop: true,
    };
    let _ = begin_targeting_interaction(state, action);
    format!("You ready {}.", item.name)
}

fn apply_item_prompt_selection(
    state: &mut GameState,
    interaction: &ItemPromptInteraction,
    item_id: u32,
    events: &mut Vec<Event>,
) -> String {
    match interaction.context {
        ItemPromptContext::Quaff => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let effect_note = apply_item_usef_effect(state, &item, events);
            format!("Quaffed {} ({effect_note}).", item.name)
        }
        ItemPromptContext::Read => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let effect_note = apply_item_usef_effect(state, &item, events);
            if state.progression.quest_state == LegacyQuestState::NotStarted {
                state.progression.quest_state = LegacyQuestState::Active;
                state.progression.quest_steps_completed = 1;
                events.push(Event::QuestAdvanced {
                    state: state.progression.quest_state,
                    steps_completed: state.progression.quest_steps_completed,
                });
            }
            format!("Read {} ({effect_note}).", item.name)
        }
        ItemPromptContext::Eat => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let effect_note = apply_item_usef_effect(state, &item, events);
            format!("Ate {} ({effect_note}).", item.name)
        }
        ItemPromptContext::Drop => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let name = item.name.clone();
            events.push(Event::Dropped { item_id: item.id, name: name.clone() });
            if state.world_mode == WorldMode::Countryside {
                format!("Dropped {} in the wilderness.", name)
            } else {
                let position = state.player.position;
                state.ground_items.push(GroundItem { position, item });
                format!("Dropped {}.", name)
            }
        }
        ItemPromptContext::FireThrow => begin_fire_throw_for_item(state, item_id, events),
        ItemPromptContext::ActivateThing => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let effect_note = apply_item_usef_effect(state, &item, events);
            format!("Activated {} ({effect_note}).", item.name)
        }
        ItemPromptContext::ZapStick => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            push_or_refresh_status(&mut state.status_effects, "wand_charge", 1, 0);
            let effect_note = apply_item_usef_effect(state, &item, events);
            format!("Activated {} ({effect_note}).", item.name)
        }
        ItemPromptContext::ActivateArtifact => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            let effect_note = apply_item_usef_effect(state, &item, events);
            state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.max(2);
            state.resistances.magic = state.resistances.magic.max(2);
            state.immunities.fear = true;
            events.push(Event::QuestAdvanced {
                state: state.progression.quest_state,
                steps_completed: state.progression.quest_steps_completed,
            });
            format!("Activated {} ({effect_note}).", item.name)
        }
        ItemPromptContext::CallItem => {
            let Some(item) = state.player.inventory.iter_mut().find(|entry| entry.id == item_id)
            else {
                return "That item is no longer available.".to_string();
            };
            if !item.name.starts_with("inscribed ") {
                item.name = format!("inscribed {}", item.name);
            }
            format!("Named item: {}.", item.name)
        }
        ItemPromptContext::Give => {
            let Some(item) = remove_inventory_item_by_id(state, item_id) else {
                return "That item is no longer available.".to_string();
            };
            state.progression.deity_favor += 2;
            state.progression.law_chaos_score += 1;
            format!("Gifted {}.", item.name)
        }
    }
}

fn resolve_pending_item_prompt_interaction<R: RandomSource>(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
    rng: &mut R,
    bonus_minutes: &mut u64,
) -> Option<ItemPromptInteractionResolution> {
    let Some(interaction) = state.pending_item_prompt.clone() else {
        return None;
    };
    let _ = (rng, bonus_minutes);
    let mut resolution = ItemPromptInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    if let Command::Drop { slot } = command {
        if let Some(item_id) = item_prompt_selection_from_index(state, &interaction, *slot) {
            let note = apply_item_prompt_selection(state, &interaction, item_id, events);
            state.pending_item_prompt = None;
            state.interaction_buffer.clear();
            record_item_prompt_note(state, events, note);
            if state.pending_targeting_interaction.is_some() {
                resolution.freeze_world_progression = true;
                resolution.command_for_accounting = Command::Legacy { token: "F".to_string() };
                resolution.turn_minutes = 0;
            } else {
                resolution.freeze_world_progression = false;
                resolution.command_for_accounting = Command::Legacy {
                    token: item_prompt_context_token(&interaction.context).to_string(),
                };
                resolution.turn_minutes = item_prompt_turn_minutes(&interaction.context);
            }
        } else {
            let note = item_prompt_help_hint(&interaction);
            record_item_prompt_note(state, events, note);
        }
        return Some(resolution);
    }

    let input = parse_wizard_input_token(command);
    match input {
        WizardInputToken::Cancel => {
            state.pending_item_prompt = None;
            state.interaction_buffer.clear();
            record_item_prompt_note(state, events, "Item prompt canceled.".to_string());
        }
        WizardInputToken::Enter => {
            let choices = item_prompt_choice_pairs(state, &interaction);
            if choices.len() == 1 {
                let item_id = choices[0].1;
                let note = apply_item_prompt_selection(state, &interaction, item_id, events);
                state.pending_item_prompt = None;
                state.interaction_buffer.clear();
                record_item_prompt_note(state, events, note);
                if state.pending_targeting_interaction.is_some() {
                    resolution.freeze_world_progression = true;
                    resolution.command_for_accounting = Command::Legacy { token: "F".to_string() };
                    resolution.turn_minutes = 0;
                } else {
                    resolution.freeze_world_progression = false;
                    resolution.command_for_accounting = Command::Legacy {
                        token: item_prompt_context_token(&interaction.context).to_string(),
                    };
                    resolution.turn_minutes = item_prompt_turn_minutes(&interaction.context);
                }
            } else {
                let note = item_prompt_help_hint(&interaction);
                record_item_prompt_note(state, events, note);
            }
        }
        WizardInputToken::Text(text) => {
            if text.trim() == "?" {
                let note = item_prompt_list_choices(state, &interaction);
                record_item_prompt_note(state, events, note);
            } else if let Some(ch) = text.chars().next() {
                if let Some(item_id) = item_prompt_selection_from_key(state, &interaction, ch) {
                    let note = apply_item_prompt_selection(state, &interaction, item_id, events);
                    state.pending_item_prompt = None;
                    state.interaction_buffer.clear();
                    record_item_prompt_note(state, events, note);
                    if state.pending_targeting_interaction.is_some() {
                        resolution.freeze_world_progression = true;
                        resolution.command_for_accounting =
                            Command::Legacy { token: "F".to_string() };
                        resolution.turn_minutes = 0;
                    } else {
                        resolution.freeze_world_progression = false;
                        resolution.command_for_accounting = Command::Legacy {
                            token: item_prompt_context_token(&interaction.context).to_string(),
                        };
                        resolution.turn_minutes = item_prompt_turn_minutes(&interaction.context);
                    }
                } else {
                    let note = item_prompt_help_hint(&interaction);
                    record_item_prompt_note(state, events, note);
                }
            } else {
                let note = item_prompt_help_hint(&interaction);
                record_item_prompt_note(state, events, note);
            }
        }
        _ => {
            let note = item_prompt_help_hint(&interaction);
            record_item_prompt_note(state, events, note);
        }
    }

    Some(resolution)
}

fn clamp_target_to_range(origin: Position, requested: Position, max_range: i32) -> Position {
    if max_range <= 0 {
        return origin;
    }
    let path = line_path(origin, requested);
    let max_steps = usize::try_from(max_range).unwrap_or(0);
    if path.len() > max_steps.saturating_add(1) { path[max_steps] } else { requested }
}

fn resolve_projectile_action<R: RandomSource>(
    state: &mut GameState,
    action: &PendingProjectileAction,
    target: Position,
    events: &mut Vec<Event>,
    rng: &mut R,
) -> ProjectileResolution {
    let origin = state.player.position;
    let bounded_target = clamp_target_to_range(origin, target, action.max_range.max(1));
    let final_pos = projectile_trace_to_target(state, origin, bounded_target, true);
    let mut rendered_path = line_path(origin, final_pos);
    if !rendered_path.is_empty() {
        rendered_path.remove(0);
    }
    state.transient_projectile_path = rendered_path;
    state.transient_projectile_impact = if final_pos != origin { Some(final_pos) } else { None };
    let mut lines = Vec::new();
    let mut hit_monster_id = None;
    let mut dropped_item = None;
    let mut consumed_item = false;

    match action.mode {
        ProjectileKind::ThrownItem => lines.push(format!("You throw {}.", action.item_name)),
        ProjectileKind::Arrow | ProjectileKind::Bolt => {
            lines.push(format!("You fire {}.", action.item_name))
        }
        ProjectileKind::MagicMissile | ProjectileKind::FireBolt | ProjectileKind::LightningBolt => {
            lines.push(format!("You cast {}.", action.item_name))
        }
    }

    let mut launched_item =
        action.item_id.and_then(|item_id| remove_single_inventory_unit_by_id(state, item_id));
    if action.item_id.is_some() && launched_item.is_none() {
        lines.push("No projectile item is available.".to_string());
        return ProjectileResolution {
            final_pos: origin,
            hit_monster_id: None,
            dropped_item: None,
            consumed_item: false,
            log_lines: lines,
        };
    }

    let crossbow_loaded_before_shot = weapon_hand_crossbow_loaded(state);
    let forced_scythe_miss = launched_item.as_ref().is_some_and(is_scythe_throw);
    let maybe_monster_idx = monster_index_at(state, final_pos);

    if let Some(monster_idx) = maybe_monster_idx {
        let monster_ac = state.monsters[monster_idx].stats.defense;
        let to_hit = state.player.stats.attack_max + action.hit_bonus;
        let hit = !forced_scythe_miss && legacy_hit_roll(to_hit, monster_ac, rng);
        if forced_scythe_miss {
            lines.push("It isn't very aerodynamic... you miss.".to_string());
        } else if hit {
            let mut damage_min = action.damage_min.max(1);
            let mut damage_max = action.damage_max.max(damage_min);
            let player_damage_component =
                ((state.player.stats.attack_min + state.player.stats.attack_max) / 2).max(1);
            if action.mode == ProjectileKind::Arrow && weapon_hand_is_longbow(state) {
                damage_min += player_damage_component / 2;
                damage_max += player_damage_component;
            }
            if action.mode == ProjectileKind::Bolt
                && weapon_hand_is_crossbow(state)
                && crossbow_loaded_before_shot
            {
                damage_min += player_damage_component / 2;
                damage_max += player_damage_component;
            }
            let rolled = rng.range_inclusive_i32(damage_min, damage_max.max(damage_min));
            let resolved_damage = (rolled + action.damage_bonus).max(1);
            let (monster_id, monster_name, remaining_hp, defeated, applied) = {
                let monster = &mut state.monsters[monster_idx];
                let applied = monster.stats.apply_damage(resolved_damage);
                (
                    monster.id,
                    monster.name.clone(),
                    monster.stats.hp,
                    !monster.stats.is_alive(),
                    applied,
                )
            };
            hit_monster_id = Some(monster_id);
            events.push(Event::Attacked { monster_id, damage: applied, remaining_hp });
            lines.push(format!(
                "{} hits {} for {} damage.",
                action.item_name, monster_name, applied
            ));
            if action.mode == ProjectileKind::Bolt && crossbow_loaded_before_shot {
                set_weapon_hand_crossbow_loaded(state, false);
            }
            if defeated {
                let _ = remove_monster_with_drops(state, monster_idx, events);
                state.monsters_defeated = state.monsters_defeated.saturating_add(1);
                events.push(Event::MonsterDefeated { monster_id });
                lines.push(format!("{monster_name} is defeated."));
            }
        } else {
            lines.push("You miss it.".to_string());
        }
    } else if final_pos == origin && bounded_target != origin {
        lines.push("The shot is blocked.".to_string());
    }

    if let Some(item) = launched_item.take() {
        let mut should_break = false;
        if hit_monster_id.is_some()
            && matches!(action.mode, ProjectileKind::Arrow | ProjectileKind::Bolt)
        {
            should_break = rng.range_inclusive_i32(0, 3) == 0;
        }
        if should_break {
            consumed_item = true;
            lines.push(format!("The {} breaks.", item.name));
        } else if action.allows_drop {
            state.ground_items.push(GroundItem { position: final_pos, item: item.clone() });
            dropped_item = Some(item.name.clone());
        }
    }

    ProjectileResolution {
        final_pos,
        hit_monster_id,
        dropped_item,
        consumed_item,
        log_lines: lines,
    }
}

fn resolve_pending_targeting_interaction<R: RandomSource>(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
    rng: &mut R,
) -> Option<TargetingInteractionResolution> {
    let Some(interaction) = state.pending_targeting_interaction.clone() else {
        return None;
    };
    let mut resolution = TargetingInteractionResolution {
        freeze_world_progression: true,
        command_for_accounting: Command::Legacy { token: "F".to_string() },
        turn_minutes: 0,
    };

    let mut commit = false;
    let mut show_hint = false;
    let mut maybe_delta: Option<(i32, i32)> = None;

    match parse_wizard_input_token(command) {
        WizardInputToken::Cancel => {
            state.pending_targeting_interaction = None;
            state.pending_projectile_action = None;
            state.target_input_buffer.clear();
            record_targeting_note(state, events, "Targeting canceled.".to_string());
            return Some(resolution);
        }
        WizardInputToken::Enter => {
            commit = true;
        }
        WizardInputToken::DirectionDelta { dx, dy } => {
            maybe_delta = Some((dx, dy));
        }
        WizardInputToken::Text(text) => {
            for ch in text.chars() {
                if ch == '?' {
                    show_hint = true;
                    continue;
                }
                if ch == '.' {
                    commit = true;
                    continue;
                }
                if let Some((dx, dy)) = direction_delta_from_char(ch) {
                    maybe_delta = Some((dx, dy));
                    continue;
                }
                if let Some((dx, dy)) = parse_direction_delta_from_text(&ch.to_string()) {
                    maybe_delta = Some((dx, dy));
                }
            }
        }
        _ => {
            show_hint = true;
        }
    }

    if let Some((dx, dy)) = maybe_delta {
        let cursor = Position {
            x: (interaction.cursor.x + dx).clamp(0, state.bounds.width.saturating_sub(1)),
            y: (interaction.cursor.y + dy).clamp(0, state.bounds.height.saturating_sub(1)),
        };
        state.pending_targeting_interaction = Some(TargetingInteraction {
            origin: interaction.origin,
            cursor,
            mode: interaction.mode,
        });
    }

    if show_hint {
        let hint = targeting_interaction_help_hint(state, &interaction);
        record_targeting_note(state, events, hint);
    }

    if commit {
        let Some(action) = state.pending_projectile_action.clone() else {
            state.pending_targeting_interaction = None;
            state.target_input_buffer.clear();
            record_targeting_note(state, events, "Targeting canceled.".to_string());
            return Some(resolution);
        };
        let target = state
            .pending_targeting_interaction
            .as_ref()
            .map(|it| it.cursor)
            .unwrap_or(interaction.cursor);
        let resolved = resolve_projectile_action(state, &action, target, events, rng);
        state.pending_targeting_interaction = None;
        state.pending_projectile_action = None;
        state.target_input_buffer.clear();
        if resolved.log_lines.is_empty() {
            record_targeting_note(state, events, "Projectile resolved.".to_string());
        } else {
            for line in resolved.log_lines {
                record_targeting_note(state, events, line);
            }
        }
        resolution.freeze_world_progression = false;
        resolution.command_for_accounting = Command::Legacy { token: action.source_token.clone() };
        resolution.turn_minutes = action.turn_minutes;
    }

    Some(resolution)
}

fn parse_site_interaction_choice(
    state: &GameState,
    kind: &SiteInteractionKind,
    token: &str,
) -> Option<usize> {
    let trimmed = token.trim();
    if let Ok(choice) = trimmed.parse::<usize>() {
        return Some(choice);
    }
    let key = trimmed.chars().next()?.to_ascii_lowercase();
    match kind {
        SiteInteractionKind::Shop => match key {
            'r' => Some(1),
            'p' => Some(2),
            'i' => Some(3),
            'l' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Armorer => match key {
            'a' => Some(1),
            'w' => Some(2),
            'r' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Club => match key {
            'm' => Some(1),
            'l' => Some(2),
            'x' | 'q' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Gym => match key {
            'd' => Some(1),
            's' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Healer => match key {
            'h' => Some(1),
            'c' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Casino => match key {
            'b' => Some(1),
            'p' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Commandant => match key {
            'b' => Some(1),
            'r' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Diner => match key {
            'm' => Some(1),
            'c' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Craps => match key {
            'b' => Some(1),
            'r' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Tavern => match key {
            'a' => Some(1),
            'm' => Some(2),
            'r' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::PawnShop => match key {
            'b' => Some(1),
            's' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Brothel => match key {
            'r' => Some(1),
            'g' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Condo => match key {
            'r' => Some(1),
            's' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Bank => match key {
            'd' => Some(1),
            'w' => Some(2),
            's' => Some(3),
            'l' => Some(4),
            _ => None,
        },
        SiteInteractionKind::MercGuild => match key {
            't' => Some(1),
            'c' => Some(2),
            'p' => Some(3),
            'l' => Some(4),
            _ => None,
        },
        SiteInteractionKind::ThievesGuild => match key {
            'j' => Some(1),
            'h' => Some(2),
            'p' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Temple => match key {
            't' => Some(1),
            'p' => Some(2),
            'b' => Some(3),
            's' => Some(4),
            'l' | 'x' => Some(5),
            _ => None,
        },
        SiteInteractionKind::College => match key {
            'm' => Some(1),
            'l' => Some(2),
            'i' => Some(3),
            'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Sorcerors => match key {
            'r' => Some(1),
            'd' => Some(2),
            't' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Castle => match key {
            'f' => Some(1),
            'a' => Some(2),
            'p' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Palace => match key {
            'a' => Some(1),
            'p' => Some(2),
            'l' | 'x' => Some(3),
            _ => None,
        },
        SiteInteractionKind::Order => match key {
            'v' => Some(1),
            'a' => Some(2),
            'u' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Charity => match key {
            'm' => Some(1),
            'c' => Some(2),
            'v' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Monastery => match key {
            'm' => Some(1),
            'd' => Some(2),
            'v' => Some(3),
            'l' | 'x' => Some(4),
            _ => None,
        },
        SiteInteractionKind::Arena => {
            if state.progression.arena_rank > 0 {
                match key {
                    'y' | 'e' => Some(1),
                    'n' | 'l' => Some(2),
                    _ => None,
                }
            } else {
                match key {
                    'e' | 'y' => Some(1),
                    'r' => Some(2),
                    'n' | 'l' => Some(3),
                    _ => None,
                }
            }
        }
        SiteInteractionKind::Altar { .. } => {
            if altar_needs_initial_worship(state) {
                match key {
                    'y' | 'p' => Some(1),
                    'n' | 'l' => Some(2),
                    _ => None,
                }
            } else {
                match key {
                    'b' => Some(1),
                    's' => Some(2),
                    'p' => Some(3),
                    'l' => Some(4),
                    _ => None,
                }
            }
        }
    }
}

fn begin_site_interaction(
    state: &mut GameState,
    kind: SiteInteractionKind,
    events: &mut Vec<Event>,
    origin: &str,
) -> String {
    let prompt = site_interaction_prompt(state, &kind);
    state.pending_site_interaction = Some(kind);
    events.push(Event::LegacyHandled {
        token: "interaction".to_string(),
        note: format!("{origin} opened site interaction"),
        fully_modeled: true,
    });
    prompt
}

fn push_log_line(state: &mut GameState, line: String) {
    if state.log.last().map(String::as_str) != Some(line.as_str()) {
        state.log.push(line);
    }
}

fn push_ui_log(state: &mut GameState, class: UiLogClass, line: String) {
    if class == UiLogClass::Timeline {
        push_log_line(state, line);
    }
}

pub fn push_timeline_line(state: &mut GameState, line: impl Into<String>) {
    push_ui_log(state, UiLogClass::Timeline, line.into());
}

fn classify_note_against_active_interactions(state: &GameState, note: &str) -> UiLogClass {
    if active_wizard_interaction_prompt(state).as_deref() == Some(note)
        || active_spell_interaction_prompt(state).as_deref() == Some(note)
        || active_quit_interaction_prompt(state).as_deref() == Some(note)
        || active_talk_direction_prompt(state).as_deref() == Some(note)
        || active_activation_interaction_prompt(state).as_deref() == Some(note)
        || active_targeting_interaction_prompt(state).as_deref() == Some(note)
        || active_inventory_interaction_prompt(state).as_deref() == Some(note)
        || active_item_prompt(state).as_deref() == Some(note)
        || active_site_interaction_prompt(state).as_deref() == Some(note)
    {
        return UiLogClass::Prompt;
    }
    if active_wizard_interaction_help_hint(state).as_deref() == Some(note)
        || active_spell_interaction_help_hint(state).as_deref() == Some(note)
        || active_quit_interaction_help_hint(state).as_deref() == Some(note)
        || active_talk_direction_help_hint(state).as_deref() == Some(note)
        || active_activation_interaction_help_hint(state).as_deref() == Some(note)
        || active_targeting_interaction_help_hint(state).as_deref() == Some(note)
        || active_inventory_interaction_help_hint(state).as_deref() == Some(note)
        || active_item_prompt_help_hint(state).as_deref() == Some(note)
        || active_site_interaction_help_hint(state).as_deref() == Some(note)
    {
        return UiLogClass::Hint;
    }
    UiLogClass::Timeline
}

fn is_legacy_prompt_noise_line(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    if trimmed.starts_with("Site prompt active:")
        || trimmed.starts_with("Arena prompt active:")
        || trimmed.starts_with("Temple prompt active:")
        || trimmed.starts_with("Rampart Coliseum prompt:")
        || trimmed.starts_with("Altar prompt:")
        || trimmed.starts_with("Wizard prompt active:")
        || trimmed.starts_with("Cast Spell:")
        || trimmed.starts_with("Possible Spells:")
        || trimmed.starts_with("Inventory active:")
        || trimmed.starts_with("Pack selection active:")
        || trimmed.starts_with("Inventory action [")
        || trimmed.starts_with("Activation prompt active:")
        || trimmed.starts_with("Activate -- item [i] or artifact [a]")
        || trimmed.starts_with("Item prompt active:")
        || trimmed.starts_with("Wish text:")
        || trimmed.starts_with("Acquire which kind of item:")
        || trimmed.starts_with("Choose status bit index")
        || trimmed.starts_with("Set or Reset or Forget it")
        || trimmed.starts_with("Bashing -- choose direction")
    {
        return true;
    }

    if trimmed.starts_with("Legacy command `")
        && (trimmed.contains("prompt active")
            || trimmed.contains("What do you wish for?")
            || trimmed.contains("inventory mode viewed")
            || trimmed.contains("Wish text:")
            || trimmed.contains("choose a bracketed option"))
    {
        return true;
    }

    if trimmed.starts_with("Step trigger:")
        && (trimmed.contains("[1]") || trimmed.contains("[1/") || trimmed.contains("choose "))
    {
        return true;
    }

    false
}

pub fn sanitize_legacy_prompt_noise(log: &mut Vec<String>) {
    log.retain(|line| !is_legacy_prompt_noise_line(line));
}

pub fn renderable_timeline_lines(state: &GameState, limit: usize) -> Vec<String> {
    if limit == 0 {
        return Vec::new();
    }
    let mut lines: Vec<String> =
        state.log.iter().filter(|line| !is_legacy_prompt_noise_line(line)).cloned().collect();
    if lines.len() > limit {
        let split = lines.len() - limit;
        lines.drain(0..split);
    }
    lines
}

fn resolve_pending_site_interaction(
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) -> bool {
    let Some(kind) = state.pending_site_interaction.clone() else {
        return false;
    };

    let choice = match command {
        Command::Legacy { token } => {
            let trimmed = token.trim();
            if trimmed.eq_ignore_ascii_case("q") || trimmed.eq_ignore_ascii_case("x") {
                state.pending_site_interaction = None;
                let note = "Site interaction closed.".to_string();
                push_log_line(state, note.clone());
                events.push(Event::LegacyHandled {
                    token: "interaction".to_string(),
                    note,
                    fully_modeled: true,
                });
                return true;
            }
            let Some(choice) = parse_site_interaction_choice(state, &kind, trimmed) else {
                let note = site_interaction_help_hint(state, &kind);
                events.push(Event::LegacyHandled {
                    token: "interaction".to_string(),
                    note,
                    fully_modeled: true,
                });
                return true;
            };
            Some(choice)
        }
        _ => {
            let note = site_interaction_help_hint(state, &kind);
            events.push(Event::LegacyHandled {
                token: "interaction".to_string(),
                note,
                fully_modeled: true,
            });
            return true;
        }
    };

    let choice = choice.unwrap_or(0);
    let action_note = apply_site_interaction_choice(state, kind, choice, events, true);
    push_log_line(state, action_note.clone());
    let note = format!("Selected option {choice}. {action_note}");
    events.push(Event::LegacyHandled {
        token: "interaction".to_string(),
        note,
        fully_modeled: true,
    });
    true
}

fn apply_site_interaction_choice(
    state: &mut GameState,
    kind: SiteInteractionKind,
    choice: usize,
    events: &mut Vec<Event>,
    reopen_prompt: bool,
) -> String {
    let mut keep_open = true;
    let note = match kind {
        SiteInteractionKind::Shop => match choice {
            1 => {
                if state.gold >= 12 {
                    state.gold -= 12;
                    let result = add_item_to_inventory_or_ground(state, "food ration", events);
                    events.push(Event::EconomyUpdated {
                        source: "shop".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    format!("Bought ration ({result}).")
                } else {
                    "Not enough gold for ration.".to_string()
                }
            }
            2 => {
                if state.gold >= 30 {
                    state.gold -= 30;
                    let result = add_item_to_inventory_or_ground(state, "healing potion", events);
                    events.push(Event::EconomyUpdated {
                        source: "shop".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Bought potion ({result}).")
                } else {
                    "Not enough gold for potion.".to_string()
                }
            }
            3 => {
                if state.gold >= 40 {
                    state.gold -= 40;
                    let result = add_item_to_inventory_or_ground(state, "scroll-identify", events);
                    events.push(Event::EconomyUpdated {
                        source: "shop".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Bought identify scroll ({result}).")
                } else {
                    "Not enough gold for identify scroll.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left shop.".to_string()
            }
            _ => "Invalid shop choice.".to_string(),
        },
        SiteInteractionKind::Armorer => match choice {
            1 => {
                if state.gold >= 70 {
                    state.gold -= 70;
                    let result = add_item_to_inventory_or_ground(state, "chain mail", events);
                    events.push(Event::EconomyUpdated {
                        source: "armorer".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Bought chain mail ({result}).")
                } else {
                    "Not enough gold for chain mail.".to_string()
                }
            }
            2 => {
                if state.gold >= 65 {
                    state.gold -= 65;
                    let result = add_item_to_inventory_or_ground(state, "long sword", events);
                    events.push(Event::EconomyUpdated {
                        source: "armorer".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Bought a weapon ({result}).")
                } else {
                    "Not enough gold for weapon purchase.".to_string()
                }
            }
            3 => {
                if state.gold >= 30 {
                    state.gold -= 30;
                    state.player.stats.defense += 1;
                    events.push(Event::EconomyUpdated {
                        source: "armorer".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Armorer refit improves your defenses.".to_string()
                } else {
                    "Not enough gold for armor refit.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left armorer.".to_string()
            }
            _ => "Invalid armorer choice.".to_string(),
        },
        SiteInteractionKind::Club => match choice {
            1 => {
                if state.gold >= 20 {
                    state.gold -= 20;
                    state.player.stats.hp =
                        (state.player.stats.hp + 1).min(state.player.stats.max_hp);
                    state.food = state.food.saturating_add(1);
                    events.push(Event::EconomyUpdated {
                        source: "club".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Club hospitality steadies your nerves.".to_string()
                } else {
                    "Not enough gold for club membership.".to_string()
                }
            }
            2 => {
                if state.gold >= 20 {
                    state.gold -= 20;
                    state.legal_heat = state.legal_heat.saturating_sub(1);
                    state.progression.quests.order.xp =
                        state.progression.quests.order.xp.saturating_add(12);
                    events.push(Event::EconomyUpdated {
                        source: "club".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "A quiet favor eases legal scrutiny.".to_string()
                } else {
                    "Not enough gold for legal favor.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left club.".to_string()
            }
            _ => "Invalid club choice.".to_string(),
        },
        SiteInteractionKind::Gym => match choice {
            1 => {
                if state.gold >= 30 {
                    state.gold -= 30;
                    state.player.stats.max_hp += 1;
                    state.player.stats.hp =
                        (state.player.stats.hp + 1).min(state.player.stats.max_hp);
                    state.progression.quests.merc.rank = state.progression.quests.merc.rank.max(1);
                    state.progression.quests.merc.xp =
                        state.progression.quests.merc.xp.saturating_add(20);
                    events.push(Event::EconomyUpdated {
                        source: "gym".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Gym drills improve your conditioning.".to_string()
                } else {
                    "Not enough gold for gym drills.".to_string()
                }
            }
            2 => {
                if state.gold >= 35 {
                    state.gold -= 35;
                    state.monsters_defeated = state.monsters_defeated.saturating_add(1);
                    if state.progression.quest_state == LegacyQuestState::NotStarted {
                        let _ = start_main_quest_from_dialogue(state, events);
                    }
                    state.progression.guild_rank = state.progression.guild_rank.max(1);
                    state.progression.quests.merc.rank = state
                        .progression
                        .quests
                        .merc
                        .rank
                        .max(i16::from(state.progression.guild_rank));
                    state.progression.quests.merc.xp =
                        state.progression.quests.merc.xp.saturating_add(30);
                    events.push(Event::EconomyUpdated {
                        source: "gym".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Gym sparring contract recorded.".to_string()
                } else {
                    "Not enough gold for spar contract.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left gym.".to_string()
            }
            _ => "Invalid gym choice.".to_string(),
        },
        SiteInteractionKind::Healer => match choice {
            1 => {
                if state.gold >= 18 {
                    state.gold -= 18;
                    state.player.stats.hp = state.player.stats.max_hp;
                    events.push(Event::EconomyUpdated {
                        source: "healer".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "The healer restores your wounds.".to_string()
                } else {
                    "Not enough gold for healing.".to_string()
                }
            }
            2 => {
                if state.gold >= 25 {
                    state.gold -= 25;
                    let before = state.status_effects.len();
                    state.status_effects.retain(|effect| effect.id != "poison");
                    events.push(Event::EconomyUpdated {
                        source: "healer".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    if state.status_effects.len() < before {
                        "The healer purges poison from your system.".to_string()
                    } else {
                        "No poison was detected; tonic still invigorates you.".to_string()
                    }
                } else {
                    "Not enough gold for antidote.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left healer.".to_string()
            }
            _ => "Invalid healer choice.".to_string(),
        },
        SiteInteractionKind::Casino => match choice {
            1 => {
                if state.gold >= 25 {
                    state.gold -= 25;
                    state.bank_gold += 5;
                    events.push(Event::EconomyUpdated {
                        source: "casino".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You buy chips at the casino tables.".to_string()
                } else {
                    "Not enough gold for casino chips.".to_string()
                }
            }
            2 => {
                if state.gold >= 10 {
                    state.gold -= 10;
                    let payout = if state.clock.turn % 2 == 0 { 18 } else { 0 };
                    state.gold += payout;
                    events.push(Event::EconomyUpdated {
                        source: "casino".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    if payout > 0 {
                        "The tables run hot; you cash out with winnings.".to_string()
                    } else {
                        "The house edge prevails this round.".to_string()
                    }
                } else {
                    "Not enough gold to play the tables.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left casino.".to_string()
            }
            _ => "Invalid casino choice.".to_string(),
        },
        SiteInteractionKind::Commandant => match choice {
            1 => {
                if state.gold >= 20 {
                    state.gold -= 20;
                    state.food = state.food.saturating_add(5);
                    let result =
                        add_item_to_inventory_or_ground(state, "bucket of rations", events);
                    events.push(Event::EconomyUpdated {
                        source: "commandant".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Buy a bucket! ({result})")
                } else {
                    "Not enough gold for a bucket.".to_string()
                }
            }
            2 => {
                state.legal_heat = state.legal_heat.saturating_sub(2);
                state.progression.quests.order.xp =
                    state.progression.quests.order.xp.saturating_add(10);
                "The commandant updates your patrol record.".to_string()
            }
            3 => {
                keep_open = false;
                "Left commandant office.".to_string()
            }
            _ => "Invalid commandant choice.".to_string(),
        },
        SiteInteractionKind::Diner => match choice {
            1 => {
                if state.gold >= 8 {
                    state.gold -= 8;
                    state.food = state.food.saturating_add(2);
                    state.player.stats.hp =
                        (state.player.stats.hp + 1).min(state.player.stats.max_hp);
                    events.push(Event::EconomyUpdated {
                        source: "diner".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You finish a hot meal at the diner.".to_string()
                } else {
                    "Not enough gold for a meal.".to_string()
                }
            }
            2 => {
                if state.gold >= 6 {
                    state.gold -= 6;
                    state.spellbook.mana = (state.spellbook.mana + 6).min(state.spellbook.max_mana);
                    events.push(Event::EconomyUpdated {
                        source: "diner".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Strong coffee sharpens your focus.".to_string()
                } else {
                    "Not enough gold for coffee.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left diner.".to_string()
            }
            _ => "Invalid diner choice.".to_string(),
        },
        SiteInteractionKind::Craps => match choice {
            1 => {
                if state.gold >= 15 {
                    state.gold -= 15;
                    state.legal_heat = state.legal_heat.saturating_add(1);
                    events.push(Event::EconomyUpdated {
                        source: "craps".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You buy into a back-room dice game.".to_string()
                } else {
                    "Not enough gold to buy into craps.".to_string()
                }
            }
            2 => {
                if state.legal_heat > 0 {
                    state.legal_heat = state.legal_heat.saturating_sub(1);
                    state.gold += 10;
                    events.push(Event::EconomyUpdated {
                        source: "craps".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You cash out before the watch arrives.".to_string()
                } else {
                    "No active game to cash out from.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left craps table.".to_string()
            }
            _ => "Invalid craps choice.".to_string(),
        },
        SiteInteractionKind::Tavern => match choice {
            1 => {
                if state.gold >= 6 {
                    state.gold -= 6;
                    state.food = state.food.saturating_add(1);
                    state.legal_heat = state.legal_heat.saturating_add(1);
                    events.push(Event::EconomyUpdated {
                        source: "tavern".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "The tavern ale restores your spirits.".to_string()
                } else {
                    "Not enough gold for ale.".to_string()
                }
            }
            2 => {
                if state.gold >= 10 {
                    state.gold -= 10;
                    state.food = state.food.saturating_add(3);
                    state.player.stats.hp =
                        (state.player.stats.hp + 1).min(state.player.stats.max_hp);
                    events.push(Event::EconomyUpdated {
                        source: "tavern".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You eat a heavy tavern stew.".to_string()
                } else {
                    "Not enough gold for stew.".to_string()
                }
            }
            3 => {
                if state.gold >= 8 {
                    state.gold -= 8;
                    let started = start_main_quest_from_dialogue(state, events);
                    let rumor = tavern_rumor_line(state);
                    events.push(Event::EconomyUpdated {
                        source: "tavern".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    if started {
                        format!("You overhear a rumor: {rumor} Quest updated.")
                    } else {
                        format!("You overhear a rumor: {rumor}")
                    }
                } else {
                    "Not enough gold for tavern rumors.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left tavern.".to_string()
            }
            _ => "Invalid tavern choice.".to_string(),
        },
        SiteInteractionKind::PawnShop => match choice {
            1 => {
                if state.gold >= 15 {
                    state.gold -= 15;
                    let oddity_name = choose_pawn_stock_item_name(state)
                        .unwrap_or_else(|| "food ration".to_string());
                    let mut oddity = instantiate_item_from_name(state.next_item_id, &oddity_name);
                    oddity.known = true;
                    state.next_item_id += 1;
                    let result = add_existing_item_to_inventory_or_ground(state, oddity, events);
                    events.push(Event::EconomyUpdated {
                        source: "pawn_shop".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Bought {oddity_name} ({result}).")
                } else {
                    "Not enough gold for pawned goods.".to_string()
                }
            }
            2 => {
                if state.player.inventory.is_empty() {
                    "No item available to pawn.".to_string()
                } else {
                    let item = state.player.inventory.remove(0);
                    unequip_item_id(&mut state.player.equipment, item.id);
                    remove_item_from_pack_order(state, item.id);
                    state.carry_burden =
                        state.carry_burden.saturating_sub(item_burden(&item)).max(0);
                    state.gold += 12;
                    events.push(Event::EconomyUpdated {
                        source: "pawn_shop".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Pawned {} for 12 gold.", item.name)
                }
            }
            3 => {
                keep_open = false;
                "Left pawn shop.".to_string()
            }
            _ => "Invalid pawn shop choice.".to_string(),
        },
        SiteInteractionKind::Brothel => match choice {
            1 => {
                if state.gold >= 25 {
                    state.gold -= 25;
                    state.player.stats.hp = state.player.stats.max_hp;
                    state.spellbook.mana =
                        (state.spellbook.mana + 10).min(state.spellbook.max_mana);
                    events.push(Event::EconomyUpdated {
                        source: "brothel".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You rent a room and recover fully.".to_string()
                } else {
                    "Not enough gold for a private room.".to_string()
                }
            }
            2 => {
                if state.gold >= 10 {
                    state.gold -= 10;
                    state.progression.quests.thieves.quest_flags |= 0x0002;
                    state.legal_heat = state.legal_heat.saturating_add(1);
                    events.push(Event::EconomyUpdated {
                        source: "brothel".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Costly gossip opens a few shadowy leads.".to_string()
                } else {
                    "Not enough gold for gossip.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left brothel.".to_string()
            }
            _ => "Invalid brothel choice.".to_string(),
        },
        SiteInteractionKind::Condo => match choice {
            1 => {
                if state.gold >= 40 {
                    state.gold -= 40;
                    state.player.stats.hp = state.player.stats.max_hp;
                    state.status_effects.clear();
                    events.push(Event::EconomyUpdated {
                        source: "condo".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "You rest at your condo and recover.".to_string()
                } else {
                    "Not enough gold to rent the condo.".to_string()
                }
            }
            2 => {
                if state.gold >= 15 {
                    state.gold -= 15;
                    state.bank_gold += 15;
                    events.push(Event::EconomyUpdated {
                        source: "condo".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Condo steward secures valuables in your lockbox.".to_string()
                } else {
                    "Not enough gold to secure a stash.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left condo.".to_string()
            }
            _ => "Invalid condo choice.".to_string(),
        },
        SiteInteractionKind::Bank => match choice {
            1 => {
                if state.gold > 0 {
                    let deposit = state.gold.min(50).max(0);
                    state.gold -= deposit;
                    state.bank_gold += deposit;
                    state.progression.quests.bank.rank = state.progression.quests.bank.rank.max(1);
                    state.progression.quests.bank.xp =
                        state.progression.quests.bank.xp.saturating_add(i64::from(deposit));
                    events.push(Event::EconomyUpdated {
                        source: "bank".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    format!("Deposited {deposit} gold.")
                } else {
                    "No gold available to deposit.".to_string()
                }
            }
            2 => {
                if state.bank_gold > 0 {
                    let withdrawal = state.bank_gold.min(50).max(0);
                    state.bank_gold -= withdrawal;
                    state.gold += withdrawal;
                    state.progression.quests.bank.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "bank".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Withdrew {withdrawal} gold.")
                } else {
                    "No bank balance available.".to_string()
                }
            }
            3 => {
                if state.gold >= 25 {
                    state.gold -= 25;
                    state.legal_heat = state.legal_heat.saturating_sub(1);
                    state.progression.law_chaos_score =
                        state.progression.law_chaos_score.max(0) + 1;
                    state.progression.quests.bank.quest_flags |= 0x0002;
                    events.push(Event::EconomyUpdated {
                        source: "bank".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Posted surety with the city bank.".to_string()
                } else {
                    "Not enough gold to post surety.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left bank.".to_string()
            }
            _ => "Invalid bank choice.".to_string(),
        },
        SiteInteractionKind::MercGuild => match choice {
            1 => {
                if state.gold >= 40 {
                    state.gold -= 40;
                    state.player.stats.attack_max += 1;
                    state.progression.guild_rank = state.progression.guild_rank.max(1);
                    state.progression.quests.merc.rank = state
                        .progression
                        .quests
                        .merc
                        .rank
                        .max(i16::from(state.progression.guild_rank));
                    state.progression.quests.merc.xp =
                        state.progression.quests.merc.xp.saturating_add(40);
                    events.push(Event::EconomyUpdated {
                        source: "merc_guild".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Merc training completed.".to_string()
                } else {
                    "Not enough gold for training.".to_string()
                }
            }
            2 => {
                if state.gold >= 40 {
                    state.gold -= 40;
                    let before_state = state.progression.quest_state;
                    state.progression.guild_rank = state.progression.guild_rank.max(1);
                    state.progression.quests.merc.rank = state
                        .progression
                        .quests
                        .merc
                        .rank
                        .max(i16::from(state.progression.guild_rank));
                    state.progression.quests.merc.xp =
                        state.progression.quests.merc.xp.saturating_add(40);
                    if state.progression.quest_state == LegacyQuestState::NotStarted {
                        state.progression.quest_state = LegacyQuestState::Active;
                        state.progression.quest_steps_completed = 1;
                        state.progression.main_quest.stage = state.progression.quest_state;
                    } else {
                        state.progression.quest_steps_completed =
                            state.progression.quest_steps_completed.max(1);
                    }
                    let objective = merc_contract_objective(state);
                    state.progression.main_quest.objective = objective.clone();
                    state.progression.quests.merc.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "merc_guild".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    if before_state != state.progression.quest_state {
                        events.push(Event::QuestAdvanced {
                            state: state.progression.quest_state,
                            steps_completed: state.progression.quest_steps_completed,
                        });
                    }
                    format!("Accepted legion contract. {objective}")
                } else {
                    "Not enough gold for a guild contract.".to_string()
                }
            }
            3 => {
                if state.gold < 60 {
                    "Not enough gold for promotion review.".to_string()
                } else {
                    let required_kills = u64::from(state.progression.guild_rank.max(1)) * 3;
                    if state.monsters_defeated < required_kills {
                        format!(
                            "Promotion denied: defeat {} more foes.",
                            required_kills.saturating_sub(state.monsters_defeated)
                        )
                    } else if state.progression.guild_rank >= 4 {
                        "Guild rank is already at maximum.".to_string()
                    } else {
                        state.gold -= 60;
                        state.progression.guild_rank += 1;
                        state.progression.quests.merc.rank = state
                            .progression
                            .quests
                            .merc
                            .rank
                            .max(i16::from(state.progression.guild_rank));
                        state.progression.quests.merc.xp =
                            state.progression.quests.merc.xp.saturating_add(120);
                        state.player.stats.attack_max += 1;
                        events.push(Event::EconomyUpdated {
                            source: "merc_guild".to_string(),
                            gold: state.gold,
                            bank_gold: state.bank_gold,
                        });
                        events.push(Event::ProgressionUpdated {
                            guild_rank: state.progression.guild_rank,
                            priest_rank: state.progression.priest_rank,
                            alignment: state.progression.alignment,
                        });
                        if state.progression.quest_state == LegacyQuestState::ReturnToPatron
                            && state.progression.guild_rank >= 2
                            && state.progression.priest_rank >= 1
                        {
                            state.progression.quest_state = LegacyQuestState::Completed;
                            state.progression.quest_steps_completed = 4;
                            state.progression.main_quest.stage = state.progression.quest_state;
                            state.progression.main_quest.completion_flags |= 0x0001;
                            events.push(Event::QuestAdvanced {
                                state: state.progression.quest_state,
                                steps_completed: state.progression.quest_steps_completed,
                            });
                        }
                        "Guild promotion granted.".to_string()
                    }
                }
            }
            4 => {
                keep_open = false;
                "Left merc guild.".to_string()
            }
            _ => "Invalid merc guild choice.".to_string(),
        },
        SiteInteractionKind::ThievesGuild => match choice {
            1 => {
                if state.progression.quests.thieves.rank > 0 {
                    "You're already in the thieves guild.".to_string()
                } else if state.progression.alignment == Alignment::Lawful {
                    "The thieves guild rejects openly lawful petitioners.".to_string()
                } else if state.progression.quests.order.rank > 0 {
                    "The thieves guild refuses active order enforcers.".to_string()
                } else if state.legal_heat > 12 {
                    "The guild cools your petition until the city watch pressure fades.".to_string()
                } else if state.gold < 30 {
                    "Not enough gold to pay guild dues.".to_string()
                } else {
                    state.gold -= 30;
                    state.progression.quests.thieves.rank = 1;
                    state.progression.quests.thieves.dues_paid =
                        state.progression.quests.thieves.dues_paid.saturating_add(30);
                    state.progression.quests.thieves.quest_flags |= 0x0001;
                    state.progression.main_quest.chaos_path = true;
                    state.progression.law_chaos_score =
                        state.progression.law_chaos_score.min(0) - 1;
                    state.progression.quests.thieves.xp =
                        state.progression.quests.thieves.xp.saturating_add(20);
                    events.push(Event::EconomyUpdated {
                        source: "thieves_guild".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "The thieves guild accepts your oath.".to_string()
                }
            }
            2 => {
                if state.progression.quests.thieves.rank <= 0 {
                    "Only guild members can take a heist contract.".to_string()
                } else if state.gold < 25 {
                    "Not enough gold to seed a heist.".to_string()
                } else if state.legal_heat > 14 {
                    "Too much city heat; lie low before attempting another heist.".to_string()
                } else {
                    state.gold -= 25;
                    let base_payout =
                        45 + i32::from(state.progression.quests.thieves.rank.max(1) as i8) * 20;
                    let stealth_bonus = match state.progression.alignment {
                        Alignment::Chaotic => 15,
                        Alignment::Neutral => 5,
                        Alignment::Lawful => -10,
                    };
                    let heat_penalty = if state.legal_heat > 8 { 10 } else { 0 };
                    let payout = (base_payout + stealth_bonus - heat_penalty).max(20);
                    state.gold += payout;
                    state.legal_heat = state.legal_heat.saturating_add(1);
                    state.progression.quests.thieves.xp =
                        state.progression.quests.thieves.xp.saturating_add(i64::from(payout));
                    state.progression.quests.thieves.quest_flags |= 0x0002;
                    if payout >= 70 {
                        state.progression.quests.thieves.promotion_flags |= 1 << 1;
                    }
                    if state.progression.quest_state == LegacyQuestState::Active {
                        state.progression.quest_steps_completed =
                            state.progression.quest_steps_completed.max(2);
                    }
                    events.push(Event::EconomyUpdated {
                        source: "thieves_guild".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Heist completed. Fence payout: {payout} gold.")
                }
            }
            3 => {
                if state.progression.quests.thieves.rank <= 0 {
                    "Promotion denied: join the guild first.".to_string()
                } else if state.gold < 55 {
                    "Not enough gold for promotion review.".to_string()
                } else if state.progression.quests.thieves.quest_flags & 0x0002 == 0 {
                    "Promotion denied: complete at least one heist contract first.".to_string()
                } else {
                    let required =
                        u64::from(state.progression.quests.thieves.rank.max(1) as u16) * 4;
                    if state.monsters_defeated < required {
                        format!(
                            "Promotion denied: neutralize {} more witnesses.",
                            required.saturating_sub(state.monsters_defeated)
                        )
                    } else if state.legal_heat > 18 {
                        "Promotion denied: too much legal pressure on your cell.".to_string()
                    } else if state.progression.quests.thieves.rank >= 5 {
                        "Thieves guild rank is already at maximum.".to_string()
                    } else {
                        state.gold -= 55;
                        state.progression.quests.thieves.rank += 1;
                        state.progression.quests.thieves.promotion_flags |=
                            1u64 << state.progression.quests.thieves.rank.min(63);
                        state.progression.quests.thieves.xp =
                            state.progression.quests.thieves.xp.saturating_add(150);
                        state.progression.quests.thieves.quest_flags |= 0x0004;
                        state.progression.main_quest.chaos_path = true;
                        state.progression.law_chaos_score -= 1;
                        if state.progression.quests.thieves.rank >= 3 {
                            state.progression.main_quest.completion_flags |= 0x0010;
                        }
                        events.push(Event::EconomyUpdated {
                            source: "thieves_guild".to_string(),
                            gold: state.gold,
                            bank_gold: state.bank_gold,
                        });
                        format!(
                            "Thieves guild promotion granted (rank {}).",
                            state.progression.quests.thieves.rank
                        )
                    }
                }
            }
            4 => {
                keep_open = false;
                "Left thieves guild.".to_string()
            }
            _ => "Invalid thieves guild choice.".to_string(),
        },
        SiteInteractionKind::Temple => match choice {
            1 => {
                if state.gold >= 15 {
                    state.gold -= 15;
                    state.progression.deity_favor += 4;
                    state.progression.priest_rank = state.progression.priest_rank.max(1);
                    state.progression.quests.temple.rank = state
                        .progression
                        .quests
                        .temple
                        .rank
                        .max(i16::from(state.progression.priest_rank));
                    state.progression.quests.temple.xp =
                        state.progression.quests.temple.xp.saturating_add(20);
                    state.progression.main_quest.law_path = true;
                    events.push(Event::EconomyUpdated {
                        source: "temple".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Temple tithe accepted.".to_string()
                } else {
                    "Not enough gold for tithe.".to_string()
                }
            }
            2 => {
                let (talk_note, _) = apply_talk_command(state, events);
                talk_note
            }
            3 => {
                if state.gold >= 35 {
                    if state.progression.deity_favor < 3 {
                        "Blessing denied: insufficient favor.".to_string()
                    } else {
                        state.gold -= 35;
                        state.progression.deity_favor =
                            state.progression.deity_favor.saturating_sub(3);
                        state.player.stats.hp =
                            (state.player.stats.hp + 6).min(state.player.stats.max_hp);
                        state.spellbook.mana =
                            (state.spellbook.mana + 12).min(state.spellbook.max_mana);
                        state.progression.priest_rank = state.progression.priest_rank.max(1);
                        state.progression.quests.temple.rank = state
                            .progression
                            .quests
                            .temple
                            .rank
                            .max(i16::from(state.progression.priest_rank));
                        state.progression.quests.temple.xp =
                            state.progression.quests.temple.xp.saturating_add(45);
                        events.push(Event::EconomyUpdated {
                            source: "temple".to_string(),
                            gold: state.gold,
                            bank_gold: state.bank_gold,
                        });
                        events.push(Event::ProgressionUpdated {
                            guild_rank: state.progression.guild_rank,
                            priest_rank: state.progression.priest_rank,
                            alignment: state.progression.alignment,
                        });
                        "Temple blessing restored health and mana.".to_string()
                    }
                } else {
                    "Not enough gold for blessing.".to_string()
                }
            }
            4 => {
                let had_poison = state.status_effects.iter().any(|effect| effect.id == "poison");
                state.status_effects.retain(|effect| effect.id != "poison");
                state.legal_heat = state.legal_heat.saturating_sub(1);
                state.player.stats.hp = (state.player.stats.hp + 2).min(state.player.stats.max_hp);
                state.progression.quests.temple.quest_flags |= 0x0002;
                if had_poison {
                    "Temple sanctuary cleansed poison and calmed legal trouble.".to_string()
                } else {
                    "Temple sanctuary offered quiet refuge.".to_string()
                }
            }
            5 => {
                keep_open = false;
                "Left temple.".to_string()
            }
            _ => "Invalid temple choice.".to_string(),
        },
        SiteInteractionKind::College => match choice {
            1 => {
                if state.progression.alignment == Alignment::Chaotic
                    && state.progression.quests.college.rank <= 0
                {
                    "The collegium refuses chaotic applicants without sponsorship.".to_string()
                } else if state.gold >= 25 {
                    state.gold -= 25;
                    state.spellbook.max_mana = (state.spellbook.max_mana + 5).min(300);
                    state.spellbook.mana = state.spellbook.max_mana;
                    state.progression.quests.college.rank =
                        state.progression.quests.college.rank.max(1);
                    state.progression.quests.college.xp =
                        state.progression.quests.college.xp.saturating_add(25);
                    state.progression.quests.college.dues_paid =
                        state.progression.quests.college.dues_paid.saturating_add(25);
                    state.progression.quests.college.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "college".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Mana training completed.".to_string()
                } else {
                    "Not enough gold for training.".to_string()
                }
            }
            2 => {
                if state.progression.quests.college.rank <= 0 {
                    "Only enrolled collegium members can request advanced instruction.".to_string()
                } else if state.gold >= 40 {
                    state.gold -= 40;
                    let learned =
                        teach_first_unknown_from_pool(state, &[12, 3, 2, 11, 16, 30, 36, 21, 40]);
                    state.spellbook.mana =
                        (state.spellbook.mana + 15).min(state.spellbook.max_mana);
                    state.progression.quests.college.rank =
                        state.progression.quests.college.rank.max(2);
                    state.progression.quests.college.xp =
                        state.progression.quests.college.xp.saturating_add(40);
                    state.progression.quests.college.quest_flags |= 0x0002;
                    if state.progression.quests.college.rank < 3
                        && state.progression.quests.college.xp >= 150
                    {
                        state.progression.quests.college.rank += 1;
                        state.progression.quests.college.promotion_flags |=
                            1 << state.progression.quests.college.rank.min(63);
                    }
                    events.push(Event::EconomyUpdated {
                        source: "college".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    if let Some(spell_id) = learned {
                        format!("Learned advanced spellcraft: {}.", spell_name_by_id(spell_id))
                    } else {
                        "Advanced instruction refreshed known spell forms.".to_string()
                    }
                } else {
                    "Not enough gold for advanced instruction.".to_string()
                }
            }
            3 => {
                if state.gold < 30 {
                    "Not enough gold for identification.".to_string()
                } else if state.player.inventory.is_empty() {
                    "No item available for identification.".to_string()
                } else {
                    state.gold -= 30;
                    events.push(Event::EconomyUpdated {
                        source: "college".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    state.progression.quests.college.quest_flags |= 0x0004;
                    if let Some(item) = state.player.inventory.first_mut() {
                        if !item.name.starts_with("identified ") {
                            item.name = format!("identified {}", item.name);
                            state.progression.quests.college.xp =
                                state.progression.quests.college.xp.saturating_add(25);
                            state.progression.quests.college.promotion_flags |= 1 << 1;
                        }
                        "College scholars identified your foremost item.".to_string()
                    } else {
                        "No item available for identification.".to_string()
                    }
                }
            }
            4 => {
                keep_open = false;
                "Left collegium.".to_string()
            }
            _ => "Invalid college choice.".to_string(),
        },
        SiteInteractionKind::Sorcerors => match choice {
            1 => {
                if state.progression.alignment == Alignment::Lawful
                    && state.progression.quests.sorcerors.rank <= 0
                {
                    "The circle denies initiation to lawful petitioners without black sponsorship."
                        .to_string()
                } else if state.gold >= 30 {
                    state.gold -= 30;
                    state.spellbook.mana =
                        (state.spellbook.mana + 20).min(state.spellbook.max_mana);
                    state.progression.quests.sorcerors.rank =
                        state.progression.quests.sorcerors.rank.max(1);
                    state.progression.quests.sorcerors.xp =
                        state.progression.quests.sorcerors.xp.saturating_add(30);
                    state.progression.quests.sorcerors.dues_paid =
                        state.progression.quests.sorcerors.dues_paid.saturating_add(30);
                    state.progression.quests.sorcerors.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "sorcerors".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Arcane recharge completed.".to_string()
                } else {
                    "Not enough gold for recharge.".to_string()
                }
            }
            2 => {
                if state.progression.quests.sorcerors.rank <= 0 {
                    "Only initiated sorcerors can study deep lore.".to_string()
                } else if state.gold >= 50 {
                    state.gold -= 50;
                    let learned =
                        teach_first_unknown_from_pool(state, &[31, 32, 33, 34, 37, 38, 39, 15, 40]);
                    state.progression.quests.sorcerors.rank =
                        state.progression.quests.sorcerors.rank.max(2);
                    state.progression.quests.sorcerors.xp =
                        state.progression.quests.sorcerors.xp.saturating_add(50);
                    state.progression.quests.sorcerors.quest_flags |= 0x0002;
                    if state.progression.quests.sorcerors.xp >= 180 {
                        state.progression.quests.sorcerors.promotion_flags |= 1 << 2;
                    }
                    events.push(Event::EconomyUpdated {
                        source: "sorcerors".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    if let Some(spell_id) = learned {
                        format!("Deep lore unlocked: {}.", spell_name_by_id(spell_id))
                    } else {
                        "Deep lore study improved your existing arcana.".to_string()
                    }
                } else {
                    "Not enough gold for deep lore.".to_string()
                }
            }
            3 => {
                if state.progression.quests.sorcerors.rank <= 0 {
                    "Initiation is required before transmutation rites.".to_string()
                } else if state.gold >= 45 {
                    state.gold -= 45;
                    let result = add_item_to_inventory_or_ground(state, "charged stick", events);
                    state.spellbook.mana = (state.spellbook.mana + 5).min(state.spellbook.max_mana);
                    state.progression.quests.sorcerors.quest_flags |= 0x0004;
                    state.progression.quests.sorcerors.xp =
                        state.progression.quests.sorcerors.xp.saturating_add(35);
                    if state.progression.alignment == Alignment::Lawful {
                        state.progression.law_chaos_score -= 1;
                    }
                    events.push(Event::EconomyUpdated {
                        source: "sorcerors".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Sorcerors transmuted a focus ({result}).")
                } else {
                    "Not enough gold for transmutation.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left sorcerors.".to_string()
            }
            _ => "Invalid sorcerors choice.".to_string(),
        },
        SiteInteractionKind::Castle => match choice {
            1 => {
                if state.legal_heat > 0 {
                    let fine = (state.legal_heat * 3).max(5);
                    let paid = fine.min(state.gold.max(0));
                    state.gold -= paid;
                    state.legal_heat = state.legal_heat.saturating_sub(2);
                    state.progression.quests.castle.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "castle".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    format!("Paid {paid} gold in fines.")
                } else {
                    "No legal fines pending.".to_string()
                }
            }
            2 => {
                let (talk_note, _) = apply_talk_command(state, events);
                state.progression.quests.castle.rank = state.progression.quests.castle.rank.max(1);
                state.progression.main_quest.palace_access =
                    state.progression.main_quest.palace_access || state.progression.guild_rank >= 1;
                talk_note
            }
            3 => match state.progression.quest_state {
                LegacyQuestState::ArtifactRecovered => {
                    state.progression.quest_state = LegacyQuestState::ReturnToPatron;
                    state.progression.main_quest.stage = LegacyQuestState::ReturnToPatron;
                    state.progression.quest_steps_completed = 3;
                    state.progression.guild_rank = state.progression.guild_rank.max(2);
                    state.progression.quests.castle.rank =
                        state.progression.quests.castle.rank.max(2);
                    state.progression.main_quest.completion_flags |= 0x0002;
                    state.gold += 120;
                    events.push(Event::QuestAdvanced {
                        state: state.progression.quest_state,
                        steps_completed: state.progression.quest_steps_completed,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    events.push(Event::EconomyUpdated {
                        source: "castle".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Royal petition accepted; return to your patron for final rites.".to_string()
                }
                LegacyQuestState::ReturnToPatron => {
                    if state.progression.guild_rank >= 2 && state.progression.priest_rank >= 1 {
                        state.progression.quest_state = LegacyQuestState::Completed;
                        state.progression.main_quest.stage = LegacyQuestState::Completed;
                        state.progression.quest_steps_completed = 4;
                        state.progression.total_winner_unlocked = true;
                        state.progression.main_quest.completion_flags |= 0x0004;
                        state.progression.score += 500;
                        state.gold += 200;
                        state.progression.quests.castle.rank =
                            state.progression.quests.castle.rank.max(4);
                        events.push(Event::QuestAdvanced {
                            state: state.progression.quest_state,
                            steps_completed: state.progression.quest_steps_completed,
                        });
                        events.push(Event::EconomyUpdated {
                            source: "castle".to_string(),
                            gold: state.gold,
                            bank_gold: state.bank_gold,
                        });
                        "Royal charter granted. Quest line completed.".to_string()
                    } else {
                        "Petition deferred: gain rank with guild and temple first.".to_string()
                    }
                }
                LegacyQuestState::Completed => {
                    state.gold += 60;
                    events.push(Event::EconomyUpdated {
                        source: "castle".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "The crown recognizes prior service with a stipend.".to_string()
                }
                _ => "No active royal petition is available.".to_string(),
            },
            4 => {
                keep_open = false;
                "Left castle.".to_string()
            }
            _ => "Invalid castle choice.".to_string(),
        },
        SiteInteractionKind::Palace => match choice {
            1 => {
                if !state.progression.main_quest.palace_access {
                    "Palace guards deny your audience request.".to_string()
                } else {
                    state.progression.quests.palace.rank =
                        state.progression.quests.palace.rank.max(1);
                    state.progression.quests.palace.xp =
                        state.progression.quests.palace.xp.saturating_add(20);
                    state.progression.main_quest.law_path = true;
                    "You are announced before the palace chamberlain.".to_string()
                }
            }
            2 => {
                if !state.progression.main_quest.palace_access {
                    "Petition denied: you lack standing at the palace.".to_string()
                } else if state.progression.main_quest.stage == LegacyQuestState::ArtifactRecovered
                {
                    state.progression.main_quest.stage = LegacyQuestState::ReturnToPatron;
                    state.progression.quest_state = LegacyQuestState::ReturnToPatron;
                    state.progression.quest_steps_completed =
                        state.progression.quest_steps_completed.max(3);
                    state.progression.quests.palace.quest_flags |= 0x0001;
                    state.gold += 160;
                    events.push(Event::QuestAdvanced {
                        state: state.progression.quest_state,
                        steps_completed: state.progression.quest_steps_completed,
                    });
                    events.push(Event::EconomyUpdated {
                        source: "palace".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Palace petition accepted. Return to your patron for final investiture."
                        .to_string()
                } else if state.progression.main_quest.stage == LegacyQuestState::ReturnToPatron
                    && state.progression.guild_rank >= 2
                    && state.progression.priest_rank >= 1
                {
                    state.progression.main_quest.stage = LegacyQuestState::Completed;
                    state.progression.quest_state = LegacyQuestState::Completed;
                    state.progression.total_winner_unlocked = true;
                    state.progression.main_quest.completion_flags |= 0x0008;
                    state.progression.quests.palace.rank =
                        state.progression.quests.palace.rank.max(4);
                    state.progression.score += 800;
                    state.gold += 240;
                    events.push(Event::QuestAdvanced {
                        state: state.progression.quest_state,
                        steps_completed: state.progression.quest_steps_completed.max(4),
                    });
                    events.push(Event::EconomyUpdated {
                        source: "palace".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    "Royal investiture complete. Your palace quest line is fulfilled.".to_string()
                } else {
                    "The palace petition is deferred pending greater deeds.".to_string()
                }
            }
            3 => {
                keep_open = false;
                "Left palace.".to_string()
            }
            _ => "Invalid palace choice.".to_string(),
        },
        SiteInteractionKind::Order => match choice {
            1 => {
                if state.progression.quests.thieves.rank > 0 {
                    "Order masters reject active thieves from lawful vows.".to_string()
                } else {
                    state.progression.alignment = Alignment::Lawful;
                    state.progression.law_chaos_score = state.progression.law_chaos_score.max(5);
                    state.progression.quests.order.rank =
                        state.progression.quests.order.rank.max(1);
                    state.progression.quests.order.xp =
                        state.progression.quests.order.xp.saturating_add(20);
                    state.progression.main_quest.law_path = true;
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Order service realigned you toward law.".to_string()
                }
            }
            2 => {
                if state.gold >= 25 {
                    state.gold -= 25;
                    state.legal_heat = state.legal_heat.saturating_sub(2);
                    if state.progression.alignment == Alignment::Chaotic {
                        state.progression.alignment = Alignment::Neutral;
                    }
                    state.progression.law_chaos_score =
                        state.progression.law_chaos_score.max(-1) + 2;
                    state.progression.quests.order.quest_flags |= 0x0001;
                    events.push(Event::EconomyUpdated {
                        source: "order".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Order absolution reduced your legal burden.".to_string()
                } else {
                    "Not enough gold for absolution rites.".to_string()
                }
            }
            3 => {
                let (talk_note, _) = apply_talk_command(state, events);
                talk_note
            }
            4 => {
                keep_open = false;
                "Left order hall.".to_string()
            }
            _ => "Invalid order choice.".to_string(),
        },
        SiteInteractionKind::Charity => match choice {
            1 => {
                state.player.stats.hp = (state.player.stats.hp + 4).min(state.player.stats.max_hp);
                state.food += 1;
                state.progression.quests.charity.rank =
                    state.progression.quests.charity.rank.max(1);
                events.push(Event::ProgressionUpdated {
                    guild_rank: state.progression.guild_rank,
                    priest_rank: state.progression.priest_rank,
                    alignment: state.progression.alignment,
                });
                "Charity meal restored health and food.".to_string()
            }
            2 => {
                let before = state.status_effects.len();
                state.status_effects.retain(|effect| effect.id != "poison" && effect.id != "fear");
                state.progression.quests.charity.quest_flags |= 0x0001;
                if before == state.status_effects.len() {
                    "Charity cleansing found no ailments.".to_string()
                } else {
                    "Charity cleansing removed lingering curses.".to_string()
                }
            }
            3 => {
                state.progression.law_chaos_score += 1;
                state.legal_heat = state.legal_heat.saturating_sub(1);
                if state.progression.alignment == Alignment::Chaotic
                    && state.progression.law_chaos_score >= 0
                {
                    state.progression.alignment = Alignment::Neutral;
                }
                state.progression.quests.charity.xp =
                    state.progression.quests.charity.xp.saturating_add(10);
                events.push(Event::ProgressionUpdated {
                    guild_rank: state.progression.guild_rank,
                    priest_rank: state.progression.priest_rank,
                    alignment: state.progression.alignment,
                });
                "You volunteer with charity and restore civic trust.".to_string()
            }
            4 => {
                keep_open = false;
                "Left charity.".to_string()
            }
            _ => "Invalid charity choice.".to_string(),
        },
        SiteInteractionKind::Monastery => match choice {
            1 => {
                if state.gold < 20 {
                    "Not enough gold for monastery meditation.".to_string()
                } else if state.progression.alignment == Alignment::Chaotic
                    && state.progression.quests.monastery.rank <= 1
                {
                    "The masters deny meditation until you complete an atonement vow.".to_string()
                } else {
                    state.gold -= 20;
                    state.progression.quests.monastery.rank =
                        state.progression.quests.monastery.rank.max(1);
                    state.progression.quests.monastery.xp =
                        state.progression.quests.monastery.xp.saturating_add(25);
                    state.progression.quests.monastery.dues_paid =
                        state.progression.quests.monastery.dues_paid.saturating_add(20);
                    state.progression.quests.monastery.quest_flags |= 0x0001;
                    if state.progression.quests.monastery.quest_flags & 0x0200 == 0 {
                        state.progression.quests.monastery.quest_flags |= 0x0200;
                        state.progression.quests.monastery.xp =
                            state.progression.quests.monastery.xp.saturating_add(5);
                        state.progression.quests.monastery.dues_paid =
                            state.progression.quests.monastery.dues_paid.saturating_add(1);
                    } else {
                        state.progression.quests.monastery.quest_flags |= 0x0400;
                    }
                    state.spellbook.max_mana = (state.spellbook.max_mana + 2).min(320);
                    state.spellbook.mana = state.spellbook.max_mana;
                    state.player.stats.hp =
                        (state.player.stats.hp + 3).min(state.player.stats.max_hp);
                    if state.legal_heat > 0 {
                        state.legal_heat = state.legal_heat.saturating_sub(1);
                        state.progression.quests.monastery.quest_flags |= 0x0002;
                    }
                    if state.progression.deity_favor < 5 {
                        state.progression.deity_favor += 1;
                    } else {
                        state.progression.quests.monastery.quest_flags |= 0x0004;
                    }
                    if state.progression.quests.monastery.xp >= 120
                        && state.progression.quests.monastery.rank < 2
                    {
                        state.progression.quests.monastery.rank = 2;
                        state.progression.quests.monastery.promotion_flags |= 1 << 2;
                    }
                    events.push(Event::EconomyUpdated {
                        source: "monastery".to_string(),
                        gold: state.gold,
                        bank_gold: state.bank_gold,
                    });
                    events.push(Event::ProgressionUpdated {
                        guild_rank: state.progression.guild_rank,
                        priest_rank: state.progression.priest_rank,
                        alignment: state.progression.alignment,
                    });
                    "Monastery meditation deepens your focus and steadies your spirit.".to_string()
                }
            }
            2 => {
                if state.player.inventory.is_empty() {
                    "You have no worldly goods to donate.".to_string()
                } else if let Some(item_id) = first_pack_item_id(state) {
                    if let Some(item) = remove_item_by_id(state, item_id) {
                        state.progression.quests.monastery.rank =
                            state.progression.quests.monastery.rank.max(1);
                        state.progression.quests.monastery.dues_paid =
                            state.progression.quests.monastery.dues_paid.saturating_add(1);
                        state.progression.quests.monastery.xp =
                            state.progression.quests.monastery.xp.saturating_add(35);
                        state.progression.deity_favor =
                            state.progression.deity_favor.saturating_add(1);
                        state.progression.quests.monastery.quest_flags |= 0x0008;
                        if item.blessing < 0 {
                            state.progression.deity_favor =
                                state.progression.deity_favor.saturating_add(1);
                            state.progression.law_chaos_score += 1;
                            state.progression.quests.monastery.quest_flags |= 0x0010;
                        }
                        if item.blessing > 0 {
                            state.progression.quests.monastery.xp =
                                state.progression.quests.monastery.xp.saturating_add(10);
                            state.progression.quests.monastery.quest_flags |= 0x0020;
                        }
                        if state.progression.quests.monastery.dues_paid >= 5
                            && state.progression.quests.monastery.rank < 2
                        {
                            state.progression.quests.monastery.rank = 2;
                            state.progression.quests.monastery.promotion_flags |= 1 << 2;
                        }
                        if state.progression.quests.monastery.dues_paid >= 12
                            && state.progression.quests.monastery.rank < 3
                        {
                            state.progression.quests.monastery.rank = 3;
                            state.progression.quests.monastery.promotion_flags |= 1 << 3;
                        }
                        format!("You donate {} to the monastery hospice.", item.name)
                    } else {
                        "Monastery donation failed: item could not be offered.".to_string()
                    }
                } else {
                    "Monastery donation failed: no pack item selected.".to_string()
                }
            }
            3 => {
                let before = state.progression.alignment;
                state.progression.alignment = Alignment::Lawful;
                state.progression.law_chaos_score = state.progression.law_chaos_score.max(3);
                state.progression.quests.monastery.rank =
                    state.progression.quests.monastery.rank.max(1);
                state.progression.quests.monastery.quest_flags |= 0x0040;
                state.progression.quests.monastery.xp =
                    state.progression.quests.monastery.xp.saturating_add(20);
                if state.progression.alignment == Alignment::Chaotic && state.legal_heat > 6 {
                    state.legal_heat = state.legal_heat.saturating_sub(2);
                    state.progression.law_chaos_score += 2;
                }
                if state.progression.quests.monastery.rank >= 2
                    && state.progression.deity_favor >= 5
                    && state.gold >= 10
                {
                    state.gold -= 10;
                    state.spellbook.max_mana = (state.spellbook.max_mana + 1).min(340);
                    state.spellbook.mana = state.spellbook.max_mana;
                    state.progression.quests.monastery.quest_flags |= 0x0080;
                }
                events.push(Event::ProgressionUpdated {
                    guild_rank: state.progression.guild_rank,
                    priest_rank: state.progression.priest_rank,
                    alignment: state.progression.alignment,
                });
                events.push(Event::EconomyUpdated {
                    source: "monastery".to_string(),
                    gold: state.gold,
                    bank_gold: state.bank_gold,
                });
                if before == Alignment::Lawful {
                    "You renew your discipline before the masters.".to_string()
                } else {
                    "You vow discipline and turn your path toward law.".to_string()
                }
            }
            4 => {
                keep_open = false;
                "Left monastery.".to_string()
            }
            _ => "Invalid monastery choice.".to_string(),
        },
        SiteInteractionKind::Arena => {
            if state.progression.arena_rank > 0 {
                match choice {
                    1 => {
                        if state.progression.arena_match_active {
                            "You're already in the games. Defeat your current challenger first."
                                .to_string()
                        } else {
                            keep_open = false;
                            state.progression.quests.arena.quest_flags |= 0x0001;
                            start_arena_challenge(state)
                        }
                    }
                    2 => {
                        keep_open = false;
                        "You decline the arena challenge.".to_string()
                    }
                    _ => "Invalid arena choice.".to_string(),
                }
            } else {
                match choice {
                    1 => {
                        if state.progression.arena_match_active {
                            "You're already in the games. Defeat your current challenger first."
                                .to_string()
                        } else {
                            keep_open = false;
                            state.progression.quests.arena.quest_flags |= 0x0001;
                            start_arena_challenge(state)
                        }
                    }
                    2 => {
                        state.progression.arena_rank = 1;
                        state.progression.quests.arena.rank = 1;
                        state.progression.arena_opponent = state.progression.arena_opponent.max(3);
                        events.push(Event::ProgressionUpdated {
                            guild_rank: state.progression.guild_rank,
                            priest_rank: state.progression.priest_rank,
                            alignment: state.progression.alignment,
                        });
                        "Ok, yer now an Arena Trainee.".to_string()
                    }
                    3 => {
                        keep_open = false;
                        "You leave the Coliseum.".to_string()
                    }
                    _ => "Invalid arena choice.".to_string(),
                }
            }
        }
        SiteInteractionKind::Altar { deity_id } => {
            if altar_needs_initial_worship(state) {
                match choice {
                    1 => apply_altar_prayer(state, deity_id, events),
                    2 => {
                        keep_open = false;
                        "You step away from the altar.".to_string()
                    }
                    _ => "Invalid altar choice.".to_string(),
                }
            } else {
                match choice {
                    1 => apply_altar_blessing(state, deity_id, events),
                    2 => apply_altar_sacrifice(state, deity_id, events),
                    3 => apply_altar_prayer(state, deity_id, events),
                    4 => {
                        keep_open = false;
                        "You leave the altar.".to_string()
                    }
                    _ => "Invalid altar choice.".to_string(),
                }
            }
        }
    };

    if reopen_prompt && keep_open {
        state.pending_site_interaction = Some(kind.clone());
    } else {
        state.pending_site_interaction = None;
    }
    note
}

fn trigger_step_site_interaction(state: &mut GameState, events: &mut Vec<Event>) {
    if state.world_mode != WorldMode::DungeonCity {
        return;
    }

    let site_aux =
        state.tile_site_at(state.player.position).map(|site| site.aux).unwrap_or(SITE_AUX_NONE);
    let open_arena_gateway = open_arena_gateway_exit_target(state, state.player.position).is_some();
    if site_aux == SITE_AUX_NONE && !open_arena_gateway {
        return;
    }

    if state.options.interactive_sites
        && let Some(kind) = interaction_kind_for_site_aux(state, site_aux)
    {
        let _ = begin_site_interaction(state, kind, events, "step");
        return;
    }

    if let Some(note) = apply_site_service(state, site_aux, events) {
        let class = classify_note_against_active_interactions(state, &note);
        push_ui_log(state, class, note.clone());
        events.push(Event::LegacyHandled { token: "step".to_string(), note, fully_modeled: true });
        return;
    }

    if site_aux == SITE_AUX_EXIT_COUNTRYSIDE
        || site_aux == SITE_AUX_EXIT_ARENA
        || open_arena_gateway
    {
        let (note, _) = resolve_enter_local_site(state, events);
        let class = classify_note_against_active_interactions(state, &note);
        push_ui_log(state, class, note.clone());
        events.push(Event::LegacyHandled { token: "step".to_string(), note, fully_modeled: true });
    }
}

fn open_door_at(state: &mut GameState, pos: Position) -> bool {
    if !state.bounds.contains(pos) {
        return false;
    }
    let glyph = state.map_glyph_at(pos);
    if glyph != '-' && glyph != 'D' && glyph != 'J' {
        return false;
    }

    let mut flags = state.tile_site_at(pos).map(|cell| cell.flags).unwrap_or(0);
    let _ = state.set_map_glyph_at(pos, '/');
    flags &= !TILE_FLAG_BLOCK_MOVE;
    flags |= TILE_FLAG_OPENED_DOOR;
    set_site_flags_at(state, pos, flags);
    true
}

fn try_bump_interaction_on_blocked_move<R: RandomSource>(
    state: &mut GameState,
    from: Position,
    target: Position,
    rng: &mut R,
    events: &mut Vec<Event>,
    bonus_minutes: &mut u64,
) -> bool {
    if state.world_mode != WorldMode::DungeonCity || !open_door_at(state, target) {
        return false;
    }

    let note = format!("opened door at ({}, {})", target.x, target.y);
    push_timeline_line(state, note.clone());
    events.push(Event::LegacyHandled { token: "step".to_string(), note, fully_modeled: true });

    if state.tile_is_walkable(target) && !is_occupied(state, target) {
        state.player.position = target;
        state.log.push("You move.".to_string());
        events.push(Event::Moved { from, to: target });
        apply_post_move_effects(state, rng, events, bonus_minutes);
    }

    true
}

fn try_bump_attack_on_move<R: RandomSource>(
    state: &mut GameState,
    direction: Direction,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> bool {
    let target = state.player.position.offset(direction);
    if monster_index_at(state, target).is_none() {
        return false;
    }
    resolve_attack_command(state, direction, rng, events);
    true
}

fn apply_lost_navigation_direction<R: RandomSource>(
    state: &mut GameState,
    intended: Direction,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> Direction {
    if state.world_mode != WorldMode::Countryside || !state.navigation_lost {
        return intended;
    }
    let redirected = random_cardinal_direction(rng);
    if redirected != intended {
        let note = "Being lost, you strike out randomly....".to_string();
        push_timeline_line(state, note.clone());
        events.push(Event::LegacyHandled { token: "lost".to_string(), note, fully_modeled: true });
    }
    redirected
}

fn apply_post_move_effects<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
    bonus_minutes: &mut u64,
) {
    if state.world_mode == WorldMode::Countryside {
        *bonus_minutes = bonus_minutes.saturating_add(apply_countryside_travel(state, rng, events));
    } else {
        trigger_step_site_interaction(state, events);
    }
    if state.options.pickup {
        try_pickup_at_player(state, events);
    }
    if state.options.belligerent
        && let Some(direction) = adjacent_monster_direction(state)
    {
        resolve_attack_command(state, direction, rng, events);
    }
}

fn apply_door_interaction(state: &mut GameState, close: bool) -> String {
    if state.world_mode != WorldMode::DungeonCity {
        return "door interaction only works in dungeon/city maps".to_string();
    }

    let candidates = [
        Position { x: state.player.position.x, y: state.player.position.y - 1 },
        Position { x: state.player.position.x + 1, y: state.player.position.y },
        Position { x: state.player.position.x, y: state.player.position.y + 1 },
        Position { x: state.player.position.x - 1, y: state.player.position.y },
    ];

    for pos in candidates {
        if !state.bounds.contains(pos) {
            continue;
        }
        let glyph = state.map_glyph_at(pos);
        let mut flags = state.tile_site_at(pos).map(|cell| cell.flags).unwrap_or(0);

        if close {
            let is_open_door = glyph == '/' || glyph == '|' || (flags & TILE_FLAG_OPENED_DOOR) != 0;
            if !is_open_door {
                continue;
            }
            let _ = state.set_map_glyph_at(pos, '-');
            flags |= TILE_FLAG_BLOCK_MOVE;
            flags &= !TILE_FLAG_OPENED_DOOR;
            set_site_flags_at(state, pos, flags);
            return format!("closed door at ({}, {})", pos.x, pos.y);
        }

        let is_closed_door = glyph == '-' || glyph == 'D' || glyph == 'J';
        if !is_closed_door {
            continue;
        }
        let _ = state.set_map_glyph_at(pos, '/');
        flags &= !TILE_FLAG_BLOCK_MOVE;
        flags |= TILE_FLAG_OPENED_DOOR;
        set_site_flags_at(state, pos, flags);
        return format!("opened door at ({}, {})", pos.x, pos.y);
    }

    if close {
        "no adjacent open door to close".to_string()
    } else {
        "no adjacent closed door to open".to_string()
    }
}

fn set_site_flags_at(state: &mut GameState, pos: Position, flags: u16) {
    let Some(idx) = tile_index(state.bounds, pos) else {
        return;
    };
    if let Some(cell) = state.site_grid.get_mut(idx) {
        cell.flags = flags;
    }
    match state.map_binding.semantic {
        MapSemanticKind::City => {
            if let Some(cell) = state.city_site_grid.get_mut(idx) {
                cell.flags = flags;
            }
        }
        MapSemanticKind::Country => {
            if let Some(cell) = state.country_site_grid.get_mut(idx) {
                cell.flags = flags;
            }
        }
        _ => {}
    }
}

fn set_site_glyph_at(state: &mut GameState, pos: Position, glyph: char) {
    let Some(idx) = tile_index(state.bounds, pos) else {
        return;
    };
    if let Some(cell) = state.site_grid.get_mut(idx) {
        cell.glyph = glyph;
    }
    match state.map_binding.semantic {
        MapSemanticKind::City => {
            if let Some(cell) = state.city_site_grid.get_mut(idx) {
                cell.glyph = glyph;
            }
        }
        MapSemanticKind::Country => {
            if let Some(cell) = state.country_site_grid.get_mut(idx) {
                cell.glyph = glyph;
            }
        }
        _ => {}
    }
}

fn monster_is_hostile_to_player(state: &GameState, behavior: MonsterBehavior, faction: Faction) -> bool {
    match (faction, state.progression.alignment, behavior) {
        (Faction::Law, Alignment::Lawful, _) => false,
        (Faction::Chaos, Alignment::Chaotic, _) => false,
        (Faction::Neutral, _, MonsterBehavior::Social) => false,
        (Faction::Wild, _, MonsterBehavior::Social) => false,
        (Faction::Neutral, _, _) => true,
        (Faction::Wild, _, _) => true,
        _ => true,
    }
}

fn resolve_talk_direction(
    state: &mut GameState,
    target: Position,
    events: &mut Vec<Event>,
) -> (String, bool) {
    if !state.bounds.contains(target) {
        return ("Talk -- nobody there.".to_string(), true);
    }

    let Some(idx) = monster_index_at(state, target) else {
        return ("Talk -- nobody there.".to_string(), true);
    };

    let monster = state.monsters[idx].clone();
    let hostile = monster_is_hostile_to_player(state, monster.behavior, monster.faction);
    if hostile {
        return (
            format!("{} refuses to parley and keeps their distance.", monster.name),
            true,
        );
    }

    events.push(Event::DialogueAdvanced {
        speaker: monster.name.clone(),
        quest_state: state.progression.quest_state,
    });
    (format!("You talk with {}.", monster.name), true)
}

fn resolve_tunnel_direction(state: &mut GameState, target: Position) -> (String, bool) {
    if !state.bounds.contains(target) {
        return ("You can't tunnel through that!".to_string(), true);
    }

    if let Some(site) = state.tile_site_at(target)
        && (site.flags & TILE_FLAG_PORTCULLIS) != 0
    {
        return ("You can't tunnel through that!".to_string(), true);
    }

    let glyph = state.map_glyph_at(target);
    let tunnelable = matches!(glyph, '#' | '=' | '-' | 'D' | 'J' | '|');
    if !tunnelable {
        return ("You can't tunnel through that!".to_string(), true);
    }

    let mut flags = state.tile_site_at(target).map(|site| site.flags).unwrap_or(0);
    flags &= !(TILE_FLAG_BLOCK_MOVE | TILE_FLAG_OPENED_DOOR);
    set_site_flags_at(state, target, flags);
    set_site_glyph_at(state, target, '.');
    let _ = state.set_map_glyph_at(target, '.');
    ("You carve a tunnel through the stone!".to_string(), true)
}

fn speaker_for_site_aux(state: &GameState, site_aux: i32) -> &'static str {
    match site_aux {
        SITE_AUX_SERVICE_ORDER => "order",
        SITE_AUX_SERVICE_CASTLE => "castle",
        SITE_AUX_SERVICE_PALACE => "palace",
        SITE_AUX_SERVICE_TEMPLE => "temple",
        SITE_AUX_SERVICE_ARMORER => "armorer",
        SITE_AUX_SERVICE_CLUB => "club",
        SITE_AUX_SERVICE_GYM => "gym",
        SITE_AUX_SERVICE_HEALER => "healer",
        SITE_AUX_SERVICE_CASINO => "casino",
        SITE_AUX_SERVICE_COMMANDANT => "commandant",
        SITE_AUX_SERVICE_DINER => "diner",
        SITE_AUX_SERVICE_CRAPS => "craps",
        SITE_AUX_SERVICE_TAVERN => "tavern",
        SITE_AUX_SERVICE_PAWN_SHOP => "pawn",
        SITE_AUX_SERVICE_BROTHEL => "brothel",
        SITE_AUX_SERVICE_CONDO => "condo",
        SITE_AUX_SERVICE_MERC_GUILD => "merc_guild",
        SITE_AUX_SERVICE_THIEVES => "thieves_guild",
        SITE_AUX_SERVICE_COLLEGE => "college",
        SITE_AUX_SERVICE_SORCERORS => "sorcerors",
        SITE_AUX_SERVICE_BANK => "banker",
        SITE_AUX_SERVICE_CHARITY => "charity",
        SITE_AUX_SERVICE_MONASTERY => "monastery",
        SITE_AUX_SERVICE_ARENA => "arena",
        SITE_AUX_SERVICE_SHOP => "merchant",
        _ => match state.environment {
            LegacyEnvironment::Village => "villager",
            _ => "local_npc",
        },
    }
}

fn start_main_quest_from_dialogue(state: &mut GameState, events: &mut Vec<Event>) -> bool {
    if state.progression.quest_state != LegacyQuestState::NotStarted {
        return false;
    }
    state.progression.quest_state = LegacyQuestState::Active;
    state.progression.main_quest.stage = state.progression.quest_state;
    state.progression.quest_steps_completed = 1;
    state.progression.main_quest.objective =
        "Seek distinction through guild and temple service.".to_string();
    events.push(Event::QuestAdvanced {
        state: state.progression.quest_state,
        steps_completed: state.progression.quest_steps_completed,
    });
    true
}

fn tavern_rumor_line(state: &GameState) -> String {
    if !state.progression.main_quest.objective.trim().is_empty() {
        return state.progression.main_quest.objective.clone();
    }
    const LEGACY_TAVERN_RUMORS: [&str; 20] = [
        "There is an entrance to the sewers in the Garden.",
        "Statues can be dangerous.",
        "Unidentified Artifacts can be dangerous.",
        "The Mercenaries are the best equipped fighters.",
        "The Gladiators are the most skilled fighters.",
        "They say some famous people live in mansions.",
        "There are caves due south of Rampart.",
        "The Temple of Athena is to the North-East.",
        "The Temple of Set can be found in a desert.",
        "The Temple of Hecate is in the swamp.",
        "The Temple of Odin is to the South in some mountains.",
        "The Star Gem is guarded by the Circle of Sorcerors.",
        "The Lawgiver can be found at Star Peak.",
        "The Demon Emperor resides in the Volcano.",
        "The aligned temples are dangerous to unbelievers.",
        "The Circle of Sorcerors has an Astral HQ.",
        "Only a master of chaos would kill all the city guards!",
        "Each sect has its own main temple outside the city.",
        "A wish for Location might help you become Adept.",
        "They say blessings can really change an item's attitude.",
    ];
    let seed = (state.clock.turn as usize)
        .wrapping_mul(13)
        .wrapping_add((state.clock.minutes as usize) / 5)
        .wrapping_add(state.next_item_id as usize)
        .wrapping_add(state.progression.quest_steps_completed as usize);
    LEGACY_TAVERN_RUMORS[seed % LEGACY_TAVERN_RUMORS.len()].to_string()
}

fn choose_pawn_stock_item_name(state: &GameState) -> Option<String> {
    let kinds = &WISH_ITEM_KINDS_NON_ARTIFACT;
    if kinds.is_empty() {
        return None;
    }
    let seed = state
        .next_item_id
        .wrapping_add((state.clock.turn as u32).wrapping_mul(97))
        .wrapping_add((state.clock.minutes as u32).wrapping_mul(17));
    for offset in 0..kinds.len() {
        let kind = kinds[(seed as usize + offset) % kinds.len()];
        if let Some(name) = random_item_from_kind(state, kind) {
            return Some(name);
        }
    }
    None
}

fn merc_contract_objective(state: &GameState) -> String {
    let rank =
        state.progression.quests.merc.rank.max(i16::from(state.progression.guild_rank)).max(1);
    match rank {
        0 | 1 => {
            "Recruiting Centurion order: prove yourself in the field and reach 400 legion service XP."
                .to_string()
        }
        2 => {
            "Command order: maintain discipline and reach 1500 legion service XP for Force-Leader promotion."
                .to_string()
        }
        3 => "Colonel's writ: recover the Regalia of the Demon Emperor from the southern volcano."
            .to_string(),
        _ => "Commandant briefing: review force disposition and uphold legion readiness.".to_string(),
    }
}

fn inventory_has_item_with_fragments(state: &GameState, fragments: &[&str]) -> bool {
    state.player.inventory.iter().any(|item| {
        let lower = item.name.to_ascii_lowercase();
        fragments.iter().all(|fragment| lower.contains(fragment))
    })
}

fn remove_inventory_item_with_fragments(state: &mut GameState, fragments: &[&str]) -> Option<Item> {
    let item_id = state.player.inventory.iter().find_map(|item| {
        let lower = item.name.to_ascii_lowercase();
        fragments.iter().all(|fragment| lower.contains(fragment)).then_some(item.id)
    })?;
    remove_inventory_item_by_id(state, item_id)
}

fn castle_quest_briefing_for_rank(rank: i16) -> String {
    match rank {
        i16::MIN..=0 => {
            "His Grace decrees your first quest: bring the head of the Goblin King.".to_string()
        }
        1 => "Bring to me a Holy Defender from the depths below Rampart.".to_string(),
        2 => "Bring me a suit of dragonscale armor to prove your knighthood.".to_string(),
        3 => "A final service is required: recover the Orb of Mastery from the Astral reaches."
            .to_string(),
        _ => "The Duchy acknowledges your prior service; your court duties now concern governance."
            .to_string(),
    }
}

fn order_next_duty_briefing(state: &GameState) -> String {
    let rank = state.progression.quests.order.rank.max(0);
    if rank <= 0 {
        return "The Order offers induction if you renounce rival blood-oaths and uphold Law."
            .to_string();
    }
    if rank >= 4 {
        if inventory_has_item_with_fragments(state, &["star", "gem"]) {
            return "You bear the Star Gem; present it to claim the Justiciar succession."
                .to_string();
        }
        return "Paladin duty remains: recover the Star Gem and return it to the LawBringer."
            .to_string();
    }
    if rank == 3 {
        return "Paladin review pending: lawful standing and field record must exceed Justiciar standards."
            .to_string();
    }
    if rank == 2 {
        return "Chevalier advancement requires stronger lawful standing and greater service record."
            .to_string();
    }
    "Gallant briefing: continue patrol service until promotion board requirements are met."
        .to_string()
}

fn site_aux_to_objective_label(aux: i32) -> Option<&'static str> {
    match aux {
        SITE_AUX_SERVICE_MERC_GUILD => Some("Visit the Mercenary Guild for orders."),
        SITE_AUX_SERVICE_ORDER => Some("Report to the Order hall."),
        SITE_AUX_SERVICE_CASTLE => Some("Report to the Castle audience chamber."),
        SITE_AUX_SERVICE_PALACE => Some("Report to the Palace court."),
        SITE_AUX_SERVICE_TEMPLE => Some("Visit the Temple for rites."),
        SITE_AUX_SERVICE_ARENA => Some("Report to the Arena registrar."),
        SITE_AUX_SERVICE_THIEVES => Some("Meet the Thieves Guild contact."),
        SITE_AUX_SERVICE_COLLEGE => Some("Visit the College for instruction."),
        SITE_AUX_SERVICE_SORCERORS => Some("Visit the Sorcerors circle."),
        SITE_AUX_SERVICE_MONASTERY => Some("Visit the Monastery."),
        SITE_AUX_SERVICE_TAVERN => Some("Visit the Tavern for rumors."),
        SITE_AUX_SERVICE_COMMANDANT => Some("Visit the Commandant for supplies."),
        SITE_AUX_SERVICE_ARMORER => Some("Visit the Armorer."),
        SITE_AUX_SERVICE_BANK => Some("Visit the Bank."),
        SITE_AUX_SERVICE_CHARITY => Some("Visit the Poor House."),
        _ => None,
    }
}

fn objective_site_position(state: &GameState, aux: i32) -> Option<Position> {
    let width = usize::try_from(state.bounds.width).ok()?;
    if width == 0 {
        return None;
    }
    state.site_grid.iter().enumerate().find_map(|(idx, cell)| {
        if cell.aux != aux {
            return None;
        }
        let x = i32::try_from(idx % width).ok()?;
        let y = i32::try_from(idx / width).ok()?;
        let raw = Position { x, y };
        Some(objective_approach_position(state, raw))
    })
}

fn objective_approach_position(state: &GameState, target: Position) -> Position {
    let glyph = state.map_glyph_at(target);
    let door_like = matches!(glyph, '-' | '/' | '|' | 'D' | 'J');
    if state.tile_is_walkable(target) && !door_like {
        return target;
    }
    let around = [
        Position { x: target.x, y: target.y - 1 },
        Position { x: target.x + 1, y: target.y },
        Position { x: target.x, y: target.y + 1 },
        Position { x: target.x - 1, y: target.y },
        Position { x: target.x + 1, y: target.y - 1 },
        Position { x: target.x + 1, y: target.y + 1 },
        Position { x: target.x - 1, y: target.y + 1 },
        Position { x: target.x - 1, y: target.y - 1 },
    ];
    let mut best: Option<(Position, i32)> = None;
    for pos in around {
        if pos == state.player.position
            || !state.bounds.contains(pos)
            || !state.tile_is_walkable(pos)
        {
            continue;
        }
        let distance = pos.manhattan_distance(state.player.position);
        match best {
            Some((best_pos, best_distance))
                if distance > best_distance
                    || (distance == best_distance
                        && (pos.y > best_pos.y
                            || (pos.y == best_pos.y && pos.x >= best_pos.x))) => {}
            _ => best = Some((pos, distance)),
        }
    }
    best.map(|(pos, _)| pos).unwrap_or(target)
}

fn objective_hints_from_summary(state: &GameState, summary: &str) -> Vec<ObjectiveHint> {
    let lower = summary.to_ascii_lowercase();
    let mut hints: Vec<ObjectiveHint> = Vec::new();
    let mut push_hint = |aux: i32| {
        if hints.iter().any(|hint| hint.target == objective_site_position(state, aux)) {
            return;
        }
        if let Some(label) = site_aux_to_objective_label(aux) {
            hints.push(ObjectiveHint {
                label: label.to_string(),
                target: objective_site_position(state, aux),
            });
        }
    };

    if lower.contains("merc") || lower.contains("legion") || lower.contains("centurion") {
        push_hint(SITE_AUX_SERVICE_MERC_GUILD);
    }
    if lower.contains("order") || lower.contains("law") || lower.contains("justiciar") {
        push_hint(SITE_AUX_SERVICE_ORDER);
    }
    if lower.contains("castle")
        || lower.contains("duke")
        || lower.contains("goblin king")
        || lower.contains("holy defender")
        || lower.contains("dragonscale")
    {
        push_hint(SITE_AUX_SERVICE_CASTLE);
    }
    if lower.contains("palace") || lower.contains("court") {
        push_hint(SITE_AUX_SERVICE_PALACE);
    }
    if lower.contains("temple")
        || lower.contains("altar")
        || lower.contains("deity")
        || lower.contains("blessing")
    {
        push_hint(SITE_AUX_SERVICE_TEMPLE);
    }
    if lower.contains("arena") || lower.contains("champion") || lower.contains("vict") {
        push_hint(SITE_AUX_SERVICE_ARENA);
    }
    if lower.contains("thief") {
        push_hint(SITE_AUX_SERVICE_THIEVES);
    }
    if lower.contains("college") {
        push_hint(SITE_AUX_SERVICE_COLLEGE);
    }
    if lower.contains("sorcer") || lower.contains("wizard") {
        push_hint(SITE_AUX_SERVICE_SORCERORS);
    }
    if lower.contains("monastery") {
        push_hint(SITE_AUX_SERVICE_MONASTERY);
    }
    if lower.contains("rumor") || lower.contains("tavern") {
        push_hint(SITE_AUX_SERVICE_TAVERN);
    }
    hints
}

pub fn objective_journal(state: &GameState) -> Vec<ObjectiveSnapshot> {
    let mut journal = Vec::new();
    let quest_state = state.progression.quest_state;
    let objective_text = state.progression.main_quest.objective.trim();
    if quest_state != LegacyQuestState::NotStarted || !objective_text.is_empty() {
        let completed =
            matches!(quest_state, LegacyQuestState::Completed | LegacyQuestState::Failed);
        let summary = if !objective_text.is_empty() {
            objective_text.to_string()
        } else {
            format!("Quest state {:?}", quest_state)
        };
        let mut steps = Vec::new();
        steps.push(ObjectiveStep {
            id: "main-stage".to_string(),
            description: format!("Stage {:?}", state.progression.main_quest.stage),
            complete: completed,
        });
        if state.progression.quest_steps_completed > 0 {
            steps.push(ObjectiveStep {
                id: "main-steps".to_string(),
                description: format!("Completed steps {}", state.progression.quest_steps_completed),
                complete: completed,
            });
        }
        journal.push(ObjectiveSnapshot {
            id: "main_quest".to_string(),
            title: "Main Quest".to_string(),
            summary: summary.clone(),
            active: !completed,
            completed,
            steps,
            hints: objective_hints_from_summary(state, &summary),
        });
    }

    let push_track = |rows: &mut Vec<ObjectiveSnapshot>,
                      id: &str,
                      title: &str,
                      track: &GuildTrackState,
                      details: Option<String>| {
        let engaged = track.rank > 0
            || track.xp > 0
            || track.dues_paid > 0
            || track.salary_due > 0
            || track.promotion_flags != 0
            || track.quest_flags != 0;
        if !engaged {
            return;
        }
        let mut summary = format!("Rank {} XP {}", track.rank, track.xp);
        if track.salary_due > 0 {
            summary.push_str(&format!(" SalaryDue {}", track.salary_due));
        }
        if track.dues_paid > 0 {
            summary.push_str(&format!(" DuesPaid {}", track.dues_paid));
        }
        if let Some(extra) = details {
            summary.push_str(": ");
            summary.push_str(extra.trim());
        }
        rows.push(ObjectiveSnapshot {
            id: id.to_string(),
            title: title.to_string(),
            summary: summary.clone(),
            active: true,
            completed: false,
            steps: vec![ObjectiveStep {
                id: format!("{id}_rank"),
                description: format!("Current rank {}", track.rank),
                complete: false,
            }],
            hints: objective_hints_from_summary(state, &format!("{title} {summary}")),
        });
    };

    push_track(
        &mut journal,
        "merc",
        "Mercenary Guild",
        &state.progression.quests.merc,
        Some(merc_contract_objective(state)),
    );
    push_track(
        &mut journal,
        "order",
        "Order",
        &state.progression.quests.order,
        Some(order_next_duty_briefing(state)),
    );
    push_track(
        &mut journal,
        "castle",
        "Castle",
        &state.progression.quests.castle,
        Some(castle_quest_briefing_for_rank(state.progression.quests.castle.rank)),
    );
    push_track(
        &mut journal,
        "arena",
        "Arena",
        &state.progression.quests.arena,
        Some(format!(
            "Arena rank {} opponent {}",
            state.progression.arena_rank, state.progression.arena_opponent
        )),
    );
    push_track(&mut journal, "temple", "Temple", &state.progression.quests.temple, None);
    push_track(&mut journal, "thieves", "Thieves Guild", &state.progression.quests.thieves, None);
    push_track(&mut journal, "college", "College", &state.progression.quests.college, None);
    push_track(&mut journal, "sorcerors", "Sorcerors", &state.progression.quests.sorcerors, None);
    push_track(&mut journal, "palace", "Palace", &state.progression.quests.palace, None);
    push_track(&mut journal, "monastery", "Monastery", &state.progression.quests.monastery, None);

    journal
}

pub fn active_objective_snapshot(state: &GameState) -> Option<ObjectiveSnapshot> {
    let journal = objective_journal(state);
    journal.iter().find(|entry| entry.active).cloned().or_else(|| journal.first().cloned())
}

pub fn objective_map_hints(state: &GameState) -> Vec<Position> {
    let mut hints = Vec::new();
    if let Some(active) = active_objective_snapshot(state) {
        for hint in active.hints {
            if let Some(target) = hint.target
                && !hints.contains(&target)
            {
                hints.push(target);
            }
        }
    }
    hints
}

fn advance_main_quest_from_court_audience(
    state: &mut GameState,
    events: &mut Vec<Event>,
) -> Option<String> {
    if state.progression.quest_state == LegacyQuestState::NotStarted {
        let _ = start_main_quest_from_dialogue(state, events);
        return Some("A formal charge is issued: prove your worth through service.".to_string());
    }
    if state.progression.quest_state == LegacyQuestState::ArtifactRecovered {
        state.progression.quest_state = LegacyQuestState::ReturnToPatron;
        state.progression.main_quest.stage = state.progression.quest_state;
        state.progression.quest_steps_completed = 3;
        events.push(Event::QuestAdvanced {
            state: state.progression.quest_state,
            steps_completed: state.progression.quest_steps_completed,
        });
        return Some(
            "Your report is accepted. Return to your patron for final investiture.".to_string(),
        );
    }
    if state.progression.quest_state == LegacyQuestState::ReturnToPatron
        && state.progression.guild_rank >= 2
        && state.progression.priest_rank >= 1
    {
        state.progression.quest_state = LegacyQuestState::Completed;
        state.progression.main_quest.stage = state.progression.quest_state;
        state.progression.quest_steps_completed = 4;
        state.progression.main_quest.completion_flags |= 0x0010;
        events.push(Event::QuestAdvanced {
            state: state.progression.quest_state,
            steps_completed: state.progression.quest_steps_completed,
        });
        return Some(
            "The court confirms your charter and records your completed service.".to_string(),
        );
    }
    None
}

fn apply_castle_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let mut notes = vec!["The castellan ushers you into the castle before His Grace.".to_string()];
    let mut rank = state.progression.quests.castle.rank.max(0);
    if rank == 0 {
        if state.progression.quest_state == LegacyQuestState::NotStarted {
            let _ = start_main_quest_from_dialogue(state, events);
        }
        rank = 1;
        state.progression.quests.castle.rank = rank;
        state.progression.quests.castle.xp = state.progression.quests.castle.xp.saturating_add(25);
        state.progression.main_quest.objective = castle_quest_briefing_for_rank(rank - 1);
        state.progression.quests.castle.quest_flags |= 0x0001;
        notes.push(state.progression.main_quest.objective.clone());
        return notes.join(" ");
    }

    if rank == 1 {
        if remove_inventory_item_with_fragments(state, &["goblin", "king"]).is_some() {
            state.progression.quests.castle.rank = 2;
            state.progression.quests.castle.xp =
                state.progression.quests.castle.xp.saturating_add(100);
            state.progression.main_quest.objective = castle_quest_briefing_for_rank(1);
            notes.push("Good job, sirrah! You are promoted to esquire.".to_string());
            notes.push(state.progression.main_quest.objective.clone());
        } else {
            notes
                .push("Do not return until you bring the Goblin King's head, caitiff.".to_string());
        }
        return notes.join(" ");
    }

    if rank == 2 {
        if remove_inventory_item_with_fragments(state, &["defender"]).is_some() {
            state.progression.quests.castle.rank = 3;
            state.progression.quests.castle.xp =
                state.progression.quests.castle.xp.saturating_add(250);
            state.progression.main_quest.objective = castle_quest_briefing_for_rank(2);
            notes.push("My thanks, squire. In return, I dub thee knight.".to_string());
            notes.push(state.progression.main_quest.objective.clone());
        } else {
            notes.push("Greetings, squire. Bring me the Holy Defender.".to_string());
        }
        return notes.join(" ");
    }

    if rank == 3 {
        if remove_inventory_item_with_fragments(state, &["dragon", "scale"]).is_some()
            || remove_inventory_item_with_fragments(state, &["dragonscale"]).is_some()
        {
            state.progression.quests.castle.rank = 4;
            state.progression.quests.castle.xp =
                state.progression.quests.castle.xp.saturating_add(500);
            state.progression.main_quest.objective = castle_quest_briefing_for_rank(3);
            state.progression.main_quest.palace_access = true;
            notes.push(
                "Thanks, good sir knight. You are granted letters patent to a peerage.".to_string(),
            );
            notes.push(state.progression.main_quest.objective.clone());
        } else {
            notes.push(
                "Your quest is not yet complete, sir knight. Bring dragonscale armor.".to_string(),
            );
        }
        return notes.join(" ");
    }

    if rank == 4 {
        if inventory_has_item_with_fragments(state, &["orb", "mastery"]) {
            state.progression.quests.castle.rank = 5;
            state.progression.quests.castle.xp =
                state.progression.quests.castle.xp.saturating_add(1200);
            state.progression.main_quest.palace_access = true;
            notes.push(
                "My sincerest thanks, my lord. The Duchy recognizes your paragon service."
                    .to_string(),
            );
            notes.push("You may keep the Orb of Mastery; court gates now stand open.".to_string());
            if let Some(quest_note) = advance_main_quest_from_court_audience(state, events) {
                notes.push(quest_note);
            }
        } else {
            notes.push("I require the Orb of Mastery before final investiture.".to_string());
        }
        return notes.join(" ");
    }

    notes.push("The court records your completed service and governance standing.".to_string());
    if let Some(quest_note) = advance_main_quest_from_court_audience(state, events) {
        notes.push(quest_note);
    }
    notes.join(" ")
}

fn apply_order_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let mut notes =
        vec!["The Headquarters of the Order of Paladins convenes your audience.".to_string()];
    let quest_started = start_main_quest_from_dialogue(state, events);
    let rank = state.progression.quests.order.rank;
    let alignment_before = state.progression.alignment;

    if alignment_before == Alignment::Chaotic || state.progression.law_chaos_score < 0 {
        if rank > 0 {
            state.progression.quests.order.rank = -1;
            state.legal_heat = state.legal_heat.saturating_add(3);
            state.progression.quests.order.quest_flags |= 0x0001;
            notes.push(
                "You have been tainted by chaos and stripped of rank in the Order.".to_string(),
            );
            notes.push("Guards escort you toward the city jail.".to_string());
        } else {
            notes.push("Get thee hence, minion of chaos.".to_string());
        }
        return notes.join(" ");
    }

    if rank < 0 {
        return "Thou again? Get thee hence, minion of chaos.".to_string();
    }

    state.progression.alignment = Alignment::Lawful;
    state.progression.law_chaos_score = state.progression.law_chaos_score.max(5);

    if rank == 0 {
        if state.progression.arena_rank > 0 {
            return "We do not accept bloodstained gladiators into our Order.".to_string();
        }
        if state.progression.guild_rank > 0 {
            return "Go back to your barracks, mercenary.".to_string();
        }
        state.progression.quests.order.rank = 1;
        state.progression.quests.order.xp = state.progression.quests.order.xp.saturating_add(1);
        state.progression.alignment = Alignment::Lawful;
        state.progression.law_chaos_score = state.progression.law_chaos_score.max(4);
        state.progression.quests.order.quest_flags |= 0x0001;
        notes
            .push("You are now a Gallant in the Order and are issued a blessed spear.".to_string());
        if quest_started {
            notes.push("The Order records your first formal service quest.".to_string());
        }
        notes.push(order_next_duty_briefing(state));
        events.push(Event::ProgressionUpdated {
            guild_rank: state.progression.guild_rank,
            priest_rank: state.progression.priest_rank,
            alignment: state.progression.alignment,
        });
        return notes.join(" ");
    }

    state.legal_heat = state.legal_heat.saturating_sub(1);
    state.progression.quests.order.xp = state.progression.quests.order.xp.saturating_add(20);

    if rank >= 4 && inventory_has_item_with_fragments(state, &["star", "gem"]) {
        state.progression.quests.order.rank = 5;
        state.progression.quests.order.quest_flags |= 0x0010;
        let _ = add_item_to_inventory_or_ground(state, "blessed shield of deflection", events);
        notes.push("The previous Justiciar steps down in your favor.".to_string());
        notes.push("You are now the Justiciar of Rampart and the Order.".to_string());
        notes.push("You are awarded a blessed shield of deflection.".to_string());
    } else if rank == 3 && state.progression.quests.order.xp >= 4000 {
        state.progression.quests.order.rank = 4;
        state.progression.quests.order.quest_flags |= 0x0008;
        let _ = add_item_to_inventory_or_ground(state, "mithril plate armor", events);
        notes.push("You are made a Paladin of the Order and granted mithril plate.".to_string());
        notes.push(order_next_duty_briefing(state));
    } else if rank == 2 && state.progression.quests.order.xp >= 2000 {
        state.progression.quests.order.rank = 3;
        notes.push("You are promoted to Chevalier by the Order council.".to_string());
        notes.push(order_next_duty_briefing(state));
    } else if rank == 1 && state.progression.quests.order.xp >= 800 {
        state.progression.quests.order.rank = 2;
        notes.push("You are promoted to Champion of the Order.".to_string());
        notes.push(order_next_duty_briefing(state));
    } else {
        notes.push(order_next_duty_briefing(state));
    }

    if let Some(quest_note) = advance_main_quest_from_court_audience(state, events) {
        notes.push(quest_note);
    }
    events.push(Event::ProgressionUpdated {
        guild_rank: state.progression.guild_rank,
        priest_rank: state.progression.priest_rank,
        alignment: state.progression.alignment,
    });
    notes.join(" ")
}

fn apply_palace_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    state.progression.main_quest.palace_access = true;
    state.progression.quests.palace.rank = state.progression.quests.palace.rank.max(1);
    state.progression.quests.palace.xp = state.progression.quests.palace.xp.saturating_add(20);
    let mut notes =
        vec!["The chamberlain receives your audience request in the palace hall.".to_string()];
    if let Some(quest_note) = advance_main_quest_from_court_audience(state, events) {
        notes.push(quest_note);
    } else {
        notes.push("No new royal decree is issued at this time.".to_string());
    }
    notes.join(" ")
}

fn apply_temple_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Your prayer echoes through the temple nave.".to_string()];
    if quest_started {
        notes.push("An omen marks the beginning of your service quest.".to_string());
    }
    if state.progression.priest_rank == 0 && state.progression.deity_favor >= 4 {
        state.progression.priest_rank = 1;
        state.progression.quests.temple.rank =
            state.progression.quests.temple.rank.max(i16::from(state.progression.priest_rank));
        events.push(Event::ProgressionUpdated {
            guild_rank: state.progression.guild_rank,
            priest_rank: state.progression.priest_rank,
            alignment: state.progression.alignment,
        });
        notes.push("The clergy name you an initiate for your devotion.".to_string());
    } else if state.progression.deity_favor < 4 {
        notes.push("The priests urge greater devotion before promotion.".to_string());
    }
    notes.join(" ")
}

fn apply_armorer_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The armorer measures you for steel and mail.".to_string()];
    notes.push("Both armor and weapons are available for commissioned work.".to_string());
    if quest_started {
        notes.push("The armorer warns that real service demands reliable gear.".to_string());
    }
    notes.join(" ")
}

fn apply_club_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes =
        vec!["Club stewards discuss membership etiquette and civic favors.".to_string()];
    if state.legal_heat > 0 {
        notes.push("Quiet donations can reduce scrutiny from city watch ledgers.".to_string());
    }
    if quest_started {
        notes.push("Patrons mention that service record matters more than bravado.".to_string());
    }
    notes.join(" ")
}

fn apply_gym_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    state.progression.quests.merc.rank = state.progression.quests.merc.rank.max(1);
    let mut notes = vec!["The gym master posts drills and paid spar contracts.".to_string()];
    notes.push("Your conditioning record can support mercenary advancement.".to_string());
    if quest_started {
        notes.push("A first contract is available for proven discipline.".to_string());
    }
    notes.join(" ")
}

fn apply_healer_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The healer offers wound treatment and antidotes.".to_string()];
    if state.status_effects.iter().any(|effect| effect.id == "poison") {
        notes.push("Poison symptoms are diagnosed immediately.".to_string());
    }
    if quest_started {
        notes.push("The healer advises preparation before taking formal contracts.".to_string());
    }
    notes.join(" ")
}

fn apply_casino_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Casino clerks explain chip purchase and table limits.".to_string()];
    notes.push("The house always takes a cut over time.".to_string());
    if quest_started {
        notes.push("Rumors tie casino debts to local power brokers.".to_string());
    }
    notes.join(" ")
}

fn apply_commandant_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The commandant barks logistics orders to passing recruits.".to_string()];
    notes.push("Supply office bulletin: Buy a bucket!".to_string());
    if quest_started {
        notes.push("Patrol reports can improve your standing with lawful authorities.".to_string());
    }
    notes.join(" ")
}

fn apply_diner_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Diner staff offer quick meals and strong coffee.".to_string()];
    if state.food <= 2 {
        notes.push("They suggest eating before your next expedition.".to_string());
    }
    if quest_started {
        notes.push("Travelers exchange practical rumors over late-night plates.".to_string());
    }
    notes.join(" ")
}

fn apply_craps_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Dice runners outline buy-in and payout customs.".to_string()];
    notes.push("Games here attract attention from both thieves and guards.".to_string());
    if quest_started {
        notes.push("A sharp eye for odds may help in risk-heavy assignments.".to_string());
    }
    notes.join(" ")
}

fn apply_tavern_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The tavern keeper offers ale, stew, and paid rumors.".to_string()];
    if state.legal_heat > 0 {
        notes.push("Watch patrol gossip suggests you keep a low profile.".to_string());
    }
    if quest_started {
        notes.push("Mercs and priests trade clues tied to the broader quest chain.".to_string());
    }
    notes.join(" ")
}

fn apply_pawn_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The pawnbroker inspects oddities and offers hard bargains.".to_string()];
    if state.player.inventory.is_empty() {
        notes.push("Without goods to sell, only risky buys remain.".to_string());
    }
    if quest_started {
        notes.push("Recovered curios can be liquidated here between missions.".to_string());
    }
    notes.join(" ")
}

fn apply_brothel_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The madam quotes room rates and discreet information fees.".to_string()];
    if state.progression.quests.thieves.rank > 0 {
        notes.push("Guild whispers are often traded after midnight.".to_string());
    }
    if quest_started {
        notes.push("Informants hint that social venues hide strategic leads.".to_string());
    }
    notes.join(" ")
}

fn apply_condo_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Condo stewards offer secure rest and lockbox services.".to_string()];
    notes.push("Long campaigns benefit from stable lodging and storage.".to_string());
    if quest_started {
        notes
            .push("Your service standing may eventually unlock better accommodations.".to_string());
    }
    notes.join(" ")
}

fn apply_shop_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The merchant quotes current prices and supply routes.".to_string()];
    if state.gold <= 0 {
        notes.push("You will need coin before any trade can proceed.".to_string());
    }
    if quest_started {
        notes.push("The shopkeeper mentions travelers seeking proven adventurers.".to_string());
    }
    notes.join(" ")
}

fn apply_bank_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["The banker reviews your account and civic surety options.".to_string()];
    if state.bank_gold > 0 {
        notes.push(format!("Current balance on record: {} gold.", state.bank_gold));
    } else {
        notes.push("No deposits are currently on record.".to_string());
    }
    if quest_started {
        notes.push("A clerk hints that patrons value disciplined service.".to_string());
    }
    notes.join(" ")
}

fn apply_merc_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    state.progression.quests.merc.rank = state.progression.quests.merc.rank.max(1);
    let mut notes =
        vec!["The quartermaster reviews contracts and promotion requirements.".to_string()];
    if state.progression.arena_rank > 0 {
        notes.push(
            "Legion records flag your arena service; command reviews this conflict carefully."
                .to_string(),
        );
    }
    if state.progression.quests.order.rank > 0 {
        notes.push("Paladin oaths conflict with legion command service.".to_string());
    }
    if state.progression.guild_rank <= 0 {
        if state.attributes.constitution < 12 || state.attributes.strength < 10 {
            notes.push(
                "The recruiting centurion says your physicals are below legion standards."
                    .to_string(),
            );
        } else {
            notes.push(
                "The recruiting centurion offers induction drills for legion service.".to_string(),
            );
        }
    } else {
        notes.push(format!("Current mercenary rank recorded at {}.", state.progression.guild_rank));
    }
    notes.push(merc_contract_objective(state));
    if quest_started {
        notes.push("A first contract is posted to begin your service record.".to_string());
    }
    notes.join(" ")
}

fn apply_thieves_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes =
        vec!["The guild fence sizes up your risk tolerance and discretion.".to_string()];
    if state.progression.quests.thieves.rank <= 0 {
        notes.push("Dues must be paid before confidential work is offered.".to_string());
    } else {
        notes.push(format!(
            "Whispers confirm your standing at thieves rank {}.",
            state.progression.quests.thieves.rank
        ));
    }
    if state.progression.alignment == Alignment::Lawful {
        notes.push("Your lawful bearing draws immediate suspicion.".to_string());
    }
    if quest_started {
        notes.push("Rumors tie your fate to the wider conflict in the city.".to_string());
    }
    notes.join(" ")
}

fn apply_college_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let known_spells = state.spellbook.spells.iter().filter(|spell| spell.known).count();
    let mut notes =
        vec!["The collegium registrar reviews tuition and sanctioned studies.".to_string()];
    notes.push(format!("Known spell forms on file: {}.", known_spells));
    if quest_started {
        notes.push("Faculty note your name for forthcoming service examinations.".to_string());
    }
    notes.join(" ")
}

fn apply_sorcerors_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes =
        vec!["A sorceror archivist outlines dues, research, and transmutation rites.".to_string()];
    if state.progression.alignment == Alignment::Lawful {
        notes.push("Your lawful oath limits access to forbidden grimoires.".to_string());
    } else {
        notes.push("The circle permits controlled study of chaotic formulae.".to_string());
    }
    if quest_started {
        notes.push("The archivist links your studies to the broader quest chain.".to_string());
    }
    notes.join(" ")
}

fn apply_charity_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes = vec!["Charity stewards ask whether you seek aid or service work.".to_string()];
    if state.player.stats.hp < state.player.stats.max_hp || state.food <= 0 {
        notes.push("They recommend immediate relief before further travel.".to_string());
    } else {
        notes.push("You are invited to volunteer with civic relief patrols.".to_string());
    }
    if quest_started {
        notes.push("The stewards mention your growing reputation among patrons.".to_string());
    }
    notes.join(" ")
}

fn apply_arena_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    let mut notes =
        vec!["Arena officials brief you on registration, purses, and gate rules.".to_string()];
    if state.progression.arena_rank <= 0 {
        notes.push("You are currently listed as an unranked challenger.".to_string());
    } else {
        notes.push(format!(
            "Your arena rank {} credentials remain active.",
            state.progression.arena_rank
        ));
    }
    if state.progression.arena_match_active {
        notes.push("A current bout is recorded as still in progress.".to_string());
    }
    if quest_started {
        notes.push("Officials note that arena success influences court recognition.".to_string());
    }
    notes.join(" ")
}

fn apply_monastery_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let quest_started = start_main_quest_from_dialogue(state, events);
    state.progression.quests.monastery.rank = state.progression.quests.monastery.rank.max(1);
    let mut notes = vec![
        "Monastery wardens discuss discipline vows, study stipends, and service duty.".to_string(),
    ];
    notes.push(format!(
        "Monastic standing recorded at rank {}.",
        state.progression.quests.monastery.rank
    ));
    if quest_started {
        notes.push("The wardens mark your quest obligations in their ledger.".to_string());
    }
    notes.join(" ")
}

fn apply_generic_talk_command(
    state: &mut GameState,
    events: &mut Vec<Event>,
    speaker: &str,
) -> String {
    if start_main_quest_from_dialogue(state, events) {
        format!("The {speaker} points you toward service and duty.")
    } else {
        format!("You exchange a few words with the {speaker}.")
    }
}

fn apply_talk_command(state: &mut GameState, events: &mut Vec<Event>) -> (String, bool) {
    let site_aux =
        state.tile_site_at(state.player.position).map(|site| site.aux).unwrap_or(SITE_AUX_NONE);
    let speaker = speaker_for_site_aux(state, site_aux);

    let note = match site_aux {
        SITE_AUX_SERVICE_SHOP => apply_shop_talk_command(state, events),
        SITE_AUX_SERVICE_ARMORER => apply_armorer_talk_command(state, events),
        SITE_AUX_SERVICE_CLUB => apply_club_talk_command(state, events),
        SITE_AUX_SERVICE_GYM => apply_gym_talk_command(state, events),
        SITE_AUX_SERVICE_HEALER => apply_healer_talk_command(state, events),
        SITE_AUX_SERVICE_CASINO => apply_casino_talk_command(state, events),
        SITE_AUX_SERVICE_COMMANDANT => apply_commandant_talk_command(state, events),
        SITE_AUX_SERVICE_DINER => apply_diner_talk_command(state, events),
        SITE_AUX_SERVICE_CRAPS => apply_craps_talk_command(state, events),
        SITE_AUX_SERVICE_TAVERN => apply_tavern_talk_command(state, events),
        SITE_AUX_SERVICE_PAWN_SHOP => apply_pawn_talk_command(state, events),
        SITE_AUX_SERVICE_BROTHEL => apply_brothel_talk_command(state, events),
        SITE_AUX_SERVICE_CONDO => apply_condo_talk_command(state, events),
        SITE_AUX_SERVICE_BANK => apply_bank_talk_command(state, events),
        SITE_AUX_SERVICE_MERC_GUILD => apply_merc_talk_command(state, events),
        SITE_AUX_SERVICE_THIEVES => apply_thieves_talk_command(state, events),
        SITE_AUX_SERVICE_COLLEGE => apply_college_talk_command(state, events),
        SITE_AUX_SERVICE_SORCERORS => apply_sorcerors_talk_command(state, events),
        SITE_AUX_SERVICE_CASTLE => apply_castle_talk_command(state, events),
        SITE_AUX_SERVICE_ORDER => apply_order_talk_command(state, events),
        SITE_AUX_SERVICE_PALACE => apply_palace_talk_command(state, events),
        SITE_AUX_SERVICE_TEMPLE => apply_temple_talk_command(state, events),
        SITE_AUX_SERVICE_CHARITY => apply_charity_talk_command(state, events),
        SITE_AUX_SERVICE_ARENA => apply_arena_talk_command(state, events),
        SITE_AUX_SERVICE_MONASTERY => apply_monastery_talk_command(state, events),
        _ => apply_generic_talk_command(state, events, speaker),
    };

    events.push(Event::DialogueAdvanced {
        speaker: speaker.to_string(),
        quest_state: state.progression.quest_state,
    });
    (note, true)
}

fn resolve_countryside_hunt<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> (String, u64) {
    if state.country_grid.cells.is_empty() {
        state.food += 1;
        let item_name = format!("foraged ration {}", state.next_item_id);
        state.place_item(item_name, state.player.position);
        return ("hunt completed; food stock increased".to_string(), 0);
    }
    let terrain = state
        .country_cell_at(state.player.position)
        .map(|cell| cell.current_terrain)
        .unwrap_or(CountryTerrainKind::Plains);
    let bonus = terrain_hunt_minutes(terrain).saturating_sub(180);
    let food_gain = match terrain {
        CountryTerrainKind::Forest | CountryTerrainKind::Jungle => 2,
        CountryTerrainKind::Swamp | CountryTerrainKind::Mountains => 1,
        _ => 1,
    };
    state.food += food_gain;
    let item_name = format!("foraged ration {}", state.next_item_id);
    state.place_item(item_name, state.player.position);

    let encountered = spawn_countryside_encounter(state, rng, events, terrain);
    let note = if encountered {
        "hunt completed; food gathered and an encounter appeared"
    } else {
        "hunt completed; food gathered"
    };
    (note.to_string(), bonus)
}

fn apply_countryside_search<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> u64 {
    if state.country_grid.cells.is_empty() {
        return 0;
    }
    let terrain = state
        .country_cell_at(state.player.position)
        .map(|cell| cell.current_terrain)
        .unwrap_or(CountryTerrainKind::Plains);
    let bonus = terrain_search_minutes(terrain).saturating_sub(60);
    let _ = spawn_countryside_encounter(state, rng, events, terrain);
    bonus
}

fn apply_countryside_travel<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> u64 {
    if state.country_grid.cells.is_empty() {
        return 0;
    }
    let terrain = state
        .country_cell_at(state.player.position)
        .map(|cell| cell.current_terrain)
        .unwrap_or(CountryTerrainKind::Plains);
    let bonus = terrain_travel_minutes(terrain).saturating_sub(60);
    apply_country_travel_hazards(state, rng, events, terrain);
    if state.status != SessionStatus::InProgress {
        return bonus;
    }
    let _ = spawn_countryside_encounter(state, rng, events, terrain);
    bonus
}

fn apply_country_travel_hazards<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
    terrain: CountryTerrainKind,
) {
    if state.world_mode != WorldMode::Countryside {
        return;
    }
    let was_seen = state.known_sites.iter().any(|pos| *pos == state.player.position);
    if terrain == CountryTerrainKind::ChaosSea {
        apply_chaos_sea_immersion(state, events);
        if state.status != SessionStatus::InProgress {
            return;
        }
    }
    let newly_lost = apply_poppy_field_event(state, rng, events);
    resolve_navigation_reorientation(state, was_seen, !newly_lost, events);
}

fn apply_poppy_field_event<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> bool {
    let roll = rng.range_inclusive_i32(0, 299);
    match roll {
        0 => {
            state.precipitation =
                state.precipitation.saturating_add(rng.range_inclusive_i32(1, 12));
            state.navigation_lost = true;
            let note = "Inclement weather throws off your bearings; you become lost.".to_string();
            push_timeline_line(state, note.clone());
            events.push(Event::LegacyHandled {
                token: "country-weather".to_string(),
                note,
                fully_modeled: true,
            });
            true
        }
        1 => {
            state.navigation_lost = true;
            let note = "You wander into a field of poppies and become disoriented.".to_string();
            push_timeline_line(state, note.clone());
            events.push(Event::LegacyHandled {
                token: "country-poppy".to_string(),
                note,
                fully_modeled: true,
            });
            true
        }
        _ => false,
    }
}

fn apply_chaos_sea_immersion(state: &mut GameState, events: &mut Vec<Event>) {
    if state.chaos_attuned {
        state.player.stats.hp = state.player.stats.max_hp;
        state.spellbook.mana = state.spellbook.max_mana;
        push_timeline_line(state, "The chaos sea doesn't seem to bother you at all.".to_string());
        return;
    }

    let can_attune =
        state.progression.quests.sorcerors.rank >= 2 || state.progression.guild_rank >= 4;
    if can_attune {
        state.chaos_attuned = true;
        state.player.stats.hp = state.player.stats.max_hp;
        state.spellbook.mana = state.spellbook.max_mana;
        let note = "You achieve oneness with Chaos.".to_string();
        push_timeline_line(state, note.clone());
        events.push(Event::LegacyHandled {
            token: "chaos-sea".to_string(),
            note,
            fully_modeled: true,
        });
        return;
    }

    if state.progression.priest_rank > 0 && !state.chaos_protection_consumed {
        state.chaos_protection_consumed = true;
        let note = "A mysterious force protects you from the chaos sea.".to_string();
        push_timeline_line(state, note.clone());
        events.push(Event::LegacyHandled {
            token: "chaos-sea".to_string(),
            note,
            fully_modeled: true,
        });
        return;
    }

    state.progression.alignment = Alignment::Chaotic;
    state.progression.law_chaos_score -= 50;
    mark_player_defeated(state, "immersion in raw Chaos", events);
}

fn resolve_navigation_reorientation(
    state: &mut GameState,
    was_seen_before_move: bool,
    allow_recover: bool,
    events: &mut Vec<Event>,
) {
    if state.navigation_lost {
        if allow_recover && state.precipitation < 1 && was_seen_before_move {
            state.navigation_lost = false;
            let note = "Ah! Now you know where you are!".to_string();
            push_timeline_line(state, note.clone());
            events.push(Event::LegacyHandled {
                token: "lost".to_string(),
                note,
                fully_modeled: true,
            });
        } else {
            push_timeline_line(state, "You're still lost.".to_string());
        }
    }
    if state.precipitation > 0 {
        state.precipitation -= 1;
    }
    ensure_known_site(state, state.player.position);
}

fn fallback_country_cell_from_rows(state: &GameState, pos: Position) -> Option<CountryCell> {
    if state.country_map_rows.is_empty() {
        return None;
    }
    let glyph = row_char_at(&state.country_map_rows, pos)?;
    let (base_terrain, current_terrain, aux) = match glyph {
        'v' => (CountryTerrainKind::Pass, CountryTerrainKind::Mountains, 0),
        '%' => (CountryTerrainKind::Castle, CountryTerrainKind::Mountains, 0),
        '|' => (CountryTerrainKind::StarPeak, CountryTerrainKind::Mountains, 0),
        '*' => (CountryTerrainKind::Caves, CountryTerrainKind::Mountains, 0),
        '!' => (CountryTerrainKind::Volcano, CountryTerrainKind::Mountains, 0),
        '$' => (CountryTerrainKind::DragonLair, CountryTerrainKind::Desert, 0),
        '&' => (CountryTerrainKind::MagicIsle, CountryTerrainKind::ChaosSea, 0),
        'K' => (CountryTerrainKind::Palace, CountryTerrainKind::Jungle, 0),
        'a'..='f' => {
            (CountryTerrainKind::Village, CountryTerrainKind::Village, 1 + (glyph as u8 - b'a'))
        }
        '1'..='6' => (CountryTerrainKind::Temple, CountryTerrainKind::Temple, glyph as u8 - b'0'),
        '-' => (CountryTerrainKind::Plains, CountryTerrainKind::Plains, 0),
        '_' => (CountryTerrainKind::Tundra, CountryTerrainKind::Tundra, 0),
        '.' => (CountryTerrainKind::Road, CountryTerrainKind::Road, 0),
        '^' => (CountryTerrainKind::Mountains, CountryTerrainKind::Mountains, 0),
        '~' => (CountryTerrainKind::River, CountryTerrainKind::River, 0),
        'O' => (CountryTerrainKind::City, CountryTerrainKind::City, 0),
        '(' => (CountryTerrainKind::Forest, CountryTerrainKind::Forest, 0),
        ')' => (CountryTerrainKind::Jungle, CountryTerrainKind::Jungle, 0),
        '=' => (CountryTerrainKind::Swamp, CountryTerrainKind::Swamp, 0),
        '"' => (CountryTerrainKind::Desert, CountryTerrainKind::Desert, 0),
        '+' => (CountryTerrainKind::ChaosSea, CountryTerrainKind::ChaosSea, 0),
        _ => (CountryTerrainKind::Unknown, CountryTerrainKind::Unknown, 0),
    };
    Some(CountryCell { glyph, base_terrain, current_terrain, aux, status: 0 })
}

fn row_char_at(rows: &[String], pos: Position) -> Option<char> {
    let y = usize::try_from(pos.y).ok()?;
    let row = rows.get(y)?;
    let x = usize::try_from(pos.x).ok()?;
    row.chars().nth(x)
}

fn set_row_char(rows: &mut [String], pos: Position, glyph: char) -> bool {
    let Some(y) = usize::try_from(pos.y).ok() else {
        return false;
    };
    let Some(x) = usize::try_from(pos.x).ok() else {
        return false;
    };
    let Some(row) = rows.get_mut(y) else {
        return false;
    };
    let mut chars: Vec<char> = row.chars().collect();
    let Some(slot) = chars.get_mut(x) else {
        return false;
    };
    *slot = glyph;
    *row = chars.into_iter().collect();
    true
}

fn guard_marker_positions(rows: &[String], bounds: MapBounds) -> Vec<Position> {
    let mut positions = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, glyph) in row.chars().enumerate() {
            if glyph != 'G' {
                continue;
            }
            let pos = Position { x: x as i32, y: y as i32 };
            if bounds.contains(pos) {
                positions.push(pos);
            }
        }
    }
    positions
}

fn guard_marker_stats() -> Stats {
    Stats { hp: 14, max_hp: 14, attack_min: 2, attack_max: 5, defense: 2 }
}

fn tile_index(bounds: MapBounds, pos: Position) -> Option<usize> {
    if !bounds.contains(pos) {
        return None;
    }
    let y = usize::try_from(pos.y).ok()?;
    let x = usize::try_from(pos.x).ok()?;
    let width = usize::try_from(bounds.width).ok()?;
    Some(y.saturating_mul(width).saturating_add(x))
}

fn terrain_travel_minutes(terrain: CountryTerrainKind) -> u64 {
    match terrain {
        CountryTerrainKind::Road => 30,
        CountryTerrainKind::Plains => 60,
        CountryTerrainKind::Forest => 80,
        CountryTerrainKind::Jungle => 90,
        CountryTerrainKind::Swamp => 120,
        CountryTerrainKind::Mountains => 120,
        CountryTerrainKind::Pass => 90,
        CountryTerrainKind::River => 90,
        CountryTerrainKind::Tundra => 90,
        CountryTerrainKind::Desert => 90,
        CountryTerrainKind::ChaosSea => 150,
        _ => 60,
    }
}

fn terrain_search_minutes(terrain: CountryTerrainKind) -> u64 {
    match terrain {
        CountryTerrainKind::Forest | CountryTerrainKind::Jungle => 80,
        CountryTerrainKind::Swamp | CountryTerrainKind::Mountains => 90,
        _ => 60,
    }
}

fn terrain_hunt_minutes(terrain: CountryTerrainKind) -> u64 {
    match terrain {
        CountryTerrainKind::Forest | CountryTerrainKind::Jungle => 210,
        CountryTerrainKind::Swamp | CountryTerrainKind::Mountains => 240,
        CountryTerrainKind::Desert | CountryTerrainKind::Tundra => 210,
        _ => 180,
    }
}

fn spawn_countryside_encounter<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
    terrain: CountryTerrainKind,
) -> bool {
    if state.world_mode != WorldMode::Countryside
        || state.environment != LegacyEnvironment::Countryside
        || state.map_binding.semantic != MapSemanticKind::Country
    {
        return false;
    }
    if state.country_grid.cells.is_empty() {
        return false;
    }
    if !state.monsters.is_empty() {
        return false;
    }
    let current_terrain = state
        .country_cell_at(state.player.position)
        .map(|cell| cell.current_terrain)
        .or_else(|| {
            fallback_country_cell_from_rows(state, state.player.position).map(|c| c.current_terrain)
        })
        .unwrap_or(terrain);
    if current_terrain != terrain && terrain != CountryTerrainKind::Unknown {
        return false;
    }
    let terrain = current_terrain;
    if matches!(
        terrain,
        CountryTerrainKind::City
            | CountryTerrainKind::Village
            | CountryTerrainKind::Temple
            | CountryTerrainKind::Castle
            | CountryTerrainKind::Palace
            | CountryTerrainKind::StarPeak
            | CountryTerrainKind::DragonLair
            | CountryTerrainKind::MagicIsle
            | CountryTerrainKind::Volcano
            | CountryTerrainKind::Caves
    ) {
        return false;
    }

    let encounter_threshold = match terrain {
        CountryTerrainKind::Road => 5,
        CountryTerrainKind::Plains => 12,
        CountryTerrainKind::Forest => 18,
        CountryTerrainKind::Jungle => 20,
        CountryTerrainKind::Swamp => 22,
        CountryTerrainKind::Mountains => 18,
        CountryTerrainKind::Pass => 15,
        CountryTerrainKind::River => 14,
        CountryTerrainKind::Tundra => 16,
        CountryTerrainKind::Desert => 16,
        CountryTerrainKind::ChaosSea => 24,
        _ => 10,
    };

    let roll = rng.range_inclusive_i32(1, 100);
    if roll > encounter_threshold {
        return false;
    }

    let Some(spawn_pos) = find_adjacent_spawn(state) else {
        return false;
    };
    let monster_name = pick_encounter_monster_name(state, rng, terrain);
    let stats = match terrain {
        CountryTerrainKind::Road | CountryTerrainKind::Plains => {
            Stats { hp: 12, max_hp: 12, attack_min: 2, attack_max: 5, defense: 1 }
        }
        CountryTerrainKind::Forest | CountryTerrainKind::Jungle => {
            Stats { hp: 14, max_hp: 14, attack_min: 3, attack_max: 6, defense: 2 }
        }
        CountryTerrainKind::Swamp | CountryTerrainKind::ChaosSea => {
            Stats { hp: 16, max_hp: 16, attack_min: 3, attack_max: 7, defense: 2 }
        }
        CountryTerrainKind::Mountains
        | CountryTerrainKind::Pass
        | CountryTerrainKind::Tundra
        | CountryTerrainKind::Desert => {
            Stats { hp: 18, max_hp: 18, attack_min: 4, attack_max: 8, defense: 3 }
        }
        _ => Stats { hp: 12, max_hp: 12, attack_min: 2, attack_max: 5, defense: 1 },
    };
    state.spawn_monster(monster_name, spawn_pos, stats);
    state.log.push(format!("A wandering threat emerges from the countryside ({terrain:?})."));
    events.push(Event::LegacyHandled {
        token: "encounter".to_string(),
        note: "countryside encounter spawned".to_string(),
        fully_modeled: true,
    });
    true
}

fn is_passive_monster_name(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("sheep")
        || lowered.contains("goat")
        || lowered.contains("cow")
        || lowered.contains("horse")
        || lowered.contains("deer")
        || lowered.contains("rabbit")
}

fn terrain_encounter_pool(terrain: CountryTerrainKind) -> &'static [&'static str] {
    match terrain {
        CountryTerrainKind::Road | CountryTerrainKind::Plains => &["bandit", "wolf", "goblin"],
        CountryTerrainKind::Forest => &["wolf", "boar", "stalker"],
        CountryTerrainKind::Jungle => &["serpent", "jaguar", "stalker"],
        CountryTerrainKind::Swamp => &["serpent", "alligator", "bog fiend"],
        CountryTerrainKind::Mountains | CountryTerrainKind::Pass => {
            &["raider", "stalker", "goblin"]
        }
        CountryTerrainKind::River => &["raider", "serpent", "bandit"],
        CountryTerrainKind::Tundra => &["raider", "wolf", "stalker"],
        CountryTerrainKind::Desert => &["raider", "viper", "scorpion"],
        CountryTerrainKind::ChaosSea => &["chaos spawn", "daemon", "sea raider"],
        _ => &["bandit", "wolf", "stalker"],
    }
}

fn pick_encounter_monster_name<R: RandomSource>(
    state: &GameState,
    rng: &mut R,
    terrain: CountryTerrainKind,
) -> String {
    let pool = terrain_encounter_pool(terrain);
    if !state.encounter_monsters.is_empty() {
        let filtered = state
            .encounter_monsters
            .iter()
            .filter(|name| !is_passive_monster_name(name))
            .collect::<Vec<_>>();
        if !filtered.is_empty() {
            let idx = rng.range_inclusive_i32(0, (filtered.len() - 1) as i32);
            return filtered[idx as usize].clone();
        }
    }
    let idx = rng.range_inclusive_i32(0, (pool.len() - 1) as i32);
    pool[idx as usize].to_string()
}

fn find_adjacent_spawn(state: &GameState) -> Option<Position> {
    let player = state.player.position;
    let candidates = [
        Position { x: player.x + 1, y: player.y },
        Position { x: player.x - 1, y: player.y },
        Position { x: player.x, y: player.y + 1 },
        Position { x: player.x, y: player.y - 1 },
        Position { x: player.x + 1, y: player.y + 1 },
        Position { x: player.x - 1, y: player.y + 1 },
        Position { x: player.x + 1, y: player.y - 1 },
        Position { x: player.x - 1, y: player.y - 1 },
    ];
    candidates.into_iter().find(|pos| state.bounds.contains(*pos) && state.tile_is_walkable(*pos))
}

fn village_map_for_aux(aux: u8) -> Option<(u16, Position, &'static str)> {
    match aux {
        1 => Some((14, Position { x: 0, y: 6 }, "starview")),
        2 => Some((19, Position { x: 39, y: 15 }, "woodmere")),
        3 => Some((15, Position { x: 63, y: 8 }, "stormwat")),
        4 => Some((17, Position { x: 32, y: 15 }, "thaumari")),
        5 => Some((12, Position { x: 2, y: 8 }, "skorch")),
        6 => Some((18, Position { x: 2, y: 2 }, "whorfen")),
        _ => None,
    }
}

fn sanitize_spawn(state: &GameState, requested: Position) -> Option<Position> {
    let bounded = Position {
        x: requested.x.clamp(0, state.bounds.width.saturating_sub(1)),
        y: requested.y.clamp(0, state.bounds.height.saturating_sub(1)),
    };
    if state.tile_is_walkable(bounded) {
        return Some(bounded);
    }
    first_walkable_position(state)
}

fn first_walkable_position(state: &GameState) -> Option<Position> {
    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            let pos = Position { x, y };
            if state.tile_is_walkable(pos) {
                return Some(pos);
            }
        }
    }
    None
}

fn requires_confirmation(token: &str) -> bool {
    matches!(token, "p" | "z" | "Z" | "S")
}

fn ensure_known_site(state: &mut GameState, pos: Position) {
    if !state.known_sites.iter().any(|candidate| *candidate == pos) {
        state.known_sites.push(pos);
    }
}

fn rotate_combat_sequence(state: &mut GameState) {
    let preset = (state.combat_sequence_cursor + 1) % 3;
    state.combat_sequence = match preset {
        0 => vec![CombatStep { maneuver: CombatManeuver::Attack, line: CombatLine::Center }],
        1 => vec![
            CombatStep { maneuver: CombatManeuver::Lunge, line: CombatLine::High },
            CombatStep { maneuver: CombatManeuver::Attack, line: CombatLine::Center },
        ],
        _ => vec![
            CombatStep { maneuver: CombatManeuver::Block, line: CombatLine::High },
            CombatStep { maneuver: CombatManeuver::Riposte, line: CombatLine::Low },
        ],
    };
    state.combat_sequence_cursor = 0;
}

fn cycle_runtime_options(state: &mut GameState) {
    state.options.belligerent = !state.options.belligerent;
    state.options.jumpmove = !state.options.jumpmove;
    state.options.pickup = !state.options.pickup;
    state.options.confirm = !state.options.confirm;
    state.options.topinv = !state.options.topinv;
    state.options.packadd = !state.options.packadd;
    state.options.searchnum =
        if state.options.searchnum >= 5 { 1 } else { state.options.searchnum + 1 };
    state.options.verbosity = match state.options.verbosity {
        LegacyVerbosity::Terse => LegacyVerbosity::Medium,
        LegacyVerbosity::Medium => LegacyVerbosity::Verbose,
        LegacyVerbosity::Verbose => LegacyVerbosity::Terse,
    };
}

fn has_legacy_status_flag(state: &GameState, bit: u64) -> bool {
    (state.legacy_status_flags & bit) != 0
}

fn set_legacy_status_flag(state: &mut GameState, bit: u64) {
    state.legacy_status_flags |= bit;
}

fn clear_legacy_status_flag(state: &mut GameState, bit: u64) {
    state.legacy_status_flags &= !bit;
}

fn sync_wizard_flag_with_legacy_bits(state: &mut GameState) {
    if state.wizard.enabled {
        set_legacy_status_flag(state, LEGACY_STATUS_CHEATED);
        state.wizard.scoring_allowed = false;
    }
    if has_legacy_status_flag(state, LEGACY_STATUS_CHEATED) {
        state.wizard.enabled = true;
        state.wizard.scoring_allowed = false;
    }
    if state.wizard.enabled {
        state.progression.high_score_eligible = false;
    }
}

fn apply_destructive_action(state: &mut GameState) -> (String, bool) {
    state.legal_heat += 1;
    state.progression.law_chaos_score -= 1;
    ("destructive action resolved with legal penalty".to_string(), true)
}

fn reveal_map_for_wizard(state: &mut GameState) {
    // Mirror classic wizard reveal: mark all tiles in the active map as discovered.
    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            ensure_known_site(state, Position { x, y });
        }
    }
    if state.environment == LegacyEnvironment::City {
        // Ensure every mapped city service destination is considered discovered for travel helpers.
        for y in 0..state.bounds.height {
            for x in 0..state.bounds.width {
                let pos = Position { x, y };
                let Some(idx) = tile_index(state.bounds, pos) else {
                    continue;
                };
                let aux = match state.map_binding.semantic {
                    MapSemanticKind::City => state.city_site_grid.get(idx).map(|cell| cell.aux),
                    MapSemanticKind::Country => {
                        state.country_site_grid.get(idx).map(|cell| cell.aux)
                    }
                    _ => state.site_grid.get(idx).map(|cell| cell.aux),
                }
                .unwrap_or(0);
                if aux != 0 {
                    ensure_known_site(state, pos);
                }
            }
        }
    }
}

fn stat_slot_name(slot: u8) -> &'static str {
    match slot {
        1 => "Strength",
        2 => "Constitution",
        3 => "Dexterity",
        4 => "Agility",
        5 => "IQ",
        6 => "Power",
        7 => "HP",
        8 => "Max HP",
        9 => "Mana",
        10 => "Max Mana",
        11 => "Gold",
        _ => "Unknown",
    }
}

fn stat_slot_value(state: &GameState, slot: u8) -> i32 {
    match slot {
        1 => state.attributes.strength,
        2 => state.attributes.constitution,
        3 => state.attributes.dexterity,
        4 => state.attributes.agility,
        5 => state.attributes.iq,
        6 => state.attributes.power,
        7 => state.player.stats.hp,
        8 => state.player.stats.max_hp,
        9 => state.spellbook.mana,
        10 => state.spellbook.max_mana,
        11 => state.gold,
        _ => 0,
    }
}

fn apply_stat_slot_value(state: &mut GameState, slot: u8, raw_value: i32) -> String {
    match slot {
        1 => {
            state.attributes.strength = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("Strength set to {}", state.attributes.strength)
        }
        2 => {
            state.attributes.constitution = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("Constitution set to {}", state.attributes.constitution)
        }
        3 => {
            state.attributes.dexterity = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("Dexterity set to {}", state.attributes.dexterity)
        }
        4 => {
            state.attributes.agility = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("Agility set to {}", state.attributes.agility)
        }
        5 => {
            state.attributes.iq = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("IQ set to {}", state.attributes.iq)
        }
        6 => {
            state.attributes.power = raw_value.clamp(1, 32);
            recompute_derived_combat_and_mana_from_attributes(state);
            format!("Power set to {}", state.attributes.power)
        }
        7 => {
            state.player.stats.hp = raw_value.clamp(0, state.player.stats.max_hp.max(1));
            format!("HP set to {}", state.player.stats.hp)
        }
        8 => {
            state.player.stats.max_hp = raw_value.clamp(1, 120);
            state.player.stats.hp = state.player.stats.hp.clamp(0, state.player.stats.max_hp);
            format!("Max HP set to {}", state.player.stats.max_hp)
        }
        9 => {
            state.spellbook.mana = raw_value.clamp(0, state.spellbook.max_mana.max(0));
            format!("Mana set to {}", state.spellbook.mana)
        }
        10 => {
            state.spellbook.max_mana = raw_value.clamp(1, 600);
            state.spellbook.mana = state.spellbook.mana.clamp(0, state.spellbook.max_mana);
            format!("Max Mana set to {}", state.spellbook.max_mana)
        }
        11 => {
            state.gold = raw_value.clamp(0, 1_000_000);
            format!("Gold set to {}", state.gold)
        }
        _ => "Invalid stat slot.".to_string(),
    }
}

const LEGACY_WISH_IINIT_H: &str =
    include_str!("../../../archive/legacy-c-runtime/2026-02-06/iinit.h");
const LEGACY_PROJECTILE_DEFS_H: &str =
    include_str!("../../../archive/legacy-c-runtime/2026-02-06/defs.h");
const LEGACY_PROJECTILE_COMMAND3_C: &str =
    include_str!("../../../archive/legacy-c-runtime/2026-02-06/command3.c");
const LEGACY_PROJECTILE_ITEMF2_C: &str =
    include_str!("../../../archive/legacy-c-runtime/2026-02-06/itemf2.c");
const LEGACY_PROJECTILE_UTIL_C: &str =
    include_str!("../../../archive/legacy-c-runtime/2026-02-06/util.c");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectileContract {
    pub ob_longbow: i32,
    pub ob_crossbow: i32,
    pub ob_arrow: i32,
    pub ob_bolt: i32,
    pub i_arrow: i32,
    pub i_bolt: i32,
    pub i_scythe: i32,
    pub loaded: i32,
    pub unloaded: i32,
    pub hit_rule: String,
    pub statmod_rule: String,
    pub references: Vec<String>,
}

pub fn legacy_projectile_contract() -> &'static ProjectileContract {
    static CONTRACT: OnceLock<ProjectileContract> = OnceLock::new();
    CONTRACT.get_or_init(parse_legacy_projectile_contract)
}

fn parse_legacy_projectile_contract() -> ProjectileContract {
    fn parse_define(name: &str) -> Option<i32> {
        let pattern = format!("#define {name} ");
        for line in LEGACY_PROJECTILE_DEFS_H.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with(&pattern) {
                continue;
            }
            let raw_value = trimmed.trim_start_matches(&pattern).trim();
            if let Ok(value) = raw_value.parse::<i32>() {
                return Some(value);
            }
            if let Some((lhs, rhs)) = raw_value.split_once('+')
                && let Ok(offset) = rhs.trim().trim_matches(')').parse::<i32>()
            {
                let base = match lhs.trim().trim_matches('(') {
                    "WEAPONID" => 89,
                    "THINGID" => 0,
                    _ => continue,
                };
                return Some(base + offset);
            }
        }
        None
    }

    ProjectileContract {
        ob_longbow: parse_define("OB_LONGBOW").unwrap_or(115),
        ob_crossbow: parse_define("OB_CROSSBOW").unwrap_or(116),
        ob_arrow: parse_define("OB_ARROW").unwrap_or(117),
        ob_bolt: parse_define("OB_BOLT").unwrap_or(118),
        i_arrow: parse_define("I_ARROW").unwrap_or(1006),
        i_bolt: parse_define("I_BOLT").unwrap_or(1007),
        i_scythe: parse_define("I_SCYTHE").unwrap_or(1014),
        loaded: parse_define("LOADED").unwrap_or(1),
        unloaded: parse_define("UNLOADED").unwrap_or(0),
        hit_rule: "natural 0 hit, natural 19 miss, else roll < (hit - ac)".to_string(),
        statmod_rule: "(stat - 10) / 2".to_string(),
        references: vec![
            "command3.c:fire/setspot/do_object_los".to_string(),
            "itemf2.c:weapon_arrow/weapon_bolt".to_string(),
            "util.c:hitp/unblocked/do_object_los".to_string(),
            format!("command3.c bytes={}", LEGACY_PROJECTILE_COMMAND3_C.len()),
            format!("itemf2.c bytes={}", LEGACY_PROJECTILE_ITEMF2_C.len()),
            format!("util.c bytes={}", LEGACY_PROJECTILE_UTIL_C.len()),
        ],
    }
}

#[derive(Debug, Clone)]
struct LegacyItemTemplate {
    legacy_id: i32,
    family: ItemFamily,
    usef: String,
    item_type: String,
    weight: i32,
    plus: i32,
    charge: i32,
    dmg: i32,
    hit: i32,
    aux: i32,
    number: i32,
    fragility: i32,
    basevalue: i64,
    known: bool,
    used: bool,
    blessing: i32,
    level: u8,
    uniqueness: String,
    objchar: String,
    objstr: String,
    truename: String,
    cursestr: String,
    normalized_names: Vec<String>,
}

const WISH_ITEM_KINDS: [WishItemKind; 12] = [
    WishItemKind::Potion,
    WishItemKind::Scroll,
    WishItemKind::Armor,
    WishItemKind::Shield,
    WishItemKind::Cloak,
    WishItemKind::Boots,
    WishItemKind::Weapon,
    WishItemKind::Stick,
    WishItemKind::Ring,
    WishItemKind::Food,
    WishItemKind::Thing,
    WishItemKind::Artifact,
];

const WISH_ITEM_KINDS_NON_ARTIFACT: [WishItemKind; 11] = [
    WishItemKind::Potion,
    WishItemKind::Scroll,
    WishItemKind::Armor,
    WishItemKind::Shield,
    WishItemKind::Cloak,
    WishItemKind::Boots,
    WishItemKind::Weapon,
    WishItemKind::Stick,
    WishItemKind::Ring,
    WishItemKind::Food,
    WishItemKind::Thing,
];

#[derive(Debug, Clone)]
struct WishResolution {
    note: String,
    committed: bool,
}

#[derive(Debug, Clone)]
struct WishCatalogEntry {
    name: String,
    normalized: String,
}

#[derive(Debug, Clone, Default)]
struct WishItemCatalog {
    potions: Vec<WishCatalogEntry>,
    scrolls: Vec<WishCatalogEntry>,
    rings: Vec<WishCatalogEntry>,
    sticks: Vec<WishCatalogEntry>,
    armor: Vec<WishCatalogEntry>,
    shields: Vec<WishCatalogEntry>,
    weapons: Vec<WishCatalogEntry>,
    boots: Vec<WishCatalogEntry>,
    cloaks: Vec<WishCatalogEntry>,
    foods: Vec<WishCatalogEntry>,
    things: Vec<WishCatalogEntry>,
    artifacts: Vec<WishCatalogEntry>,
}

impl WishItemCatalog {
    fn items_for_kind(&self, kind: WishItemKind) -> &[WishCatalogEntry] {
        match kind {
            WishItemKind::Potion => &self.potions,
            WishItemKind::Scroll => &self.scrolls,
            WishItemKind::Ring => &self.rings,
            WishItemKind::Stick => &self.sticks,
            WishItemKind::Armor => &self.armor,
            WishItemKind::Shield => &self.shields,
            WishItemKind::Weapon => &self.weapons,
            WishItemKind::Boots => &self.boots,
            WishItemKind::Cloak => &self.cloaks,
            WishItemKind::Food => &self.foods,
            WishItemKind::Thing => &self.things,
            WishItemKind::Artifact => &self.artifacts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WishHintMatch {
    Unique { kind: WishItemKind, name: String },
    Ambiguous,
    None,
}

fn wish_item_kind_label(kind: WishItemKind) -> &'static str {
    match kind {
        WishItemKind::Potion => "potions",
        WishItemKind::Scroll => "scrolls",
        WishItemKind::Ring => "rings",
        WishItemKind::Stick => "sticks",
        WishItemKind::Armor => "armor",
        WishItemKind::Shield => "shields",
        WishItemKind::Weapon => "weapons",
        WishItemKind::Boots => "boots",
        WishItemKind::Cloak => "cloaks",
        WishItemKind::Food => "food",
        WishItemKind::Thing => "things",
        WishItemKind::Artifact => "artifacts",
    }
}

fn resolve_wish_request(
    state: &mut GameState,
    events: &mut Vec<Event>,
    blessing: i8,
    raw_wish: &str,
) -> WishResolution {
    state.pending_wizard_interaction = None;
    let intent = parse_wish_intent_lenient(raw_wish);
    match intent {
        WishIntent::Acquisition { item_hint } => resolve_wish_acquisition(state, events, item_hint),
        WishIntent::Unknown => {
            if let Some(item_hint) = infer_unknown_wish_item_hint(state, raw_wish) {
                resolve_wish_acquisition(state, events, Some(item_hint))
            } else {
                WishResolution { note: "You feel stupid.".to_string(), committed: true }
            }
        }
        _ => WishResolution {
            note: apply_wish_intent(state, intent, events, blessing),
            committed: true,
        },
    }
}

fn parse_wish_intent_lenient(raw: &str) -> WishIntent {
    let lowered = raw.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return WishIntent::Unknown;
    }
    if lowered.contains("death") {
        return WishIntent::Death;
    }
    if lowered.contains("power") {
        return WishIntent::Power;
    }
    if lowered.contains("skill") {
        return WishIntent::Skill;
    }
    if lowered.contains("wealth") || lowered.contains("gold") || lowered.contains("money") {
        return WishIntent::Wealth;
    }
    if lowered.contains("balance") || lowered.contains("neutral") {
        return WishIntent::Balance;
    }
    if lowered.contains("chaos") {
        return WishIntent::Chaos;
    }
    if lowered.contains("law") {
        return WishIntent::Law;
    }
    if lowered.contains("location") || lowered.contains("teleport") || lowered.contains("where") {
        return WishIntent::Location;
    }
    if lowered.contains("knowledge") || lowered.contains("lore") {
        return WishIntent::Knowledge;
    }
    if lowered.contains("health") || lowered.contains("heal") {
        return WishIntent::Health;
    }
    if lowered.contains("destruction") || lowered.contains("destroy") {
        return WishIntent::Destruction;
    }
    if lowered.contains("summoning") || lowered.contains("summon") {
        return WishIntent::Summoning;
    }
    if lowered.contains("stats") || lowered.contains("stat") {
        return WishIntent::Stats;
    }
    if lowered.contains("acquisition")
        || lowered.contains("acquire")
        || lowered.contains("item")
        || lowered.contains("artifact")
        || lowered.contains("loot")
        || lowered.contains("gear")
        || resolve_item_kind_from_alias(&lowered).is_some()
    {
        return WishIntent::Acquisition { item_hint: parse_wish_item_hint(raw) };
    }
    WishIntent::Unknown
}

fn parse_wish_item_hint(raw: &str) -> Option<String> {
    let lowered = raw.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return None;
    }
    let mut hint = lowered.as_str();
    for prefix in ["wish for ", "wish ", "get ", "acquire ", "need ", "want "] {
        if let Some(rest) = hint.strip_prefix(prefix) {
            hint = rest;
            break;
        }
    }
    let hint = hint.trim();
    if hint.is_empty() {
        return None;
    }
    let normalized = normalize_item_lookup(hint);
    if normalized.is_empty() {
        return None;
    }
    if matches!(
        normalized.as_str(),
        "acquisition"
            | "item"
            | "items"
            | "get item"
            | "get items"
            | "artifact"
            | "artifacts"
            | "loot"
            | "gear"
            | "stuff"
            | "something"
    ) {
        return None;
    }
    if resolve_item_kind_from_alias(&normalized).is_some()
        && normalized.split_whitespace().count() == 1
    {
        return None;
    }
    Some(hint.to_string())
}

fn infer_unknown_wish_item_hint(state: &GameState, raw_wish: &str) -> Option<String> {
    let trimmed = raw_wish.trim();
    if trimmed.is_empty() {
        return None;
    }
    let hint = parse_wish_item_hint(raw_wish).unwrap_or_else(|| trimmed.to_string());
    let cheated = state.wizard.enabled || has_legacy_status_flag(state, LEGACY_STATUS_CHEATED);

    if resolve_item_kind_from_alias(&hint).is_some() {
        return Some(hint);
    }
    match resolve_item_by_hint(&hint, cheated) {
        WishHintMatch::Unique { .. } | WishHintMatch::Ambiguous => Some(hint),
        WishHintMatch::None => None,
    }
}

fn apply_wish_intent(
    state: &mut GameState,
    intent: WishIntent,
    events: &mut Vec<Event>,
    blessing: i8,
) -> String {
    match intent {
        WishIntent::Death => {
            state.player.stats.hp = 0;
            state.status = SessionStatus::Lost;
            state.death_source = Some("a deathwish".to_string());
            state.log.push("As you wish, so shall it be.".to_string());
            events.push(Event::PlayerDefeated);
            "As you wish, so shall it be.".to_string()
        }
        WishIntent::Power => {
            state.spellbook.mana = state.spellbook.max_mana;
            "You feel a sudden surge of energy.".to_string()
        }
        WishIntent::Skill => {
            if state.wizard.enabled {
                state.player.stats.attack_min = (state.player.stats.attack_min + 2).clamp(1, 24);
                state.player.stats.attack_max = (state.player.stats.attack_max + 3)
                    .clamp(state.player.stats.attack_min + 1, 32);
            } else {
                state.player.stats.attack_max = (state.player.stats.attack_max + 1)
                    .clamp(state.player.stats.attack_min + 1, 24);
            }
            "You feel more competent.".to_string()
        }
        WishIntent::Wealth => {
            state.gold = state.gold.saturating_add(10_000);
            events.push(Event::EconomyUpdated {
                source: "wish".to_string(),
                gold: state.gold,
                bank_gold: state.bank_gold,
            });
            "You are submerged in a shower of gold pieces.".to_string()
        }
        WishIntent::Balance => {
            state.progression.alignment = Alignment::Neutral;
            state.progression.law_chaos_score = 0;
            "You feel neutral.".to_string()
        }
        WishIntent::Chaos => {
            state.progression.alignment = Alignment::Chaotic;
            state.progression.law_chaos_score -= 25;
            "You feel chaotic.".to_string()
        }
        WishIntent::Law => {
            state.progression.alignment = Alignment::Lawful;
            state.progression.law_chaos_score += 25;
            "You feel lawful.".to_string()
        }
        WishIntent::Location => {
            if state.world_mode != WorldMode::Countryside {
                ensure_country_bootstrap(state);
                state.activate_country_view();
            }
            let target = state
                .topology
                .country_rampart_position
                .unwrap_or(Position { x: state.bounds.width / 2, y: state.bounds.height / 2 });
            if state.tile_is_walkable(target) {
                state.player.position = target;
            }
            "Magic portals open and cast you toward a known destination.".to_string()
        }
        WishIntent::Knowledge => {
            state.spellbook.mana = (state.spellbook.mana + 20).min(state.spellbook.max_mana);
            let learned = teach_first_unknown_from_pool(state, &[20, 22, 36, 40, 29]);
            if let Some(spell_id) = learned {
                format!(
                    "You feel more knowledgeable. Spell learned: {}.",
                    spell_name_by_id(spell_id)
                )
            } else {
                "You feel more knowledgeable.".to_string()
            }
        }
        WishIntent::Health => {
            state.player.stats.hp = state.player.stats.max_hp;
            consume_status(state, "poison");
            state.food = state.food.max(43);
            "You feel vigorous.".to_string()
        }
        WishIntent::Destruction => {
            let defeated = state.monsters.len() as u64;
            for monster in &state.monsters {
                events.push(Event::MonsterDefeated { monster_id: monster.id });
            }
            state.monsters.clear();
            state.monsters_defeated = state.monsters_defeated.saturating_add(defeated);
            if blessing < 0 {
                state.player.stats.hp = 0;
                state.status = SessionStatus::Lost;
                state.death_source = Some("a cursed wish".to_string());
                events.push(Event::PlayerDefeated);
            }
            format!("Annihilation erupts. {} hostiles destroyed.", defeated)
        }
        WishIntent::Summoning => {
            let spawn = [
                Position { x: state.player.position.x + 1, y: state.player.position.y },
                Position { x: state.player.position.x - 1, y: state.player.position.y },
                Position { x: state.player.position.x, y: state.player.position.y + 1 },
                Position { x: state.player.position.x, y: state.player.position.y - 1 },
            ]
            .into_iter()
            .find(|candidate| {
                state.bounds.contains(*candidate) && state.tile_is_walkable(*candidate)
            });
            if let Some(position) = spawn {
                state.spawn_monster(
                    if state.wizard.enabled { "summoned champion" } else { "summoned horror" },
                    position,
                    Stats {
                        hp: if state.wizard.enabled { 18 } else { 12 },
                        max_hp: if state.wizard.enabled { 18 } else { 12 },
                        attack_min: 2,
                        attack_max: 6,
                        defense: 1,
                    },
                );
                "A being answers your call.".to_string()
            } else {
                "No space answers the summoning.".to_string()
            }
        }
        WishIntent::Stats => {
            if state.wizard.enabled {
                state.attributes = PrimaryAttributes {
                    strength: 32,
                    constitution: 32,
                    dexterity: 32,
                    agility: 32,
                    iq: 32,
                    power: 32,
                };
                recompute_derived_combat_and_mana_from_attributes(state);
                state.player.stats.hp = state.player.stats.max_hp;
                state.spellbook.mana = state.spellbook.max_mana;
                "All primary attributes surge to superhuman levels.".to_string()
            } else {
                "You feel stupid.".to_string()
            }
        }
        WishIntent::Acquisition { .. } | WishIntent::Unknown => "You feel stupid.".to_string(),
    }
}

fn resolve_wish_acquisition(
    state: &mut GameState,
    events: &mut Vec<Event>,
    item_hint: Option<String>,
) -> WishResolution {
    let cheated = state.wizard.enabled || has_legacy_status_flag(state, LEGACY_STATUS_CHEATED);

    if let Some(hint) = item_hint.as_ref() {
        if !cheated {
            let artifact_hint = resolve_item_kind_from_alias(hint) == Some(WishItemKind::Artifact)
                || matches!(
                    resolve_item_by_hint(hint, true),
                    WishHintMatch::Unique { kind: WishItemKind::Artifact, .. }
                );
            if artifact_hint {
                state.pending_wizard_interaction = None;
                return WishResolution { note: "You feel stupid.".to_string(), committed: true };
            }
        }

        if let WishHintMatch::Unique { kind: _, name } = resolve_item_by_hint(hint, cheated) {
            let result = add_item_to_inventory_or_ground(state, name, events);
            state.pending_wizard_interaction = None;
            state.wizard_input_buffer.clear();
            return WishResolution {
                note: format!("Acquisition resolved ({result})."),
                committed: true,
            };
        }
    }

    let interaction = WizardInteraction::WishAcquisitionKindSelect { cheated, item_hint };
    state.pending_wizard_interaction = Some(interaction.clone());
    state.wizard_input_buffer.clear();
    WishResolution { note: wizard_interaction_prompt(state, &interaction), committed: false }
}

fn resolve_item_by_hint(hint: &str, cheated: bool) -> WishHintMatch {
    let normalized_hint = normalize_item_lookup(hint);
    if normalized_hint.is_empty() {
        return WishHintMatch::None;
    }
    let catalog = legacy_wish_item_catalog();
    let kinds = if cheated { &WISH_ITEM_KINDS[..] } else { &WISH_ITEM_KINDS_NON_ARTIFACT[..] };

    let mut exact: Vec<(WishItemKind, String)> = Vec::new();
    let mut fuzzy: Vec<(WishItemKind, String)> = Vec::new();

    for kind in kinds {
        for entry in catalog.items_for_kind(*kind) {
            if entry.normalized == normalized_hint {
                exact.push((*kind, entry.name.clone()));
            } else if entry.normalized.contains(&normalized_hint)
                || normalized_hint.contains(&entry.normalized)
            {
                fuzzy.push((*kind, entry.name.clone()));
            }
        }
    }

    let mut exact_unique: Vec<(WishItemKind, String)> = Vec::new();
    for (kind, name) in exact {
        if !exact_unique.iter().any(|(_, existing)| existing.eq_ignore_ascii_case(&name)) {
            exact_unique.push((kind, name));
        }
    }

    if exact_unique.len() == 1 {
        let (kind, name) = exact_unique.remove(0);
        return WishHintMatch::Unique { kind, name };
    }
    if exact_unique.len() > 1 {
        return WishHintMatch::Ambiguous;
    }
    let mut fuzzy_unique: Vec<(WishItemKind, String)> = Vec::new();
    for (kind, name) in fuzzy {
        if !fuzzy_unique.iter().any(|(_, existing)| existing.eq_ignore_ascii_case(&name)) {
            fuzzy_unique.push((kind, name));
        }
    }
    if fuzzy_unique.len() == 1 {
        let (kind, name) = fuzzy_unique.remove(0);
        return WishHintMatch::Unique { kind, name };
    }
    if fuzzy_unique.len() > 1 {
        return WishHintMatch::Ambiguous;
    }
    WishHintMatch::None
}

fn resolve_item_kind_from_choice_token(token: &str) -> Option<WishItemKind> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(kind) = resolve_item_kind_from_alias(trimmed) {
        return Some(kind);
    }
    let ch = trimmed.chars().next()?.to_ascii_lowercase();
    match ch {
        '!' | '1' => Some(WishItemKind::Potion),
        '?' | '2' => Some(WishItemKind::Scroll),
        ']' | '3' => Some(WishItemKind::Armor),
        '[' | '4' => Some(WishItemKind::Shield),
        '}' | '5' => Some(WishItemKind::Cloak),
        '{' | '6' => Some(WishItemKind::Boots),
        ')' | '7' => Some(WishItemKind::Weapon),
        '/' | '8' => Some(WishItemKind::Stick),
        '=' | '9' => Some(WishItemKind::Ring),
        '%' | '0' => Some(WishItemKind::Food),
        '\\' => Some(WishItemKind::Thing),
        '&' => Some(WishItemKind::Artifact),
        _ => None,
    }
}

fn resolve_item_kind_from_alias(alias: &str) -> Option<WishItemKind> {
    let normalized = normalize_item_lookup(alias);
    let compact = normalized.replace(' ', "");
    if normalized.contains("artifact") || compact == "art" {
        return Some(WishItemKind::Artifact);
    }
    if normalized.contains("potion") {
        return Some(WishItemKind::Potion);
    }
    if normalized.contains("scroll") {
        return Some(WishItemKind::Scroll);
    }
    if normalized.contains("ring") {
        return Some(WishItemKind::Ring);
    }
    if normalized.contains("stick") || normalized.contains("staff") || normalized.contains("wand") {
        return Some(WishItemKind::Stick);
    }
    if normalized.contains("armor") || normalized.contains("armour") {
        return Some(WishItemKind::Armor);
    }
    if normalized.contains("shield") {
        return Some(WishItemKind::Shield);
    }
    if normalized.contains("weapon")
        || normalized.contains("sword")
        || normalized.contains("axe")
        || normalized.contains("bow")
    {
        return Some(WishItemKind::Weapon);
    }
    if normalized.contains("boot") {
        return Some(WishItemKind::Boots);
    }
    if normalized.contains("cloak") {
        return Some(WishItemKind::Cloak);
    }
    if normalized.contains("food") || normalized.contains("ration") {
        return Some(WishItemKind::Food);
    }
    if normalized.contains("thing") || normalized.contains("tool") || normalized.contains("misc") {
        return Some(WishItemKind::Thing);
    }
    None
}

fn resolve_item_selection_by_number_or_name(kind: WishItemKind, request: &str) -> Option<String> {
    let catalog = legacy_wish_item_catalog();
    let entries = catalog.items_for_kind(kind);
    if entries.is_empty() {
        return None;
    }
    if let Ok(index) = request.trim().parse::<usize>()
        && (1..=entries.len()).contains(&index)
    {
        return Some(entries[index - 1].name.clone());
    }
    let normalized = normalize_item_lookup(request);
    if normalized.is_empty() {
        return None;
    }
    let exact: Vec<&WishCatalogEntry> =
        entries.iter().filter(|entry| entry.normalized == normalized).collect();
    if exact.len() == 1 {
        return Some(exact[0].name.clone());
    }
    if exact.len() > 1 {
        return None;
    }
    let fuzzy: Vec<&WishCatalogEntry> = entries
        .iter()
        .filter(|entry| {
            entry.normalized.contains(&normalized) || normalized.contains(&entry.normalized)
        })
        .collect();
    if fuzzy.len() == 1 {
        return Some(fuzzy[0].name.clone());
    }
    None
}

fn random_item_from_kind(state: &GameState, kind: WishItemKind) -> Option<String> {
    let catalog = legacy_wish_item_catalog();
    let entries = catalog.items_for_kind(kind);
    if entries.is_empty() {
        return None;
    }
    let seed = state
        .next_item_id
        .wrapping_add(state.scheduler.player_phase as u32)
        .wrapping_add((state.clock.turn as u32).wrapping_mul(1_103_515_245))
        .wrapping_add((state.clock.minutes as u32).wrapping_mul(97));
    let idx = (seed as usize) % entries.len();
    Some(entries[idx].name.clone())
}

fn legacy_wish_item_catalog() -> &'static WishItemCatalog {
    static CATALOG: OnceLock<WishItemCatalog> = OnceLock::new();
    CATALOG.get_or_init(parse_wish_item_catalog)
}

fn parse_wish_item_catalog() -> WishItemCatalog {
    let mut catalog = WishItemCatalog::default();
    for template in legacy_item_templates() {
        let name = sanitize_catalog_name(&template.truename)
            .or_else(|| sanitize_catalog_name(&template.objstr))
            .unwrap_or_else(|| template.truename.clone());
        let entry = WishCatalogEntry { normalized: normalize_item_lookup(&name), name };
        if entry.normalized.is_empty() {
            continue;
        }
        match template.family {
            ItemFamily::Potion => catalog.potions.push(entry),
            ItemFamily::Scroll => catalog.scrolls.push(entry),
            ItemFamily::Ring => catalog.rings.push(entry),
            ItemFamily::Stick => catalog.sticks.push(entry),
            ItemFamily::Armor => catalog.armor.push(entry),
            ItemFamily::Shield => catalog.shields.push(entry),
            ItemFamily::Weapon => catalog.weapons.push(entry),
            ItemFamily::Boots => catalog.boots.push(entry),
            ItemFamily::Cloak => catalog.cloaks.push(entry),
            ItemFamily::Food => catalog.foods.push(entry),
            ItemFamily::Thing => catalog.things.push(entry),
            ItemFamily::Artifact => catalog.artifacts.push(entry),
            ItemFamily::Unknown | ItemFamily::Cash | ItemFamily::Corpse => {}
        }
    }
    catalog
}

fn legacy_item_templates() -> &'static Vec<LegacyItemTemplate> {
    static TEMPLATES: OnceLock<Vec<LegacyItemTemplate>> = OnceLock::new();
    TEMPLATES.get_or_init(parse_legacy_item_templates)
}

fn parse_legacy_item_templates() -> Vec<LegacyItemTemplate> {
    let mut templates = Vec::new();
    for line in LEGACY_WISH_IINIT_H.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('{') || !(trimmed.ends_with("},") || trimmed.ends_with('}')) {
            continue;
        }
        let body =
            trimmed.trim_start_matches('{').trim_end_matches(',').trim_end_matches('}').trim();
        let fields = split_top_level_csv(body);
        if fields.len() < 21 {
            continue;
        }
        let id_expr = fields[0].trim().replace(' ', "");
        let Some((family, family_index)) = parse_legacy_item_family_and_index(&id_expr) else {
            continue;
        };
        let legacy_id = legacy_item_family_base_id(family) + i32::from(family_index);
        let Some(weight) = parse_i32_token(&fields[1]) else {
            continue;
        };
        let Some(plus) = parse_i32_token(&fields[2]) else {
            continue;
        };
        let Some(charge) = parse_i32_token(&fields[3]) else {
            continue;
        };
        let Some(dmg) = parse_i32_token(&fields[4]) else {
            continue;
        };
        let Some(hit) = parse_i32_token(&fields[5]) else {
            continue;
        };
        let Some(aux) = parse_i32_token(&fields[6]) else {
            continue;
        };
        let Some(number) = parse_i32_token(&fields[7]) else {
            continue;
        };
        let Some(fragility) = parse_i32_token(&fields[8]) else {
            continue;
        };
        let Some(basevalue) = parse_i64_token(&fields[9]) else {
            continue;
        };
        let known = parse_u8_token(&fields[10]).unwrap_or(0) != 0;
        let used = parse_u8_token(&fields[11]).unwrap_or(0) != 0;
        let Some(blessing) = parse_i32_token(&fields[12]) else {
            continue;
        };
        let Some(level) = parse_u8_token(&fields[16]) else {
            continue;
        };
        let objstr = parse_string_token(&fields[18]).unwrap_or_default();
        let truename = parse_string_token(&fields[19]).unwrap_or_default();
        let cursestr = parse_string_token(&fields[20]).unwrap_or_default();
        let mut normalized_names = Vec::new();
        for candidate in [&objstr, &truename, &cursestr] {
            let normalized = normalize_item_lookup(candidate);
            if !normalized.is_empty() && !normalized_names.iter().any(|v| v == &normalized) {
                normalized_names.push(normalized);
            }
        }
        templates.push(LegacyItemTemplate {
            legacy_id,
            family,
            usef: fields[15].trim().to_string(),
            item_type: fields[13].trim().to_string(),
            weight,
            plus,
            charge,
            dmg,
            hit,
            aux,
            number,
            fragility,
            basevalue,
            known,
            used,
            blessing,
            level,
            uniqueness: fields[14].trim().to_string(),
            objchar: fields[17].trim().to_string(),
            objstr,
            truename,
            cursestr,
            normalized_names,
        });
    }
    templates
}

fn parse_legacy_item_family_and_index(expr: &str) -> Option<(ItemFamily, u16)> {
    let expr = expr.trim();
    if let Some(index) = expr.strip_prefix("THINGID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Thing, index));
    }
    if let Some(index) = expr.strip_prefix("FOODID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Food, index));
    }
    if let Some(index) = expr.strip_prefix("SCROLLID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Scroll, index));
    }
    if let Some(index) = expr.strip_prefix("POTIONID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Potion, index));
    }
    if let Some(index) = expr.strip_prefix("WEAPONID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Weapon, index));
    }
    if let Some(index) = expr.strip_prefix("ARMORID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Armor, index));
    }
    if let Some(index) = expr.strip_prefix("SHIELDID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Shield, index));
    }
    if let Some(index) = expr.strip_prefix("CLOAKID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Cloak, index));
    }
    if let Some(index) = expr.strip_prefix("BOOTID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Boots, index));
    }
    if let Some(index) = expr.strip_prefix("RINGID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Ring, index));
    }
    if let Some(index) = expr.strip_prefix("STICKID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Stick, index));
    }
    if let Some(index) = expr.strip_prefix("ARTIFACTID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((ItemFamily::Artifact, index));
    }
    if expr == "CASHID" {
        return Some((ItemFamily::Cash, 0));
    }
    if expr == "CORPSEID" {
        return Some((ItemFamily::Corpse, 0));
    }
    None
}

fn legacy_item_family_base_id(family: ItemFamily) -> i32 {
    match family {
        ItemFamily::Thing => 0,
        ItemFamily::Food => 31,
        ItemFamily::Scroll => 47,
        ItemFamily::Potion => 71,
        ItemFamily::Weapon => 89,
        ItemFamily::Armor => 130,
        ItemFamily::Shield => 147,
        ItemFamily::Cloak => 155,
        ItemFamily::Boots => 162,
        ItemFamily::Ring => 169,
        ItemFamily::Stick => 178,
        ItemFamily::Artifact => 195,
        ItemFamily::Cash => 221,
        ItemFamily::Corpse => 222,
        ItemFamily::Unknown => -1,
    }
}

fn split_top_level_csv(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chunk = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    for ch in raw.chars() {
        if in_quotes {
            chunk.push(ch);
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == '"' {
                in_quotes = false;
            }
            continue;
        }
        match ch {
            '"' => {
                in_quotes = true;
                chunk.push(ch);
            }
            ',' => {
                out.push(chunk.trim().to_string());
                chunk.clear();
            }
            _ => chunk.push(ch),
        }
    }
    if !chunk.trim().is_empty() {
        out.push(chunk.trim().to_string());
    }
    out
}

fn parse_i32_token(raw: &str) -> Option<i32> {
    raw.trim().parse::<i32>().ok()
}

fn parse_i64_token(raw: &str) -> Option<i64> {
    raw.trim().parse::<i64>().ok()
}

fn parse_u8_token(raw: &str) -> Option<u8> {
    raw.trim().parse::<u8>().ok()
}

fn parse_string_token(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() < 2 || !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return None;
    }
    Some(trimmed.trim_matches('"').replace("\\\"", "\""))
}

fn sanitize_catalog_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "(null)" || trimmed == "?" {
        return None;
    }
    Some(trimmed.replace('_', " "))
}

fn normalize_item_lookup(raw: &str) -> String {
    let mut normalized = String::with_capacity(raw.len());
    let mut was_space = false;
    for ch in raw.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            normalized.push(lower);
            was_space = false;
        } else if !was_space {
            normalized.push(' ');
            was_space = true;
        }
    }
    normalized.trim().to_string()
}

const LEGACY_SPELL_NAMES: [&str; 42] = [
    "monster detection",
    "object detection",
    "magic missile",
    "firebolt",
    "teleport",
    "ball lightning",
    "sleep",
    "disrupt",
    "disintegrate",
    "polymorph",
    "healing",
    "dispelling",
    "identification",
    "breathing",
    "invisibility",
    "the warp",
    "enchantment",
    "blessing",
    "restoration",
    "curing",
    "true sight",
    "hellfire",
    "self knowledge",
    "heroism",
    "return",
    "desecration",
    "haste",
    "summoning",
    "sanctuary",
    "accuracy",
    "ritual magic",
    "apportation",
    "shadow form",
    "alertness",
    "regeneration",
    "sanctification",
    "clairvoyance",
    "energy drain",
    "levitate",
    "fear",
    "wishing",
    "nutrition",
];

const LEGACY_SPELL_COSTS: [i32; 42] = [
    3, 3, 10, 20, 20, 25, 15, 30, 40, 30, 15, 40, 10, 20, 15, 50, 30, 30, 20, 20, 20, 90, 10, 20,
    10, 50, 15, 20, 75, 20, 50, 15, 50, 15, 20, 75, 10, 40, 25, 10, 100, 25,
];

const LEGACY_SPELL_SORTED_IDS: [usize; 42] = [
    29, 33, 31, 5, 17, 13, 36, 19, 25, 8, 11, 7, 16, 37, 39, 3, 26, 10, 21, 23, 12, 14, 38, 2, 0,
    41, 1, 9, 34, 18, 24, 30, 35, 28, 22, 32, 6, 27, 4, 15, 20, 40,
];

fn cast_spell_by_id(
    state: &mut GameState,
    events: &mut Vec<Event>,
    spell_index: usize,
) -> (String, bool) {
    sync_spellbook_state(state);
    let Some(spell_name) = LEGACY_SPELL_NAMES.get(spell_index).copied() else {
        return (format!("That spell id ({spell_index}) does not exist."), true);
    };
    if !state.spellbook.spells.get(spell_index).map(|spell| spell.known).unwrap_or(false) {
        return ("You don't know that spell.".to_string(), true);
    }
    if has_active_fear(state) {
        return ("You are too afraid to concentrate on a spell!".to_string(), true);
    }
    let spell_cost = compute_spell_drain(state, spell_index);
    if spell_cost > state.spellbook.mana {
        let note = if state.progression.lunarity == -1 && state.spellbook.mana >= (spell_cost / 2) {
            "The contrary moon has made that spell too draining!".to_string()
        } else {
            "You lack the power for that spell!".to_string()
        };
        return (note, true);
    }

    state.spellbook.mana -= spell_cost;
    push_or_refresh_status(&mut state.status_effects, "spell_focus", 1, 0);
    state.spellbook.next_spell_index = (spell_index as u8).wrapping_add(1);

    let begin_projectile_spell =
        |state: &mut GameState,
         mode: ProjectileKind,
         label: &str,
         damage_min: i32,
         damage_max: i32,
         damage_type: ProjectileDamageType| {
            let action = PendingProjectileAction {
                source_token: "m".to_string(),
                turn_minutes: estimate_legacy_turn_minutes(
                    "m",
                    state.world_mode,
                    state.options.searchnum,
                ),
                mode,
                item_id: None,
                item_name: label.to_string(),
                hit_bonus: statmod(state.attributes.iq.max(1))
                    + statmod(state.attributes.power.max(1)),
                damage_bonus: 0,
                damage_min,
                damage_max,
                damage_type,
                max_range: 12,
                allows_drop: false,
            };
            let _ = begin_targeting_interaction(state, action);
            format!("{label}: choose a target.")
        };

    let effect_note = match spell_name {
        "monster detection" => {
            let detected = state
                .monsters
                .iter()
                .filter(|monster| monster.position.manhattan_distance(state.player.position) <= 8)
                .count();
            format!("detected {detected} nearby signatures")
        }
        "object detection" => {
            let objects = state
                .ground_items
                .iter()
                .filter(|item| item.position.manhattan_distance(state.player.position) <= 8)
                .count();
            format!("detected {objects} nearby objects")
        }
        "magic missile" => begin_projectile_spell(
            state,
            ProjectileKind::MagicMissile,
            "magic missile",
            6,
            8,
            ProjectileDamageType::Magic,
        ),
        "firebolt" => begin_projectile_spell(
            state,
            ProjectileKind::FireBolt,
            "firebolt",
            8,
            14,
            ProjectileDamageType::Flame,
        ),
        "teleport" => {
            spell_shift_player(state, 5, 3);
            "space folded around the caster".to_string()
        }
        "ball lightning" => spell_damage_radius(state, events, 2, 10, "electrical arcs"),
        "sleep" => spell_mark_nearest_as_skirmisher(state, 6, "target dulled into torpor"),
        "disrupt" => begin_projectile_spell(
            state,
            ProjectileKind::MagicMissile,
            "disruptive surge",
            5,
            16,
            ProjectileDamageType::Magic,
        ),
        "disintegrate" => spell_remove_nearest(state, events, 5, "target annihilated"),
        "polymorph" => spell_polymorph_nearest(state, 6),
        "healing" => {
            state.player.stats.hp = (state.player.stats.hp + 14).min(state.player.stats.max_hp);
            "major vitality restored".to_string()
        }
        "dispelling" => dispel_or_decurse_with_branching(state, 1),
        "identification" => state
            .player
            .inventory
            .first()
            .map(|item| format!("identified `{}`", item.name))
            .unwrap_or_else(|| "nothing to identify".to_string()),
        "breathing" => {
            push_or_refresh_status(&mut state.status_effects, "breathing", 10, 1);
            "lungs adapted to hostile air".to_string()
        }
        "invisibility" => {
            push_or_refresh_status(&mut state.status_effects, "invisible", 8, 1);
            "light bent around the caster".to_string()
        }
        "the warp" => {
            spell_shift_player(state, 9, 5);
            "reality folded violently".to_string()
        }
        "enchantment" => enchant_item_with_risk(state, None, 1, events),
        "blessing" => {
            state.progression.deity_favor += 1;
            state.progression.law_chaos_score += 1;
            bless_item_with_risk(state, 2, events)
        }
        "restoration" => {
            state.player.stats.hp = state.player.stats.max_hp;
            consume_status(state, "poison");
            "body and spirit restored to baseline".to_string()
        }
        "curing" => {
            consume_status(state, "poison");
            "toxins neutralized".to_string()
        }
        "true sight" => {
            ensure_known_site(state, state.player.position);
            ensure_known_site(state, Position { x: 0, y: 0 });
            ensure_known_site(
                state,
                Position {
                    x: state.bounds.width.saturating_sub(1),
                    y: state.bounds.height.saturating_sub(1),
                },
            );
            "major map anchors were revealed".to_string()
        }
        "hellfire" => spell_damage_radius(state, events, 4, 24, "infernal flames"),
        "self knowledge" => format!(
            "hp={}/{} gold={} favor={} alignment={:?}",
            state.player.stats.hp,
            state.player.stats.max_hp,
            state.gold,
            state.progression.deity_favor,
            state.progression.alignment
        ),
        "heroism" => {
            push_or_refresh_status(&mut state.status_effects, "heroism", 10, 2);
            state.player.stats.attack_max += 2;
            "battle fervor surged".to_string()
        }
        "return" => {
            if let Some(pos) = state.topology.last_city_position
                && state.tile_is_walkable(pos)
            {
                state.player.position = pos;
            }
            "caster anchored to a known waypoint".to_string()
        }
        "desecration" => {
            state.progression.deity_favor -= 1;
            state.progression.law_chaos_score -= 2;
            "chaotic taint spread through the area".to_string()
        }
        "haste" => {
            push_or_refresh_status(&mut state.status_effects, "haste", 6, 1);
            "time momentarily accelerated".to_string()
        }
        "summoning" => spell_summon_guardian(state),
        "sanctuary" => {
            push_or_refresh_status(&mut state.status_effects, "sanctuary", 8, 2);
            state.legal_heat = state.legal_heat.saturating_sub(2);
            "protective aura suppressed threats".to_string()
        }
        "accuracy" => {
            push_or_refresh_status(&mut state.status_effects, "accuracy", 10, 2);
            "targeting acuity heightened".to_string()
        }
        "ritual magic" => {
            state.progression.deity_favor += 2;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.saturating_add(1);
            "long-form rite advanced progression".to_string()
        }
        "apportation" => {
            if let Some(idx) = state
                .ground_items
                .iter()
                .position(|item| item.position.manhattan_distance(state.player.position) <= 3)
            {
                let pulled = state.ground_items.remove(idx);
                state.player.inventory.push(pulled.item);
                "an item was pulled into your pack".to_string()
            } else {
                "no item resonated with the spell".to_string()
            }
        }
        "shadow form" => {
            push_or_refresh_status(&mut state.status_effects, "shadow_form", 8, 1);
            "body phased into umbral contours".to_string()
        }
        "alertness" => {
            consume_status(state, "poison");
            consume_status(state, "immobile");
            "mind snapped into full awareness".to_string()
        }
        "regeneration" => {
            push_or_refresh_status(&mut state.status_effects, "regen", 8, 2);
            "life force accelerated recovery".to_string()
        }
        "sanctification" => {
            state.progression.deity_favor += 2;
            state.progression.law_chaos_score += 2;
            "holy resonance strengthened lawful bonds".to_string()
        }
        "clairvoyance" => {
            ensure_known_site(state, state.player.position);
            ensure_known_site(
                state,
                Position {
                    x: state.player.position.x.saturating_sub(1),
                    y: state.player.position.y,
                },
            );
            ensure_known_site(
                state,
                Position {
                    x: (state.player.position.x + 1).clamp(0, state.bounds.width.saturating_sub(1)),
                    y: state.player.position.y,
                },
            );
            "local pathways became clear".to_string()
        }
        "energy drain" => spell_energy_drain(state, events),
        "levitate" => {
            push_or_refresh_status(&mut state.status_effects, "levitate", 8, 1);
            "gravity loosened around the caster".to_string()
        }
        "fear" => {
            for monster in &mut state.monsters {
                if monster.position.manhattan_distance(state.player.position) <= 3 {
                    monster.behavior = MonsterBehavior::Skirmisher;
                }
            }
            "nearby foes recoiled in panic".to_string()
        }
        "wishing" => {
            let primary_kind =
                if state.wizard.enabled { WishItemKind::Artifact } else { WishItemKind::Thing };
            let item_name = random_item_from_kind(state, primary_kind)
                .or_else(|| random_item_from_kind(state, WishItemKind::Potion))
                .unwrap_or_else(|| "food ration".to_string());
            let result = add_item_to_inventory_or_ground(state, item_name, events);
            format!("wish magic resolved ({result})")
        }
        "nutrition" => {
            state.food += 12;
            "hunger was magically sated".to_string()
        }
        _ => "spell produced no measurable effect".to_string(),
    };

    (
        format!(
            "cast spell#{spell_index:02} `{spell_name}` cost={spell_cost}; mana {}/{} ({effect_note})",
            state.spellbook.mana, state.spellbook.max_mana
        ),
        true,
    )
}

fn disarm_adjacent_trap(state: &mut GameState, events: &mut Vec<Event>) -> (String, bool) {
    let player = state.player.position;
    if let Some(trap) = state
        .traps
        .iter_mut()
        .find(|trap| trap.armed && trap.position.manhattan_distance(player) <= 1)
    {
        trap.armed = false;
        if state.legal_heat > 0 {
            state.legal_heat -= 1;
        }
        state.gold += 5;
        events.push(Event::EconomyUpdated {
            source: "disarm_reward".to_string(),
            gold: state.gold,
            bank_gold: state.bank_gold,
        });
        return (format!("trap {} disarmed and bounty collected", trap.id), true);
    }
    events.push(Event::EconomyUpdated {
        source: "disarm_attempt".to_string(),
        gold: state.gold,
        bank_gold: state.bank_gold,
    });
    ("disarm attempted but no adjacent armed trap".to_string(), true)
}

fn has_adjacent_monster(state: &GameState) -> bool {
    state
        .monsters
        .iter()
        .any(|monster| monster.position.manhattan_distance(state.player.position) == 1)
}

fn nearest_monster_index(state: &GameState, radius: i32) -> Option<usize> {
    state
        .monsters
        .iter()
        .enumerate()
        .filter_map(|(idx, monster)| {
            let dist = monster.position.manhattan_distance(state.player.position);
            (dist <= radius).then_some((idx, dist))
        })
        .min_by_key(|(_, dist)| *dist)
        .map(|(idx, _)| idx)
}

pub fn line_path(origin: Position, target: Position) -> Vec<Position> {
    let mut points = Vec::new();
    let mut x0 = origin.x;
    let mut y0 = origin.y;
    let x1 = target.x;
    let y1 = target.y;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push(Position { x: x0, y: y0 });
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err.saturating_mul(2);
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    points
}

pub fn projectile_unblocked(state: &GameState, pos: Position) -> bool {
    if !state.bounds.contains(pos) {
        return false;
    }
    if !state.tile_is_walkable(pos) {
        return false;
    }
    if let Some(cell) = state.tile_site_at(pos)
        && (cell.flags & (TILE_FLAG_BLOCK_MOVE | TILE_FLAG_PORTCULLIS | TILE_FLAG_SECRET)) != 0
    {
        return false;
    }
    let glyph = state.map_glyph_at(pos);
    if matches!(glyph, '#' | '-' | 'D' | 'J' | '=') {
        return false;
    }
    true
}

pub fn projectile_trace_to_target(
    state: &GameState,
    origin: Position,
    target: Position,
    stop_before_blocking: bool,
) -> Position {
    let path = line_path(origin, target);
    let mut final_pos = origin;
    let mut previous = origin;
    for pos in path.into_iter().skip(1) {
        if !state.bounds.contains(pos) {
            break;
        }
        if monster_index_at(state, pos).is_some() {
            return pos;
        }
        if !projectile_unblocked(state, pos) {
            return if stop_before_blocking { previous } else { pos };
        }
        previous = pos;
        final_pos = pos;
    }
    final_pos
}

fn legacy_hit_roll<R: RandomSource>(hit: i32, ac: i32, rng: &mut R) -> bool {
    let roll = rng.range_inclusive_i32(0, 19);
    if roll == 0 {
        return true;
    }
    if roll == 19 {
        return false;
    }
    roll < (hit - ac)
}

fn weapon_hand_item_id(state: &GameState) -> Option<u32> {
    state.player.equipment.weapon_hand
}

fn weapon_hand_item(state: &GameState) -> Option<&Item> {
    let weapon_id = weapon_hand_item_id(state)?;
    state.player.inventory.iter().find(|item| item.id == weapon_id)
}

fn weapon_hand_item_mut(state: &mut GameState) -> Option<&mut Item> {
    let weapon_id = weapon_hand_item_id(state)?;
    state.player.inventory.iter_mut().find(|item| item.id == weapon_id)
}

fn weapon_hand_is_longbow(state: &GameState) -> bool {
    let contract = legacy_projectile_contract();
    weapon_hand_item(state).is_some_and(|item| item.legacy_id == contract.ob_longbow)
}

fn weapon_hand_is_crossbow(state: &GameState) -> bool {
    let contract = legacy_projectile_contract();
    weapon_hand_item(state).is_some_and(|item| item.legacy_id == contract.ob_crossbow)
}

fn weapon_hand_crossbow_loaded(state: &GameState) -> bool {
    let contract = legacy_projectile_contract();
    weapon_hand_item(state)
        .is_some_and(|item| item.legacy_id == contract.ob_crossbow && item.aux == contract.loaded)
}

fn set_weapon_hand_crossbow_loaded(state: &mut GameState, loaded: bool) {
    let contract = legacy_projectile_contract();
    if let Some(weapon) = weapon_hand_item_mut(state)
        && weapon.legacy_id == contract.ob_crossbow
    {
        weapon.aux = if loaded { contract.loaded } else { contract.unloaded };
    }
}

fn parse_direction_delta_from_text(text: &str) -> Option<(i32, i32)> {
    if text.trim().len() != 1 {
        return None;
    }
    let ch = text.chars().next()?;
    direction_delta_from_char(ch)
}

fn remove_monster_with_drops(
    state: &mut GameState,
    idx: usize,
    events: &mut Vec<Event>,
) -> Option<Monster> {
    if idx >= state.monsters.len() {
        return None;
    }
    let mut monster = state.monsters.remove(idx);
    if !monster.on_death_drops.is_empty() {
        let mut names = Vec::new();
        for item in monster.on_death_drops.drain(..) {
            names.push(item.name.clone());
            state.ground_items.push(GroundItem { position: monster.position, item });
        }
        if !names.is_empty() {
            push_timeline_line(state, format!("{} drops {}.", monster.name, names.join(", ")));
            events.push(Event::LegacyHandled {
                token: "loot_drop".to_string(),
                note: format!("{} dropped {}", monster.name, names.join(", ")),
                fully_modeled: true,
            });
        }
    }
    Some(monster)
}

fn spell_shift_player(state: &mut GameState, x_delta: i32, y_delta: i32) {
    let mut target =
        Position { x: state.player.position.x + x_delta, y: state.player.position.y + y_delta };
    target.x = target.x.clamp(0, state.bounds.width.saturating_sub(1));
    target.y = target.y.clamp(0, state.bounds.height.saturating_sub(1));

    if state.tile_is_walkable(target) && !is_occupied(state, target) {
        state.player.position = target;
        return;
    }

    let mut fallback = None;
    for radius in 1..=6 {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let probe = Position { x: target.x + dx, y: target.y + dy };
                if !state.bounds.contains(probe) {
                    continue;
                }
                if state.tile_is_walkable(probe) && !is_occupied(state, probe) {
                    fallback = Some(probe);
                    break;
                }
            }
            if fallback.is_some() {
                break;
            }
        }
        if fallback.is_some() {
            break;
        }
    }
    if let Some(pos) = fallback {
        state.player.position = pos;
    }
}

fn spell_damage_radius(
    state: &mut GameState,
    events: &mut Vec<Event>,
    radius: i32,
    damage: i32,
    flavor: &str,
) -> String {
    let center = state.player.position;
    let mut targets: Vec<usize> = state
        .monsters
        .iter()
        .enumerate()
        .filter_map(|(idx, monster)| {
            (monster.position.manhattan_distance(center) <= radius).then_some(idx)
        })
        .collect();

    if targets.is_empty() {
        return format!("{flavor}: no targets in radius");
    }

    let mut hit_count = 0usize;
    for idx in &targets {
        let monster = &mut state.monsters[*idx];
        let applied = monster.stats.apply_damage(damage.max(1));
        events.push(Event::Attacked {
            monster_id: monster.id,
            damage: applied,
            remaining_hp: monster.stats.hp,
        });
        hit_count += 1;
    }

    targets.sort_unstable();
    targets.reverse();
    for idx in targets {
        if state.monsters[idx].stats.is_alive() {
            continue;
        }
        let monster_id = state.monsters[idx].id;
        let _ = remove_monster_with_drops(state, idx, events);
        state.monsters_defeated = state.monsters_defeated.saturating_add(1);
        events.push(Event::MonsterDefeated { monster_id });
    }

    format!("{flavor}: impacted {hit_count} targets")
}

fn spell_mark_nearest_as_skirmisher(state: &mut GameState, radius: i32, flavor: &str) -> String {
    let Some(idx) = nearest_monster_index(state, radius) else {
        return "sleep failed: no target in range".to_string();
    };
    let monster = &mut state.monsters[idx];
    monster.behavior = MonsterBehavior::Skirmisher;
    format!("{flavor} on {}", monster.name)
}

fn spell_remove_nearest(
    state: &mut GameState,
    events: &mut Vec<Event>,
    radius: i32,
    flavor: &str,
) -> String {
    let Some(idx) = nearest_monster_index(state, radius) else {
        return "disintegrate failed: no target in range".to_string();
    };
    let Some(monster) = remove_monster_with_drops(state, idx, events) else {
        return "disintegrate failed: target vanished".to_string();
    };
    state.monsters_defeated = state.monsters_defeated.saturating_add(1);
    events.push(Event::MonsterDefeated { monster_id: monster.id });
    format!("{flavor} ({})", monster.name)
}

fn spell_polymorph_nearest(state: &mut GameState, radius: i32) -> String {
    let Some(idx) = nearest_monster_index(state, radius) else {
        return "polymorph failed: no target in range".to_string();
    };
    let monster = &mut state.monsters[idx];
    monster.name = format!("polymorphed {}", monster.name);
    monster.stats.max_hp = (monster.stats.max_hp + 3).max(1);
    monster.stats.hp = monster.stats.max_hp;
    monster.stats.attack_min = (monster.stats.attack_min + 1).max(1);
    monster.stats.attack_max = (monster.stats.attack_max + 2).max(monster.stats.attack_min);
    monster.behavior = MonsterBehavior::Skirmisher;
    format!("{} was transformed", monster.name)
}

fn spell_summon_guardian(state: &mut GameState) -> String {
    let base = state.player.position;
    let candidate_offsets = [
        Position { x: 1, y: 0 },
        Position { x: -1, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 0, y: -1 },
    ];

    for offset in candidate_offsets {
        let spawn = Position { x: base.x + offset.x, y: base.y + offset.y };
        if !state.bounds.contains(spawn) {
            continue;
        }
        if !state.tile_is_walkable(spawn) || is_occupied(state, spawn) {
            continue;
        }
        let guardian_id = state.spawn_monster(
            "summoned guardian",
            spawn,
            Stats { hp: 14, max_hp: 14, attack_min: 3, attack_max: 7, defense: 2 },
        );
        if let Some(monster) = state.monsters.iter_mut().find(|monster| monster.id == guardian_id) {
            monster.faction = Faction::Law;
            monster.behavior = MonsterBehavior::Brute;
        }
        return "guardian answered the summoning".to_string();
    }

    "summoning failed: no open adjacent tile".to_string()
}

fn spell_energy_drain(state: &mut GameState, events: &mut Vec<Event>) -> String {
    let Some(idx) = nearest_monster_index(state, 4) else {
        return "energy drain failed: no target in range".to_string();
    };

    let (monster_id, drained) = {
        let monster = &mut state.monsters[idx];
        let drain_amount = (monster.stats.hp / 2).max(1);
        let applied = monster.stats.apply_damage(drain_amount);
        (monster.id, applied)
    };
    let remaining_hp = state.monsters[idx].stats.hp;
    events.push(Event::Attacked { monster_id, damage: drained, remaining_hp });
    state.spellbook.mana = (state.spellbook.mana + drained).min(state.spellbook.max_mana);

    if !state.monsters[idx].stats.is_alive() {
        let _ = remove_monster_with_drops(state, idx, events);
        state.monsters_defeated = state.monsters_defeated.saturating_add(1);
        events.push(Event::MonsterDefeated { monster_id });
    }

    format!("drained {drained} energy")
}

fn adjacent_monster_direction(state: &GameState) -> Option<Direction> {
    let player = state.player.position;
    for (direction, pos) in [
        (Direction::North, player.offset(Direction::North)),
        (Direction::South, player.offset(Direction::South)),
        (Direction::East, player.offset(Direction::East)),
        (Direction::West, player.offset(Direction::West)),
    ] {
        if monster_index_at(state, pos).is_some() {
            return Some(direction);
        }
    }
    None
}

fn is_occupied(state: &GameState, pos: Position) -> bool {
    state.player.position == pos || state.monsters.iter().any(|monster| monster.position == pos)
}

fn infer_monster_profile(name: &str) -> (MonsterBehavior, Faction) {
    let lowered = name.to_ascii_lowercase();
    if lowered.contains("guard") {
        return (MonsterBehavior::Social, Faction::Neutral);
    }
    if lowered.contains("priest") || lowered.contains("oracle") || lowered.contains("sage") {
        return (MonsterBehavior::Social, Faction::Law);
    }
    if lowered.contains("sheep")
        || lowered.contains("goat")
        || lowered.contains("cow")
        || lowered.contains("horse")
        || lowered.contains("deer")
        || lowered.contains("rabbit")
    {
        return (MonsterBehavior::Social, Faction::Neutral);
    }
    if lowered.contains("mage") || lowered.contains("wizard") || lowered.contains("imp") {
        return (MonsterBehavior::Caster, Faction::Chaos);
    }
    if lowered.contains("wolf") || lowered.contains("stalker") || lowered.contains("rat") {
        return (MonsterBehavior::Skirmisher, Faction::Wild);
    }
    (MonsterBehavior::Brute, Faction::Neutral)
}

fn item_burden(item: &Item) -> i32 {
    if item.weight > 0 {
        let scaled = (item.weight + 9) / 10;
        return scaled.clamp(1, 50);
    }
    match item.family {
        ItemFamily::Armor | ItemFamily::Shield => 6,
        ItemFamily::Weapon | ItemFamily::Artifact => 4,
        ItemFamily::Food | ItemFamily::Potion | ItemFamily::Scroll => 1,
        _ => 2,
    }
}

fn canonical_item_alias_name(name: &str) -> Option<&'static str> {
    let normalized = normalize_item_lookup(name);
    match normalized.as_str() {
        "healing potion" => Some("potion of healing"),
        "scroll identify" | "identify scroll" => Some("scroll of identification"),
        "charged stick" | "wand" | "staff" => Some("staff of missiles"),
        "rations pack" | "ration" => Some("food ration"),
        "chain armor" | "chain armour" => Some("chain mail"),
        "artifact star" => Some("Star Gem"),
        _ => None,
    }
}

fn instantiate_item_from_name(item_id: u32, requested_name: &str) -> Item {
    let mut lookup_names = Vec::new();
    lookup_names.push(normalize_item_lookup(requested_name));
    if let Some(alias) = canonical_item_alias_name(requested_name) {
        lookup_names.push(normalize_item_lookup(alias));
    }
    for lookup in lookup_names {
        if lookup.is_empty() {
            continue;
        }
        if let Some(template) = legacy_item_templates()
            .iter()
            .find(|entry| entry.normalized_names.iter().any(|name| name == &lookup))
        {
            let display_name = if template.truename.is_empty() {
                requested_name.to_string()
            } else {
                template.truename.clone()
            };
            return Item {
                id: item_id,
                name: display_name,
                legacy_id: template.legacy_id,
                family: template.family,
                usef: template.usef.clone(),
                item_type: template.item_type.clone(),
                weight: template.weight,
                plus: template.plus,
                charge: template.charge,
                dmg: template.dmg,
                hit: template.hit,
                aux: template.aux,
                number: template.number,
                fragility: template.fragility,
                basevalue: template.basevalue,
                known: template.known,
                used: template.used,
                blessing: template.blessing,
                level: template.level,
                uniqueness: template.uniqueness.clone(),
                objchar: template.objchar.clone(),
                objstr: template.objstr.clone(),
                truename: template.truename.clone(),
                cursestr: template.cursestr.clone(),
            };
        }
    }

    Item::basic(item_id, requested_name)
}

fn add_item_to_inventory_or_ground(
    state: &mut GameState,
    name: impl Into<String>,
    events: &mut Vec<Event>,
) -> String {
    let name = name.into();
    let item = instantiate_item_from_name(state.next_item_id, &name);
    state.next_item_id += 1;
    add_existing_item_to_inventory_or_ground(state, item, events)
}

fn add_existing_item_to_inventory_or_ground(
    state: &mut GameState,
    item: Item,
    events: &mut Vec<Event>,
) -> String {
    let name = item.name.clone();
    let capacity = effective_inventory_capacity(state);

    if state.player.inventory.len() < capacity {
        state.carry_burden = state.carry_burden.saturating_add(item_burden(&item));
        state.player.inventory.push(item.clone());
        push_item_to_pack_front(state, item.id);
        auto_equip_item(state, &item);
        events.push(Event::PickedUp { item_id: item.id, name: item.name });
        sync_pack_order(state);
        format!("received {name}")
    } else {
        let position = state.player.position;
        state.ground_items.push(GroundItem { position, item: item.clone() });
        events.push(Event::InventoryFull { capacity });
        format!("inventory full; {name} left on ground")
    }
}

fn try_pickup_at_player(state: &mut GameState, events: &mut Vec<Event>) {
    let capacity = effective_inventory_capacity(state);
    if state.player.inventory.len() >= capacity {
        state.log.push("Inventory is full.".to_string());
        events.push(Event::InventoryFull { capacity });
    } else if let Some(ground_index) = ground_item_index_at(state, state.player.position) {
        let ground = state.ground_items.remove(ground_index);
        state.log.push(format!("Picked up {}.", ground.item.name));
        events.push(Event::PickedUp { item_id: ground.item.id, name: ground.item.name.clone() });
        state.carry_burden = state.carry_burden.saturating_add(item_burden(&ground.item));
        auto_equip_item(state, &ground.item);
        push_item_to_pack_front(state, ground.item.id);
        state.player.inventory.push(ground.item);
        sync_pack_order(state);
    } else {
        state.log.push("Nothing to pick up.".to_string());
        events.push(Event::NoItemToPickUp);
    }
}

fn is_two_handed_weapon(item: &Item) -> bool {
    if item.family != ItemFamily::Weapon {
        return false;
    }
    if item.item_type.eq_ignore_ascii_case("MISSILE") {
        return false;
    }
    item.weight >= 75
        || item.dmg >= 20
        || matches!(item.usef.as_str(), "I_VICTRIX" | "I_LIGHTSABRE" | "I_DEMONBLADE" | "I_ANTIOCH")
}

fn inventory_slot_name(slot: usize) -> &'static str {
    match slot {
        SLOT_UP_IN_AIR => "up in air",
        SLOT_READY_HAND => "ready hand",
        SLOT_WEAPON_HAND => "weapon hand",
        SLOT_LEFT_SHOULDER => "left shoulder",
        SLOT_RIGHT_SHOULDER => "right shoulder",
        SLOT_BELT_1 | SLOT_BELT_2 | SLOT_BELT_3 => "belt",
        SLOT_SHIELD => "shield",
        SLOT_ARMOR => "armor",
        SLOT_BOOTS => "boots",
        SLOT_CLOAK => "cloak",
        SLOT_RING_1 | SLOT_RING_2 | SLOT_RING_3 | SLOT_RING_4 => "ring",
        _ => "unknown",
    }
}

fn inventory_slot_item_id(state: &GameState, slot: usize) -> Option<u32> {
    match slot {
        SLOT_UP_IN_AIR => state.player.equipment.up_in_air,
        SLOT_READY_HAND => state.player.equipment.ready_hand,
        SLOT_WEAPON_HAND => state.player.equipment.weapon_hand,
        SLOT_LEFT_SHOULDER => state.player.equipment.left_shoulder,
        SLOT_RIGHT_SHOULDER => state.player.equipment.right_shoulder,
        SLOT_BELT_1 => state.player.equipment.belt_1,
        SLOT_BELT_2 => state.player.equipment.belt_2,
        SLOT_BELT_3 => state.player.equipment.belt_3,
        SLOT_SHIELD => state.player.equipment.shield,
        SLOT_ARMOR => state.player.equipment.armor,
        SLOT_BOOTS => state.player.equipment.boots,
        SLOT_CLOAK => state.player.equipment.cloak,
        SLOT_RING_1 => state.player.equipment.ring_1,
        SLOT_RING_2 => state.player.equipment.ring_2,
        SLOT_RING_3 => state.player.equipment.ring_3,
        SLOT_RING_4 => state.player.equipment.ring_4,
        _ => None,
    }
}

fn set_inventory_slot_item_id(state: &mut GameState, slot: usize, item_id: Option<u32>) -> bool {
    match slot {
        SLOT_UP_IN_AIR => state.player.equipment.up_in_air = item_id,
        SLOT_READY_HAND => state.player.equipment.ready_hand = item_id,
        SLOT_WEAPON_HAND => state.player.equipment.weapon_hand = item_id,
        SLOT_LEFT_SHOULDER => state.player.equipment.left_shoulder = item_id,
        SLOT_RIGHT_SHOULDER => state.player.equipment.right_shoulder = item_id,
        SLOT_BELT_1 => state.player.equipment.belt_1 = item_id,
        SLOT_BELT_2 => state.player.equipment.belt_2 = item_id,
        SLOT_BELT_3 => state.player.equipment.belt_3 = item_id,
        SLOT_SHIELD => state.player.equipment.shield = item_id,
        SLOT_ARMOR => state.player.equipment.armor = item_id,
        SLOT_BOOTS => state.player.equipment.boots = item_id,
        SLOT_CLOAK => state.player.equipment.cloak = item_id,
        SLOT_RING_1 => state.player.equipment.ring_1 = item_id,
        SLOT_RING_2 => state.player.equipment.ring_2 = item_id,
        SLOT_RING_3 => state.player.equipment.ring_3 = item_id,
        SLOT_RING_4 => state.player.equipment.ring_4 = item_id,
        _ => return false,
    }
    true
}

fn slot_accepts_item(slot: usize, item: &Item) -> bool {
    if slot == SLOT_ARMOR {
        return item.family == ItemFamily::Armor;
    }
    if slot == SLOT_SHIELD {
        return item.family == ItemFamily::Shield;
    }
    if slot == SLOT_BOOTS {
        return item.family == ItemFamily::Boots;
    }
    if slot == SLOT_CLOAK {
        return item.family == ItemFamily::Cloak;
    }
    if matches!(slot, SLOT_RING_1 | SLOT_RING_2 | SLOT_RING_3 | SLOT_RING_4) {
        return item.family == ItemFamily::Ring;
    }
    true
}

fn item_is_cursed_in_use(item: &Item, slot: usize) -> bool {
    slot != SLOT_UP_IN_AIR && item.blessing < 0 && item.used
}

fn equipped_weapon_is_two_handed(state: &GameState) -> bool {
    let Some(weapon_id) = state.player.equipment.weapon_hand else {
        return false;
    };
    state
        .player
        .inventory
        .iter()
        .find(|entry| entry.id == weapon_id)
        .is_some_and(is_two_handed_weapon)
}

fn auto_equip_item(state: &mut GameState, item: &Item) {
    match item.family {
        ItemFamily::Weapon => {
            if state.player.equipment.weapon_hand.is_none() {
                state.player.equipment.weapon_hand = Some(item.id);
            }
            if is_two_handed_weapon(item) {
                state.player.equipment.ready_hand = Some(item.id);
                state.player.equipment.shield = None;
            } else if state.player.equipment.ready_hand.is_none() {
                state.player.equipment.ready_hand = Some(item.id);
            }
        }
        ItemFamily::Shield => {
            if state.player.equipment.shield.is_none() && !equipped_weapon_is_two_handed(state) {
                state.player.equipment.shield = Some(item.id);
            }
        }
        ItemFamily::Armor => {
            if state.player.equipment.armor.is_none() {
                state.player.equipment.armor = Some(item.id);
            }
        }
        ItemFamily::Boots => {
            if state.player.equipment.boots.is_none() {
                state.player.equipment.boots = Some(item.id);
            }
        }
        ItemFamily::Cloak => {
            if state.player.equipment.cloak.is_none() {
                state.player.equipment.cloak = Some(item.id);
            }
        }
        ItemFamily::Ring => {
            if state.player.equipment.ring_1.is_none() {
                state.player.equipment.ring_1 = Some(item.id);
            } else if state.player.equipment.ring_2.is_none() {
                state.player.equipment.ring_2 = Some(item.id);
            } else if state.player.equipment.ring_3.is_none() {
                state.player.equipment.ring_3 = Some(item.id);
            } else if state.player.equipment.ring_4.is_none() {
                state.player.equipment.ring_4 = Some(item.id);
            }
        }
        ItemFamily::Thing => {
            if state.player.equipment.belt_1.is_none() {
                state.player.equipment.belt_1 = Some(item.id);
            } else if state.player.equipment.belt_2.is_none() {
                state.player.equipment.belt_2 = Some(item.id);
            } else if state.player.equipment.belt_3.is_none() {
                state.player.equipment.belt_3 = Some(item.id);
            }
        }
        _ => {}
    }
}

fn unequip_item_id(equipment: &mut EquipmentSlots, item_id: u32) {
    for slot in [
        &mut equipment.up_in_air,
        &mut equipment.ready_hand,
        &mut equipment.weapon_hand,
        &mut equipment.left_shoulder,
        &mut equipment.right_shoulder,
        &mut equipment.belt_1,
        &mut equipment.belt_2,
        &mut equipment.belt_3,
        &mut equipment.shield,
        &mut equipment.armor,
        &mut equipment.boots,
        &mut equipment.cloak,
        &mut equipment.ring_1,
        &mut equipment.ring_2,
        &mut equipment.ring_3,
        &mut equipment.ring_4,
    ] {
        if slot.is_some_and(|id| id == item_id) {
            *slot = None;
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct EquipmentEffectProfile {
    attack_min_bonus: i32,
    attack_max_bonus: i32,
    to_hit_bonus: i32,
    defense_bonus: i32,
    block_bonus: i32,
    poison_resist_bonus: i32,
    fire_resist_bonus: i32,
    magic_resist_bonus: i32,
    grants_poison_immunity: bool,
    grants_fear_immunity: bool,
    regen_per_turn: i32,
    carry_capacity_delta: i32,
}

fn equipped_item_ids(equipment: &EquipmentSlots) -> Vec<u32> {
    let mut ids = Vec::new();
    for item_id in [
        equipment.up_in_air,
        equipment.ready_hand,
        equipment.weapon_hand,
        equipment.left_shoulder,
        equipment.right_shoulder,
        equipment.belt_1,
        equipment.belt_2,
        equipment.belt_3,
        equipment.shield,
        equipment.armor,
        equipment.boots,
        equipment.cloak,
        equipment.ring_1,
        equipment.ring_2,
        equipment.ring_3,
        equipment.ring_4,
    ]
    .into_iter()
    .flatten()
    {
        if !ids.contains(&item_id) {
            ids.push(item_id);
        }
    }
    ids
}

fn is_item_equipped(state: &GameState, item_id: u32) -> bool {
    equipped_item_ids(&state.player.equipment).into_iter().any(|equipped_id| equipped_id == item_id)
}

fn sync_pack_order(state: &mut GameState) {
    let existing_ids: Vec<u32> = state
        .player
        .pack_order
        .iter()
        .copied()
        .filter(|id| state.player.inventory.iter().any(|item| item.id == *id))
        .filter(|id| !is_item_equipped(state, *id))
        .collect();

    let mut rebuilt = existing_ids;
    for item in &state.player.inventory {
        if is_item_equipped(state, item.id) {
            continue;
        }
        if !rebuilt.contains(&item.id) {
            rebuilt.push(item.id);
        }
    }
    state.player.pack_order = rebuilt;
}

fn push_item_to_pack_front(state: &mut GameState, item_id: u32) {
    state.player.pack_order.retain(|id| *id != item_id);
    state.player.pack_order.insert(0, item_id);
}

fn remove_item_from_pack_order(state: &mut GameState, item_id: u32) {
    state.player.pack_order.retain(|id| *id != item_id);
}

fn first_pack_item_id(state: &GameState) -> Option<u32> {
    state
        .player
        .pack_order
        .iter()
        .copied()
        .find(|id| state.player.inventory.iter().any(|item| item.id == *id))
        .or_else(|| state.player.inventory.first().map(|item| item.id))
}

fn remove_item_by_id(state: &mut GameState, item_id: u32) -> Option<Item> {
    let idx = state.player.inventory.iter().position(|entry| entry.id == item_id)?;
    let removed = state.player.inventory.remove(idx);
    state.carry_burden = state.carry_burden.saturating_sub(item_burden(&removed)).max(0);
    unequip_item_id(&mut state.player.equipment, item_id);
    remove_item_from_pack_order(state, item_id);
    Some(removed)
}

fn equipment_effect_profile(state: &GameState) -> EquipmentEffectProfile {
    let mut profile = EquipmentEffectProfile::default();
    for item_id in equipped_item_ids(&state.player.equipment) {
        let Some(item) = state.player.inventory.iter().find(|entry| entry.id == item_id) else {
            continue;
        };
        match item.family {
            ItemFamily::Weapon => {
                profile.attack_min_bonus += (item.dmg / 8).max(1) + item.plus.max(0) / 2;
                profile.attack_max_bonus += (item.dmg / 4).max(1) + item.plus.max(0);
                profile.to_hit_bonus += (item.hit / 3).max(0) + item.plus.max(0) / 2;
            }
            ItemFamily::Armor => {
                profile.defense_bonus += item.dmg.max(0) + item.aux.max(0) / 2 + item.plus.max(0);
            }
            ItemFamily::Shield => {
                profile.defense_bonus += item.aux.max(0) + item.plus.max(0);
            }
            ItemFamily::Boots | ItemFamily::Cloak => {
                profile.defense_bonus += item.aux.max(0).max(item.plus.max(0));
            }
            ItemFamily::Ring => {
                profile.defense_bonus += item.plus.max(0);
            }
            _ => {}
        }

        match item.usef.as_str() {
            "I_PERM_PROTECTION" | "I_PERM_DEFLECT" | "I_DEFLECT" | "I_DEFEND" => {
                profile.defense_bonus += 2;
                profile.block_bonus += 1;
            }
            "I_VICTRIX" => {
                profile.attack_min_bonus += 5;
                profile.attack_max_bonus += 10;
                profile.to_hit_bonus += 4;
            }
            "I_LIGHTSABRE" => {
                profile.attack_min_bonus += 3;
                profile.attack_max_bonus += 6;
                profile.to_hit_bonus += 3;
            }
            "I_DEMONBLADE" | "I_JUGGERNAUT" => {
                profile.attack_min_bonus += 4;
                profile.attack_max_bonus += 8;
                profile.to_hit_bonus += 2;
            }
            "I_PERM_FIRE_RESIST" => {
                profile.fire_resist_bonus += 2;
            }
            "I_PERM_POISON_RESIST" => {
                profile.poison_resist_bonus += 2;
            }
            "I_PERM_ENERGY_RESIST" | "I_MACE_DISRUPT" => {
                profile.magic_resist_bonus += 2;
            }
            "I_IMMUNE" | "I_PERM_NEGIMMUNE" => {
                profile.grants_poison_immunity = true;
            }
            "I_FEAR_RESIST" | "I_PERM_FEAR_RESIST" => {
                profile.grants_fear_immunity = true;
            }
            "I_PERM_REGENERATE" | "I_REGENERATE" => {
                profile.regen_per_turn = profile.regen_per_turn.max(1);
            }
            "I_PERM_BURDEN" => {
                profile.carry_capacity_delta -= 4;
            }
            "I_HOLDING" => {
                profile.carry_capacity_delta += 8;
            }
            _ => {}
        }
    }
    profile
}

fn effective_inventory_capacity(state: &GameState) -> usize {
    let profile = equipment_effect_profile(state);
    let base = state.player.inventory_capacity as i32;
    (base + profile.carry_capacity_delta).clamp(1, 64) as usize
}

fn identify_inventory_items(state: &mut GameState) -> usize {
    let mut identified = 0usize;
    for entry in &mut state.player.inventory {
        if !entry.known || !entry.used {
            entry.known = true;
            entry.used = true;
            identified += 1;
        }
    }
    identified
}

fn charge_first_stick(state: &mut GameState, amount: i32) -> bool {
    if let Some(stick) =
        state.player.inventory.iter_mut().find(|entry| entry.family == ItemFamily::Stick)
    {
        stick.charge = (stick.charge + amount).clamp(0, 99);
        return true;
    }
    false
}

fn transmutation_target_index(
    state: &GameState,
    preferred_family: Option<ItemFamily>,
) -> Option<usize> {
    if let Some(family) = preferred_family
        && let Some(idx) = state.player.inventory.iter().position(|entry| entry.family == family)
    {
        return Some(idx);
    }
    for equipped_id in equipped_item_ids(&state.player.equipment) {
        if let Some(idx) = state.player.inventory.iter().position(|entry| entry.id == equipped_id) {
            return Some(idx);
        }
    }
    state.player.inventory.iter().position(|entry| entry.family != ItemFamily::Cash)
}

fn destroy_inventory_item_by_id(state: &mut GameState, item_id: u32) -> Option<String> {
    let idx = state.player.inventory.iter().position(|entry| entry.id == item_id)?;
    let removed = state.player.inventory.remove(idx);
    state.carry_burden = state.carry_burden.saturating_sub(item_burden(&removed)).max(0);
    unequip_item_id(&mut state.player.equipment, item_id);
    remove_item_from_pack_order(state, item_id);
    Some(removed.name)
}

fn enchant_item_with_risk(
    state: &mut GameState,
    preferred_family: Option<ItemFamily>,
    delta: i32,
    events: &mut Vec<Event>,
) -> String {
    let Some(idx) = transmutation_target_index(state, preferred_family) else {
        return "You feel unlucky.".to_string();
    };

    let item_id = state.player.inventory[idx].id;
    let item_name = state.player.inventory[idx].name.clone();
    let item_plus = state.player.inventory[idx].plus;
    let is_artifact = state.player.inventory[idx].family == ItemFamily::Artifact;

    if delta < 0 {
        let item = &mut state.player.inventory[idx];
        if item.blessing < 0 || (is_artifact && (state.clock.turn % 3 != 0)) {
            return format!("{item_name} glows, but the aura flickers out.");
        }
        item.plus = 0;
        item.charge = -1;
        item.usef = "I_NOTHING".to_string();
        item.known = true;
        return format!("{item_name} radiates an aura of mundanity.");
    }

    if is_artifact && state.clock.turn % 2 == 0 {
        let backlash = (state.player.stats.max_hp / 4).max(4);
        let applied = state.player.stats.apply_damage(backlash);
        if !state.player.stats.is_alive() {
            mark_player_defeated(state, format!("artifact backlash from {item_name}"), events);
        }
        return format!("Potent enchantment backlash from {item_name} deals {applied} damage.");
    }

    if item_plus > 12 {
        let blast = (item_plus.max(1) * 3).clamp(8, 90);
        let applied = state.player.stats.apply_damage(blast);
        let _ = destroy_inventory_item_by_id(state, item_id);
        if !state.player.stats.is_alive() {
            mark_player_defeated(state, "an enchantment explosion", events);
        }
        return format!("The force of enchantment makes {item_name} explode for {applied} damage.");
    }

    let item = &mut state.player.inventory[idx];
    item.plus = (item.plus + delta + 1).clamp(-9, 25);
    item.blessing = item.blessing.max(1);
    item.known = true;
    if item.charge > -1 {
        item.charge = (item.charge + ((delta + 1) * 3)).clamp(0, 999);
    }
    if item.family == ItemFamily::Weapon {
        item.dmg = (item.dmg + delta.max(0) + 1).clamp(0, 10_000);
        item.hit = (item.hit + delta.max(0) + 1).clamp(-100, 10_000);
    }
    "The item shines with unstable enchantment.".to_string()
}

fn bless_item_with_risk(state: &mut GameState, blessing: i32, _events: &mut Vec<Event>) -> String {
    let Some(idx) = transmutation_target_index(state, None) else {
        return "Blessing finds no target.".to_string();
    };

    if blessing < 0 {
        let item = &mut state.player.inventory[idx];
        item.blessing -= 2;
        if item.blessing < 0 {
            item.plus = item.plus.abs().saturating_sub(1);
        }
        return format!("A foul odor rises from {}.", item.name);
    }

    let item_id = state.player.inventory[idx].id;
    let item_name = state.player.inventory[idx].name.clone();
    let item_blessing = state.player.inventory[idx].blessing;
    if item_blessing < -(blessing + 1) {
        return format!("{item_name} is evil enough to resist the blessing.");
    }
    if item_blessing < -1 {
        let _ = destroy_inventory_item_by_id(state, item_id);
        return format!("{item_name} disintegrates under the holy aura.");
    }
    if item_blessing < blessing + 1 {
        let item = &mut state.player.inventory[idx];
        item.blessing += 1;
        item.plus = item.plus.abs() + 1;
        item.known = true;
        return format!("{item_name} now seems affected by afflatus.");
    }
    "The hierolux fades without appreciable effect.".to_string()
}

fn dispel_or_decurse_with_branching(state: &mut GameState, potency: i32) -> String {
    if potency >= 0 {
        if let Some(idx) =
            state.player.inventory.iter().position(|item| item.used && item.blessing < 0)
        {
            let item_name = state.player.inventory[idx].name.clone();
            let item_blessing = state.player.inventory[idx].blessing;
            if potency + 1 + item_blessing >= 0 {
                state.player.inventory[idx].blessing = 0;
                return format!("You hear a sighing sound from {item_name}.");
            }
            return format!("You hear dark laughter from {item_name}.");
        }
        state.status_effects.clear();
        return "Active magical effects were dispelled.".to_string();
    }

    for effect in &mut state.status_effects {
        if effect.remaining_turns < 1000 {
            effect.remaining_turns = effect.remaining_turns.min(1);
        }
    }
    "A smell of ozone and positive ions fills the air.".to_string()
}

fn enchant_equipment_piece(
    state: &mut GameState,
    family: ItemFamily,
    plus: i32,
    events: &mut Vec<Event>,
) -> bool {
    let note = enchant_item_with_risk(state, Some(family), plus, events);
    !note.contains("unlucky")
}

fn apport_nearby_item(state: &mut GameState, radius: i32) -> Option<String> {
    let player_pos = state.player.position;
    let idx = state
        .ground_items
        .iter()
        .position(|ground| ground.position.manhattan_distance(player_pos) <= radius)?;
    let mut pulled = state.ground_items.remove(idx).item;
    pulled.known = true;
    if state.player.inventory.len() < effective_inventory_capacity(state) {
        state.carry_burden = state.carry_burden.saturating_add(item_burden(&pulled));
        let note = pulled.name.clone();
        auto_equip_item(state, &pulled);
        state.player.inventory.push(pulled);
        return Some(note);
    }
    let note = pulled.name.clone();
    state.ground_items.push(GroundItem { position: player_pos, item: pulled });
    Some(note)
}

fn drop_all_portcullises(state: &mut GameState) -> bool {
    if state.world_mode != WorldMode::DungeonCity {
        return false;
    }
    let mut changed = false;
    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            let pos = Position { x, y };
            let Some(cell) = state.tile_site_at(pos) else {
                continue;
            };
            if (cell.flags & TILE_FLAG_PORTCULLIS) == 0 {
                continue;
            }
            let mut flags = cell.flags;
            if (flags & TILE_FLAG_BLOCK_MOVE) == 0 {
                flags |= TILE_FLAG_BLOCK_MOVE;
                changed = true;
            }
            flags &= !TILE_FLAG_OPENED_DOOR;
            set_site_flags_at(state, pos, flags);
            if state.map_glyph_at(pos) == '/' {
                let _ = state.set_map_glyph_at(pos, 'P');
                changed = true;
            }
        }
    }
    changed
}

fn raise_all_portcullises(state: &mut GameState) -> bool {
    if state.world_mode != WorldMode::DungeonCity {
        return false;
    }
    let mut changed = false;
    for y in 0..state.bounds.height {
        for x in 0..state.bounds.width {
            let pos = Position { x, y };
            let Some(cell) = state.tile_site_at(pos) else {
                continue;
            };
            if (cell.flags & TILE_FLAG_PORTCULLIS) == 0 {
                continue;
            }
            let mut flags = cell.flags;
            if (flags & TILE_FLAG_BLOCK_MOVE) != 0 {
                flags &= !TILE_FLAG_BLOCK_MOVE;
                changed = true;
            }
            flags |= TILE_FLAG_OPENED_DOOR;
            set_site_flags_at(state, pos, flags);
            if state.map_glyph_at(pos) == 'P' {
                let _ = state.set_map_glyph_at(pos, '/');
                changed = true;
            }
        }
    }
    changed
}

fn open_adjacent_barrier(state: &mut GameState) -> Option<String> {
    if state.world_mode != WorldMode::DungeonCity {
        return None;
    }
    let candidates = [
        Position { x: state.player.position.x, y: state.player.position.y - 1 },
        Position { x: state.player.position.x + 1, y: state.player.position.y },
        Position { x: state.player.position.x, y: state.player.position.y + 1 },
        Position { x: state.player.position.x - 1, y: state.player.position.y },
    ];
    for pos in candidates {
        if !state.bounds.contains(pos) {
            continue;
        }
        let glyph = state.map_glyph_at(pos);
        let mut flags = state.tile_site_at(pos).map(|cell| cell.flags).unwrap_or(0);
        let closed_door = glyph == '-' || glyph == 'D' || glyph == 'J';
        let closed_portcullis =
            (flags & TILE_FLAG_PORTCULLIS) != 0 && (flags & TILE_FLAG_BLOCK_MOVE) != 0;
        if !closed_door && !closed_portcullis {
            continue;
        }
        let _ = state.set_map_glyph_at(pos, '/');
        flags &= !TILE_FLAG_BLOCK_MOVE;
        flags |= TILE_FLAG_OPENED_DOOR;
        set_site_flags_at(state, pos, flags);
        return Some(format!("opened barrier at ({}, {})", pos.x, pos.y));
    }
    None
}

fn count_detected_monsters(state: &GameState, radius: i32) -> usize {
    state
        .monsters
        .iter()
        .filter(|monster| monster.position.manhattan_distance(state.player.position) <= radius)
        .count()
}

fn count_detected_objects(state: &GameState, radius: i32) -> usize {
    state
        .ground_items
        .iter()
        .filter(|ground| ground.position.manhattan_distance(state.player.position) <= radius)
        .count()
}

fn apply_item_usef_effect(state: &mut GameState, item: &Item, events: &mut Vec<Event>) -> String {
    let begin_item_projectile =
        |state: &mut GameState,
         item: &Item,
         mode: ProjectileKind,
         label: &str,
         damage_min: i32,
         damage_max: i32,
         damage_type: ProjectileDamageType| {
            let source_token = if item.family == ItemFamily::Stick { "z" } else { "a" };
            let action = PendingProjectileAction {
                source_token: source_token.to_string(),
                turn_minutes: estimate_legacy_turn_minutes(
                    source_token,
                    state.world_mode,
                    state.options.searchnum,
                ),
                mode,
                item_id: None,
                item_name: label.to_string(),
                hit_bonus: statmod(state.attributes.iq.max(1))
                    + statmod(state.attributes.power.max(1)),
                damage_bonus: item.plus.max(0),
                damage_min,
                damage_max,
                damage_type,
                max_range: 12,
                allows_drop: false,
            };
            let _ = begin_targeting_interaction(state, action);
            format!("{label}: choose a target.")
        };

    match item.usef.as_str() {
        "I_HEAL" => {
            state.player.stats.hp = (state.player.stats.hp + 12).min(state.player.stats.max_hp);
            "healing effect applied".to_string()
        }
        "I_CURE" | "I_NEUTRALIZE_POISON" => {
            consume_status(state, "poison");
            "poison cured".to_string()
        }
        "I_RESTORE" => {
            state.player.stats.hp = state.player.stats.max_hp;
            state.spellbook.mana = state.spellbook.max_mana;
            "vitality and mana restored".to_string()
        }
        "I_SPEED" | "I_PERM_SPEED" => {
            push_or_refresh_status(&mut state.status_effects, "haste", 12, 1);
            "speed increased".to_string()
        }
        "I_INVISIBLE" | "I_PERM_INVISIBLE" | "I_HIDE" => {
            push_or_refresh_status(&mut state.status_effects, "invisible", 10, 1);
            "invisibility effect applied".to_string()
        }
        "I_BREATHING" | "I_PERM_BREATHING" => {
            push_or_refresh_status(&mut state.status_effects, "breathing", 16, 1);
            "breathing adaptation granted".to_string()
        }
        "I_ACCURACY" | "I_PERM_ACCURACY" => {
            state.player.stats.attack_max = (state.player.stats.attack_max + 1).clamp(1, 60);
            "accuracy improved".to_string()
        }
        "I_PERM_PROTECTION" | "I_PERM_DEFLECT" => {
            state.player.stats.defense = (state.player.stats.defense + 1).clamp(0, 40);
            "defense improved".to_string()
        }
        "I_PERM_STRENGTH" => {
            state.attributes.strength = (state.attributes.strength + 2).clamp(3, 30);
            recompute_derived_combat_and_mana_from_attributes(state);
            "strength increased".to_string()
        }
        "I_PERM_AGILITY" => {
            state.attributes.agility = (state.attributes.agility + 2).clamp(3, 30);
            recompute_derived_combat_and_mana_from_attributes(state);
            "agility increased".to_string()
        }
        "I_AUGMENT" => {
            state.attributes.strength = (state.attributes.strength + 1).clamp(3, 30);
            state.attributes.iq = (state.attributes.iq + 1).clamp(3, 30);
            state.attributes.dexterity = (state.attributes.dexterity + 1).clamp(3, 30);
            state.attributes.agility = (state.attributes.agility + 1).clamp(3, 30);
            state.attributes.constitution = (state.attributes.constitution + 1).clamp(3, 30);
            state.attributes.power = (state.attributes.power + 1).clamp(3, 30);
            recompute_derived_combat_and_mana_from_attributes(state);
            "attributes augmented".to_string()
        }
        "I_ALERT" => {
            consume_status(state, "poison");
            consume_status(state, "immobile");
            push_or_refresh_status(&mut state.status_effects, "alert", 8, 1);
            "you feel sharply alert".to_string()
        }
        "I_ANTIOCH" => spell_damage_radius(state, events, 5, 40, "holy hand-grenade blast"),
        "I_APPORT" => {
            if let Some(name) = apport_nearby_item(state, 6) {
                format!("apported `{name}` into reach")
            } else {
                "apportation failed: no nearby object".to_string()
            }
        }
        "I_AZOTH" => {
            state.player.stats.hp = state.player.stats.max_hp;
            state.spellbook.mana = state.spellbook.max_mana;
            state.resistances.fire = state.resistances.fire.max(2);
            state.resistances.poison = state.resistances.poison.max(2);
            state.resistances.magic = state.resistances.magic.max(2);
            "azoth rebalanced body and spirit".to_string()
        }
        "I_BLESS" => {
            let bless_note = bless_item_with_risk(state, 1, events);
            state.progression.deity_favor += 1;
            state.progression.law_chaos_score += 1;
            bless_note
        }
        "I_BOOTS_7LEAGUE" => {
            spell_shift_player(state, 12, 0);
            push_or_refresh_status(&mut state.status_effects, "haste", 6, 2);
            "you stride seven leagues in a blink".to_string()
        }
        "I_BOOTS_JUMPING" => {
            spell_shift_player(state, 3, 0);
            "you vault across the battlefield".to_string()
        }
        "I_CHAOS" => {
            state.progression.alignment = Alignment::Chaotic;
            state.progression.law_chaos_score -= 5;
            "chaotic currents surge through you".to_string()
        }
        "I_CHARGE" => {
            if charge_first_stick(state, 6) {
                "a magical stick was recharged".to_string()
            } else {
                "charge dissipated: no stick to receive it".to_string()
            }
        }
        "I_CLAIRVOYANCE" => {
            reveal_map_for_wizard(state);
            "clairvoyant vision reveals the current map".to_string()
        }
        "I_CORPSE" => {
            if item.blessing < 0 {
                let applied = state.player.stats.apply_damage(4);
                push_or_refresh_status(&mut state.status_effects, "poison", 6, 1);
                if !state.player.stats.is_alive() {
                    mark_player_defeated(state, "tainted corpse", events);
                }
                format!("putrid corpse dealt {applied} damage")
            } else {
                state.food += item.aux.max(2);
                state.player.stats.hp = (state.player.stats.hp + 1).min(state.player.stats.max_hp);
                "corpse consumption restored some nutrition".to_string()
            }
        }
        "I_CRYSTAL" => {
            state.spellbook.max_mana = (state.spellbook.max_mana + 10).clamp(1, 500);
            state.spellbook.mana = (state.spellbook.mana + 20).min(state.spellbook.max_mana);
            state.resistances.magic = state.resistances.magic.max(2);
            "crystal lattice amplifies magical reserve".to_string()
        }
        "I_DEATH" => spell_remove_nearest(state, events, 8, "death magic"),
        "I_DEFLECT" => {
            push_or_refresh_status(&mut state.status_effects, "block_bonus", 4, 2);
            "deflective field shimmers into place".to_string()
        }
        "I_DEMONBLADE" => {
            state.progression.alignment = Alignment::Chaotic;
            state.progression.law_chaos_score -= 2;
            state.player.stats.attack_max = (state.player.stats.attack_max + 4).clamp(1, 120);
            "demonblade thirst empowers brutal strikes".to_string()
        }
        "I_DISPEL" => dispel_or_decurse_with_branching(state, 1),
        "I_DISPLACE" | "I_PERM_DISPLACE" => {
            push_or_refresh_status(&mut state.status_effects, "displaced", 12, 1);
            "your outline slips away from certainty".to_string()
        }
        "I_ENCHANT" | "I_ENCHANTMENT" => {
            let _ = enchant_equipment_piece(state, ItemFamily::Weapon, 1, events);
            let _ = enchant_equipment_piece(state, ItemFamily::Armor, 1, events);
            enchant_item_with_risk(state, None, 1, events)
        }
        "I_FEAR_RESIST" => {
            state.immunities.fear = true;
            "fear resistance granted".to_string()
        }
        "I_FLUX" => {
            spell_shift_player(state, 7, 4);
            "reality flux tossed you across space".to_string()
        }
        "I_HELM" => {
            state.player.stats.defense = (state.player.stats.defense + 1).clamp(0, 80);
            state.immunities.sleep = true;
            "the helm steadies your senses".to_string()
        }
        "I_HERO" | "I_PERM_HERO" => {
            push_or_refresh_status(&mut state.status_effects, "heroism", 10, 2);
            state.player.stats.attack_max = (state.player.stats.attack_max + 2).clamp(1, 90);
            state.player.stats.defense = (state.player.stats.defense + 1).clamp(0, 80);
            "heroic vigor floods your limbs".to_string()
        }
        "I_HINT" => {
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.saturating_add(1);
            "a cryptic hint points the next step".to_string()
        }
        "I_HOLDING" => {
            state.player.pack_capacity = (state.player.pack_capacity + 4).min(64);
            state.player.inventory_capacity = (state.player.inventory_capacity + 4).min(64);
            format!("pack capacity increased to {}", state.player.inventory_capacity)
        }
        "I_ID" => {
            let identified = identify_inventory_items(state);
            format!("identified {identified} carried item(s)")
        }
        "I_ILLUMINATE" | "I_PERM_ILLUMINATE" => {
            reveal_map_for_wizard(state);
            push_or_refresh_status(&mut state.status_effects, "truesight", 12, 1);
            "illumination exposed hidden pathways".to_string()
        }
        "I_IMMUNE" => {
            state.immunities.poison = true;
            state.immunities.fear = true;
            state.immunities.sleep = true;
            state.resistances.fire = state.resistances.fire.max(2);
            state.resistances.magic = state.resistances.magic.max(2);
            "broad-spectrum immunity engaged".to_string()
        }
        "I_JANE_T" => {
            state.progression.quest_state = LegacyQuestState::Active;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.saturating_add(1);
            "Jane's token nudges the quest onward".to_string()
        }
        "I_JUGGERNAUT" => {
            push_or_refresh_status(&mut state.status_effects, "juggernaut", 10, 2);
            state.player.stats.attack_max = (state.player.stats.attack_max + 3).clamp(1, 120);
            state.player.stats.defense = (state.player.stats.defense + 2).clamp(0, 80);
            "juggernaut momentum makes you relentless".to_string()
        }
        "I_KEY" | "I_PICK" => open_adjacent_barrier(state)
            .unwrap_or_else(|| "no adjacent barrier yielded".to_string()),
        "I_RAISE_PORTCULLIS" => {
            if raise_all_portcullises(state) {
                "you hear the portcullis rise".to_string()
            } else {
                "no portcullis responds".to_string()
            }
        }
        "I_KNOWLEDGE" => {
            state.spellbook.mana = (state.spellbook.mana + 12).min(state.spellbook.max_mana);
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.saturating_add(1);
            "knowledge blooms in your mind".to_string()
        }
        "I_KOLWYNIA" => {
            push_or_refresh_status(&mut state.status_effects, "breathing", 18, 1);
            state.resistances.poison = state.resistances.poison.max(2);
            state.resistances.fire = state.resistances.fire.max(2);
            "Kolwynia's blessing shields against harsh realms".to_string()
        }
        "I_LAW" => {
            state.progression.alignment = Alignment::Lawful;
            state.progression.law_chaos_score += 5;
            "lawful conviction hardens your will".to_string()
        }
        "I_LEVITATION" | "I_PERM_LEVITATE" => {
            push_or_refresh_status(&mut state.status_effects, "levitate", 10, 1);
            "you drift above the ground".to_string()
        }
        "I_LIFE" => {
            state.player.stats.max_hp = (state.player.stats.max_hp + 6).clamp(1, 300);
            state.player.stats.hp = state.player.stats.max_hp;
            consume_status(state, "poison");
            "life force surges and restores your vitality".to_string()
        }
        "I_MONDET" => {
            let detected = count_detected_monsters(state, 9);
            format!("monster detection reports {detected} nearby threat(s)")
        }
        "I_NORMAL_ARMOR" | "I_NORMAL_SHIELD" | "I_NORMAL_WEAPON" => {
            "mundane equipment has no activated power".to_string()
        }
        "I_NO_OP" | "I_NOTHING" => "nothing happened".to_string(),
        "I_OBJDET" => {
            let detected = count_detected_objects(state, 9);
            format!("object detection reports {detected} nearby object(s)")
        }
        "I_PEPPER_FOOD" => {
            state.food += item.aux.max(2);
            push_or_refresh_status(&mut state.status_effects, "haste", 4, 1);
            "spiced ration restores appetite and quickens blood".to_string()
        }
        "I_PERM_BURDEN" => {
            state.player.pack_capacity = state.player.pack_capacity.saturating_sub(2).max(4);
            state.player.inventory_capacity =
                state.player.inventory_capacity.saturating_sub(2).max(4);
            format!("you feel burdened (capacity now {})", state.player.inventory_capacity)
        }
        "I_PERM_REGENERATE" | "I_REGENERATE" => {
            push_or_refresh_status(&mut state.status_effects, "regen", 20, 2);
            "regenerative aura settles in".to_string()
        }
        "I_PERM_TRUESIGHT" | "I_TRUESIGHT" => {
            reveal_map_for_wizard(state);
            push_or_refresh_status(&mut state.status_effects, "truesight", 20, 1);
            "true sight pierces obfuscation".to_string()
        }
        "I_PLANES" => {
            ensure_country_bootstrap(state);
            state.activate_country_view();
            "planar drift carried you to the overworld".to_string()
        }
        "I_SCEPTRE" => {
            state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.max(2);
            "the sceptre answers with sovereign authority".to_string()
        }
        "I_SERENITY" => {
            consume_status(state, "fear");
            consume_status(state, "poison");
            push_or_refresh_status(&mut state.status_effects, "sanctuary", 8, 2);
            "serenity calms body and spirit".to_string()
        }
        "I_SLEEP_SELF" => {
            push_or_refresh_status(&mut state.status_effects, "immobile", 3, 1);
            "drowsiness overtakes you".to_string()
        }
        "I_SPELLS" => {
            state.spellbook.max_mana = (state.spellbook.max_mana + 5).clamp(1, 500);
            state.spellbook.mana = (state.spellbook.mana + 15).min(state.spellbook.max_mana);
            "arcane formulae settle into memory".to_string()
        }
        "I_STARGEM" => {
            state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.max(2);
            "stargem radiance binds itself to your quest".to_string()
        }
        "I_TELEPORT" | "I_WARP" => {
            spell_shift_player(state, 9, 5);
            "space folds around you".to_string()
        }
        "I_TRAP" => {
            let (note, _) = disarm_adjacent_trap(state, events);
            note
        }
        "I_FOOD" | "I_LEMBAS" | "I_STIM" | "I_POW" => {
            state.food += item.aux.max(1);
            state.player.stats.hp = (state.player.stats.hp + 2).min(state.player.stats.max_hp);
            "nutrition restored".to_string()
        }
        "I_POISON_FOOD" => {
            let damage = (2 + item.aux.abs()).clamp(2, 18);
            let applied = state.player.stats.apply_damage(damage);
            push_or_refresh_status(&mut state.status_effects, "poison", 10, 1);
            if !state.player.stats.is_alive() {
                mark_player_defeated(state, "poisoned food", events);
            }
            format!("poisoned meal dealt {applied} damage")
        }
        "I_ACQUIRE" => {
            let kind =
                if state.wizard.enabled { WishItemKind::Artifact } else { WishItemKind::Thing };
            let item_name = random_item_from_kind(state, kind)
                .or_else(|| random_item_from_kind(state, WishItemKind::Potion))
                .unwrap_or_else(|| "food ration".to_string());
            let result = add_item_to_inventory_or_ground(state, item_name, events);
            format!("acquirement resolved ({result})")
        }
        "I_WISH" => {
            let item_name = random_item_from_kind(state, WishItemKind::Artifact)
                .or_else(|| random_item_from_kind(state, WishItemKind::Weapon))
                .unwrap_or_else(|| "food ration".to_string());
            let result = add_item_to_inventory_or_ground(state, item_name, events);
            format!("wish resolved ({result})")
        }
        "I_FIREBOLT" => begin_item_projectile(
            state,
            item,
            ProjectileKind::FireBolt,
            "firebolt",
            6,
            14,
            ProjectileDamageType::Flame,
        ),
        "I_LBOLT" => begin_item_projectile(
            state,
            item,
            ProjectileKind::LightningBolt,
            "lightning bolt",
            6,
            16,
            ProjectileDamageType::Electricity,
        ),
        "I_MISSILE" => begin_item_projectile(
            state,
            item,
            ProjectileKind::MagicMissile,
            "magic missile",
            6,
            8,
            ProjectileDamageType::Magic,
        ),
        "I_SLEEP_OTHER" => spell_mark_nearest_as_skirmisher(state, 6, "target falls asleep"),
        "I_FIREBALL" => spell_damage_radius(state, events, 3, 24, "fireball"),
        "I_LBALL" => spell_damage_radius(state, events, 3, 20, "ball lightning"),
        "I_SNOWBALL" => spell_damage_radius(state, events, 3, 16, "snowball"),
        "I_DISINTEGRATE" => spell_remove_nearest(state, events, 5, "disintegration"),
        "I_DISRUPT" => begin_item_projectile(
            state,
            item,
            ProjectileKind::MagicMissile,
            "disruption",
            5,
            18,
            ProjectileDamageType::Magic,
        ),
        "I_POLYMORPH" => spell_polymorph_nearest(state, 6),
        "I_SUMMON" => spell_summon_guardian(state),
        "I_FEAR" => {
            if state.immunities.fear {
                return "fear effect failed against your warded mind".to_string();
            }
            for monster in &mut state.monsters {
                if monster.position.manhattan_distance(state.player.position) <= 3 {
                    monster.behavior = MonsterBehavior::Skirmisher;
                }
            }
            "fear effect applied".to_string()
        }
        "I_PERM_FIRE_RESIST" => {
            state.resistances.fire = state.resistances.fire.max(2);
            "fire resistance increased".to_string()
        }
        "I_PERM_POISON_RESIST" => {
            state.resistances.poison = state.resistances.poison.max(2);
            "poison resistance increased".to_string()
        }
        "I_PERM_FEAR_RESIST" => {
            state.immunities.fear = true;
            "fear resistance granted".to_string()
        }
        "I_PERM_ENERGY_RESIST" => {
            state.resistances.magic = state.resistances.magic.max(2);
            "energy resistance increased".to_string()
        }
        "I_PERM_GAZE_IMMUNE" => {
            state.immunities.sleep = true;
            "gaze-like compulsions resisted".to_string()
        }
        "I_PERM_NEGIMMUNE" => {
            state.immunities.poison = true;
            "negative-energy effects dampened".to_string()
        }
        "I_PERM_KNOWLEDGE" => {
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.saturating_add(1);
            "self-knowledge insight gained".to_string()
        }
        "I_LIGHTSABRE" => {
            state.player.stats.attack_max = (state.player.stats.attack_max + 3).clamp(1, 80);
            "lightsabre hum sharpens your strikes".to_string()
        }
        "I_MACE_DISRUPT" => {
            state.resistances.magic = state.resistances.magic.max(3);
            "disruption aura empowered".to_string()
        }
        "I_DEFEND" => {
            state.player.stats.defense = (state.player.stats.defense + 2).clamp(0, 60);
            "defensive aura raised".to_string()
        }
        "I_VICTRIX" => {
            state.player.stats.attack_max = (state.player.stats.attack_max + 5).clamp(1, 90);
            "Victrix answers with unmatched force".to_string()
        }
        "I_DESECRATE" => {
            state.progression.law_chaos_score -= 3;
            state.progression.deity_favor -= 2;
            "desecrating force bends alignment toward chaos".to_string()
        }
        "I_SYMBOL" => {
            state.progression.deity_favor += 2;
            "holy symbol channels divine favor".to_string()
        }
        "I_ORBMASTERY" | "I_ORBFIRE" | "I_ORBWATER" | "I_ORBEARTH" | "I_ORBAIR" | "I_ORBDEAD" => {
            state.progression.quest_state = LegacyQuestState::ArtifactRecovered;
            state.progression.quest_steps_completed =
                state.progression.quest_steps_completed.max(2);
            "orb power resonates through the realm".to_string()
        }
        _ => {
            if item.usef.is_empty() {
                "no explicit item effect".to_string()
            } else {
                format!("unrecognized item effect `{}`", item.usef)
            }
        }
    }
}

fn push_or_refresh_status(
    effects: &mut Vec<StatusEffect>,
    id: &str,
    remaining_turns: u32,
    magnitude: i32,
) {
    if let Some(existing) = effects.iter_mut().find(|effect| effect.id == id) {
        existing.remaining_turns = remaining_turns;
        existing.magnitude = magnitude;
        return;
    }
    effects.push(StatusEffect { id: id.to_string(), remaining_turns, magnitude });
}

fn status_magnitude(state: &GameState, id: &str) -> i32 {
    state.status_effects.iter().find(|effect| effect.id == id).map(|e| e.magnitude).unwrap_or(0)
}

fn consume_status(state: &mut GameState, id: &str) {
    state.status_effects.retain(|effect| effect.id != id);
}

fn mark_player_defeated(state: &mut GameState, source: impl Into<String>, events: &mut Vec<Event>) {
    if state.status != SessionStatus::InProgress {
        return;
    }
    let source = source.into();
    state.status = SessionStatus::Lost;
    state.death_source = Some(source.clone());
    state.log.push("You are defeated.".to_string());
    state.log.push(format!("Killed by {source}."));
    events.push(Event::PlayerDefeated);
}

fn next_combat_step(state: &mut GameState) -> CombatStep {
    if state.combat_sequence.is_empty() {
        state.combat_sequence = default_combat_sequence();
    }
    let idx = state.combat_sequence_cursor % state.combat_sequence.len();
    let step = state.combat_sequence[idx].clone();
    state.combat_sequence_cursor = (state.combat_sequence_cursor + 1) % state.combat_sequence.len();
    step
}

fn resolve_attack_command<R: RandomSource>(
    state: &mut GameState,
    direction: Direction,
    rng: &mut R,
    events: &mut Vec<Event>,
) {
    let combat_step = next_combat_step(state);
    if matches!(combat_step.maneuver, CombatManeuver::Block | CombatManeuver::Riposte) {
        let block_magnitude = if combat_step.maneuver == CombatManeuver::Riposte { 1 } else { 2 };
        push_or_refresh_status(&mut state.status_effects, "block_bonus", 2, block_magnitude);
        if combat_step.maneuver == CombatManeuver::Riposte {
            push_or_refresh_status(&mut state.status_effects, "riposte_ready", 2, 2);
        }
        state.log.push(format!(
            "Combat step prepared: {:?} {:?}.",
            combat_step.maneuver, combat_step.line
        ));
        events.push(Event::LegacyHandled {
            token: "F".to_string(),
            note: format!("{:?} {:?} stance prepared", combat_step.maneuver, combat_step.line),
            fully_modeled: true,
        });
        return;
    }

    let profile = equipment_effect_profile(state);
    let effective_attack_min =
        (state.player.stats.attack_min + profile.attack_min_bonus).clamp(1, 400);
    let effective_attack_max = (state.player.stats.attack_max + profile.attack_max_bonus)
        .max(effective_attack_min + 1)
        .clamp(effective_attack_min + 1, 500);

    let target_pos = state.player.position.offset(direction);
    if let Some(monster_index) = monster_index_at(state, target_pos) {
        let (monster_id, monster_name, monster_faction, damage_done, remaining_hp, defeated) = {
            let monster = &mut state.monsters[monster_index];
            let rolled = rng.range_inclusive_i32(effective_attack_min, effective_attack_max);
            let maneuver_bonus = if combat_step.maneuver == CombatManeuver::Lunge { 2 } else { 0 };
            let line_bonus = match combat_step.line {
                CombatLine::High => 1,
                CombatLine::Center => 0,
                CombatLine::Low => 1,
            };
            let mitigated = (rolled + profile.to_hit_bonus + maneuver_bonus + line_bonus
                - monster.stats.defense)
                .max(1);
            let applied = monster.stats.apply_damage(mitigated);
            (
                monster.id,
                monster.name.clone(),
                monster.faction,
                applied,
                monster.stats.hp,
                !monster.stats.is_alive(),
            )
        };

        state.log.push(format!("You hit {} for {} damage.", monster_name, damage_done));
        events.push(Event::Attacked { monster_id, damage: damage_done, remaining_hp });
        match monster_faction {
            Faction::Law => {
                state.progression.law_chaos_score -= 1;
                state.legal_heat += 1;
            }
            Faction::Chaos => {
                state.progression.law_chaos_score += 1;
            }
            _ => {}
        }

        if defeated {
            let _ = remove_monster_with_drops(state, monster_index, events);
            state.monsters_defeated += 1;
            state.log.push(format!("{} is defeated.", monster_name));
            events.push(Event::MonsterDefeated { monster_id });
        }
    } else {
        state.log.push("You swing at empty space.".to_string());
        events.push(Event::AttackMissed { target: target_pos });
    }
}

fn estimate_action_points(command: &Command, world_mode: WorldMode) -> u16 {
    match command {
        Command::Wait => 100,
        Command::Move(_) => {
            if world_mode == WorldMode::Countryside {
                125
            } else {
                80
            }
        }
        Command::Attack(_) => 125,
        Command::Pickup => 90,
        Command::Drop { .. } => 70,
        Command::Legacy { token } => match token.trim() {
            "H" => 300,
            "s" => {
                if world_mode == WorldMode::Countryside {
                    200
                } else {
                    120
                }
            }
            "F" => 0,
            "Q" | "S" => 0,
            "." | "@" => 100,
            _ => 100,
        },
    }
}

fn apply_action_points(state: &mut GameState, command: &Command, events: &mut Vec<Event>) {
    let cost = estimate_action_points(command, state.world_mode);
    state.action_points_spent = state.action_points_spent.saturating_add(u64::from(cost));
    events.push(Event::ActionPointsSpent {
        cost,
        budget_per_turn: state.action_points_per_turn,
        total_spent: state.action_points_spent,
    });
}

fn apply_environment_effects<R: RandomSource>(
    state: &mut GameState,
    rng: &mut R,
    events: &mut Vec<Event>,
) {
    state.scheduler.environment_phase = state.scheduler.environment_phase.saturating_add(1);

    // Fire Propagation Logic
    let mut to_ignite = Vec::new();
    let mut to_burnout = Vec::new();

    let width = state.bounds.width;
    let height = state.bounds.height;

    for y in 0..height {
        for x in 0..width {
            let pos = Position { x, y };
            if let Some(cell) = state.tile_site_at(pos) {
                if (cell.flags & TILE_FLAG_BURNING) != 0 {
                    // Spread logic
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let neighbor_pos = Position { x: x + dx, y: y + dy };
                            if let Some(neighbor) = state.tile_site_at(neighbor_pos) {
                                // Spread to grass (")
                                if neighbor.glyph == '"' && (neighbor.flags & TILE_FLAG_BURNING) == 0 {
                                    // 30% chance to spread
                                    if rng.range_inclusive_i32(0, 99) < 30 {
                                        to_ignite.push(neighbor_pos);
                                    }
                                }
                            }
                        }
                    }

                    // Burn out logic: 10% chance
                    if rng.range_inclusive_i32(0, 99) < 10 {
                        to_burnout.push(pos);
                    }
                }
            }
        }
    }

    for pos in to_ignite {
        if let Some(cell) = state.tile_site_at_mut(pos) {
            cell.flags |= TILE_FLAG_BURNING;
        }
    }

    for pos in to_burnout {
        if let Some(cell) = state.tile_site_at_mut(pos) {
            cell.flags &= !TILE_FLAG_BURNING;
            cell.flags |= TILE_FLAG_BURNT;
            cell.glyph = '.'; // Turn to stone/ash
        }
    }

    let profile = equipment_effect_profile(state);
    let poison_resist = i32::from(state.resistances.poison.max(0)) + profile.poison_resist_bonus;
    let poison_immune = state.immunities.poison || profile.grants_poison_immunity;
    if let Some(trap) =
        state.traps.iter_mut().find(|trap| trap.armed && trap.position == state.player.position)
    {
        let reduced = (trap.damage - poison_resist).max(0);
        let applied = if poison_immune { 0 } else { state.player.stats.apply_damage(reduced) };
        let trap_effect_id = trap.effect_id.clone();
        state.log.push(format!(
            "Trap {} triggers for {} damage (effect {}).",
            trap.id, applied, trap.effect_id
        ));
        if applied > 0 && trap.effect_id == "poison" && !poison_immune {
            push_or_refresh_status(&mut state.status_effects, "poison", 3, 1);
        }
        trap.armed = false;
        events.push(Event::LegacyHandled {
            token: "trap".to_string(),
            note: format!("trap {} triggered", trap.id),
            fully_modeled: true,
        });
        if applied > 0 && !state.player.stats.is_alive() {
            mark_player_defeated(state, format!("{trap_effect_id} trap"), events);
        }
    }
}

fn apply_status_effects(state: &mut GameState, events: &mut Vec<Event>) {
    state.scheduler.timed_effect_phase = state.scheduler.timed_effect_phase.saturating_add(1);
    let profile = equipment_effect_profile(state);
    if profile.regen_per_turn > 0 && state.player.stats.hp < state.player.stats.max_hp {
        let regen = profile.regen_per_turn.max(0);
        state.player.stats.hp = (state.player.stats.hp + regen).min(state.player.stats.max_hp);
        state.log.push(format!("Equipped regeneration restores {regen} hp."));
    }
    if state.status_effects.is_empty() {
        return;
    }

    let mut expired = Vec::new();
    let mut defeat_source: Option<String> = None;
    let poison_resist = i32::from(state.resistances.poison.max(0)) + profile.poison_resist_bonus;
    let poison_immune = state.immunities.poison || profile.grants_poison_immunity;
    for effect in &mut state.status_effects {
        match effect.id.as_str() {
            "poison" => {
                if poison_immune {
                    continue;
                }
                let reduced = effect.magnitude.max(0) - poison_resist;
                let damage = reduced.max(0);
                if damage > 0 {
                    let applied = state.player.stats.apply_damage(damage);
                    state.log.push(format!("Poison deals {applied} damage."));
                    if applied > 0 && !state.player.stats.is_alive() && defeat_source.is_none() {
                        defeat_source = Some("poison".to_string());
                    }
                }
            }
            "regen" => {
                let heal = effect.magnitude.max(0);
                if heal > 0 {
                    state.player.stats.hp =
                        (state.player.stats.hp + heal).min(state.player.stats.max_hp);
                    state.log.push(format!("Regeneration restores {heal} hp."));
                }
            }
            _ => {}
        }

        if effect.remaining_turns > 0 {
            effect.remaining_turns -= 1;
        }

        events.push(Event::StatusTick {
            effect_id: effect.id.clone(),
            magnitude: effect.magnitude,
            remaining_turns: effect.remaining_turns,
        });

        if effect.remaining_turns == 0 {
            expired.push(effect.id.clone());
        }
    }

    if !expired.is_empty() {
        state.status_effects.retain(|effect| effect.remaining_turns > 0);
        for effect_id in expired {
            state.log.push(format!("Status `{effect_id}` has expired."));
            events.push(Event::StatusExpired { effect_id });
        }
    }

    if state.player.stats.hp <= 0 && state.status == SessionStatus::InProgress {
        let source = defeat_source.unwrap_or_else(|| "lingering wounds".to_string());
        mark_player_defeated(state, source, events);
    }
}

fn update_progression_from_combat(state: &mut GameState, events: &mut Vec<Event>) {
    let new_rank = if state.monsters_defeated >= 20 {
        4
    } else if state.monsters_defeated >= 12 {
        3
    } else if state.monsters_defeated >= 6 {
        2
    } else if state.monsters_defeated >= 2 {
        1
    } else {
        0
    };
    if new_rank > state.progression.guild_rank {
        state.progression.guild_rank = new_rank;
        state.log.push(format!("Guild rank advanced to {}.", state.progression.guild_rank));
        state.player.stats.attack_max = state.player.stats.attack_max.max(6 + i32::from(new_rank));
    }

    let priest_rank = if state.progression.deity_favor >= 24 {
        3
    } else if state.progression.deity_favor >= 12 {
        2
    } else if state.progression.deity_favor >= 5 {
        1
    } else {
        0
    };
    if priest_rank > state.progression.priest_rank {
        state.progression.priest_rank = priest_rank;
        state.log.push(format!("Priest rank advanced to {}.", state.progression.priest_rank));
    }

    state.progression.alignment = if state.progression.law_chaos_score >= 5 {
        Alignment::Lawful
    } else if state.progression.law_chaos_score <= -5 {
        Alignment::Chaotic
    } else {
        Alignment::Neutral
    };

    if state.progression.quest_state == LegacyQuestState::Completed
        && state.progression.guild_rank >= 4
        && state.progression.priest_rank >= 2
    {
        state.progression.total_winner_unlocked = true;
    }

    events.push(Event::ProgressionUpdated {
        guild_rank: state.progression.guild_rank,
        priest_rank: state.progression.priest_rank,
        alignment: state.progression.alignment,
    });
}

fn resolve_arena_round(state: &mut GameState, events: &mut Vec<Event>) {
    if state.environment != LegacyEnvironment::Arena || !state.progression.arena_match_active {
        return;
    }
    if !state.monsters.is_empty() {
        return;
    }

    state.progression.arena_match_active = false;
    let defeated = state.progression.arena_opponent;
    state.progression.arena_opponent = state.progression.arena_opponent.saturating_add(1);
    state.progression.quests.arena.xp =
        state.progression.quests.arena.xp.saturating_add(i64::from(defeated) * 25);

    let prize = 25 + i32::from(defeated) * 40;
    state.gold = state.gold.saturating_add(prize);
    if state.progression.arena_rank > 0
        && state.progression.arena_rank < 4
        && state.progression.arena_opponent > 5
        && state.progression.arena_opponent % 3 == 0
    {
        state.progression.arena_rank = state.progression.arena_rank.saturating_add(1);
    }
    state.progression.quests.arena.rank =
        state.progression.quests.arena.rank.max(i16::from(state.progression.arena_rank));

    state.log.push(format!("Arena victory! Prize awarded: {prize} gold."));
    if state.site_grid.iter().any(|cell| {
        (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
    }) {
        state.log.push(
            "The portcullis remains shut. Find and activate the opener to leave.".to_string(),
        );
    }
    events.push(Event::EconomyUpdated {
        source: "arena".to_string(),
        gold: state.gold,
        bank_gold: state.bank_gold,
    });
}

fn apply_explicit_victory_trigger(
    state: &mut GameState,
    trigger: VictoryTrigger,
    events: &mut Vec<Event>,
) {
    state.status = SessionStatus::Won;
    state.progression.victory_trigger = Some(trigger);
    if trigger == VictoryTrigger::ExplicitQuestCompletion {
        state.progression.quest_state = LegacyQuestState::Completed;
        state.progression.main_quest.stage = LegacyQuestState::Completed;
        state.progression.quest_steps_completed = state.progression.quest_steps_completed.max(4);
    }
    events.push(Event::VictoryAchieved);
}

fn is_adept_for_ending(progression: &PlayerProgression) -> bool {
    progression.adept_rank > 0 || progression.total_winner_unlocked
}

fn resolve_session_outcome(state: &mut GameState, events: &mut Vec<Event>) {
    let (ending, base_score) = match state.status {
        SessionStatus::InProgress => return,
        SessionStatus::Lost => (EndingKind::Defeat, (state.monsters_defeated as i64) * 5),
        SessionStatus::Won => {
            if state.progression.victory_trigger.is_none() {
                state.progression.victory_trigger = Some(VictoryTrigger::ExplicitQuestCompletion);
            }
            if is_adept_for_ending(&state.progression) {
                (EndingKind::TotalWinner, 5_000 + (state.monsters_defeated as i64) * 25)
            } else {
                (EndingKind::Victory, 2_000 + (state.monsters_defeated as i64) * 20)
            }
        }
    };

    let resource_score = i64::from(state.gold + state.bank_gold + state.food * 3);
    let quest_bonus = i64::from(state.progression.quest_steps_completed) * 100;
    let wizard_penalty = if state.wizard.enabled { -500 } else { 0 };
    state.progression.score = base_score + resource_score + quest_bonus + wizard_penalty;
    state.progression.ending = ending;
    state.progression.high_score_eligible = !state.wizard.enabled && state.wizard.scoring_allowed;
    events.push(Event::EndingResolved {
        ending,
        score: state.progression.score,
        high_score_eligible: state.progression.high_score_eligible,
    });
}

fn monster_index_at(state: &GameState, position: Position) -> Option<usize> {
    state.monsters.iter().position(|monster| monster.position == position)
}

fn ground_item_index_at(state: &GameState, position: Position) -> Option<usize> {
    state.ground_items.iter().position(|ground| ground.position == position)
}

fn is_monster_occupied_except(state: &GameState, position: Position, except_id: u64) -> bool {
    state.monsters.iter().any(|monster| monster.id != except_id && monster.position == position)
}

fn next_monster_step(monster: Position, player: Position) -> Position {
    let dx = player.x - monster.x;
    let dy = player.y - monster.y;

    if dx.abs() >= dy.abs() {
        Position { x: monster.x + dx.signum(), y: monster.y }
    } else {
        Position { x: monster.x, y: monster.y + dy.signum() }
    }
}

fn resolve_monster_projectile_strike<R: RandomSource>(
    state: &mut GameState,
    monster_idx: usize,
    equipment_profile: &EquipmentEffectProfile,
    rng: &mut R,
    events: &mut Vec<Event>,
) -> bool {
    let Some(monster) = state.monsters.get(monster_idx) else {
        return false;
    };
    let monster_id = monster.id;
    let monster_name = monster.name.clone();
    let monster_pos = monster.position;
    let attack_min = monster.stats.attack_min.max(1);
    let attack_max = monster.stats.attack_max.max(attack_min);
    let player_pos = state.player.position;
    let max_range = 6;
    if monster_pos.manhattan_distance(player_pos) > max_range {
        return false;
    }

    let final_pos = projectile_trace_to_target(state, monster_pos, player_pos, true);
    if final_pos != player_pos {
        state.log.push(format!("{monster_name} launches a magic missile, but it is blocked."));
        events.push(Event::LegacyHandled {
            token: "monster_projectile".to_string(),
            note: format!("monster {monster_id} projectile blocked"),
            fully_modeled: true,
        });
        return true;
    }

    let defense_total = state.player.stats.defense + equipment_profile.defense_bonus;
    let to_hit = attack_max + 6;
    if !legacy_hit_roll(to_hit, defense_total, rng) {
        state.log.push(format!("{monster_name} launches a magic missile, but misses."));
        events.push(Event::LegacyHandled {
            token: "monster_projectile".to_string(),
            note: format!("monster {monster_id} projectile miss"),
            fully_modeled: true,
        });
        return true;
    }

    let rolled = rng.range_inclusive_i32(attack_min, attack_max);
    let resolved_damage = (rolled - (defense_total / 2)).max(1);
    let damage = state.player.stats.apply_damage(resolved_damage);
    let remaining_hp = state.player.stats.hp;
    state.log.push(format!("{monster_name}'s magic missile hits you for {damage} damage."));
    events.push(Event::MonsterAttacked { monster_id, damage, remaining_hp });
    events.push(Event::LegacyHandled {
        token: "monster_projectile".to_string(),
        note: format!("monster {monster_id} projectile hit {damage}"),
        fully_modeled: true,
    });
    if !state.player.stats.is_alive() {
        mark_player_defeated(state, monster_name, events);
    }
    true
}

fn run_monster_turn<R: RandomSource>(state: &mut GameState, rng: &mut R, events: &mut Vec<Event>) {
    state.scheduler.monster_phase = state.scheduler.monster_phase.saturating_add(1);
    let equipment_profile = equipment_effect_profile(state);
    let monster_ids: Vec<u64> = state.monsters.iter().map(|m| m.id).collect();

    for monster_id in monster_ids {
        if state.status != SessionStatus::InProgress {
            break;
        }

        let Some(idx) = state.monsters.iter().position(|m| m.id == monster_id) else {
            continue;
        };

        let monster_pos = state.monsters[idx].position;
        let player_pos = state.player.position;
        let behavior = state.monsters[idx].behavior;
        let faction = state.monsters[idx].faction;
        let faction_hostile = match (faction, state.progression.alignment, behavior) {
            (Faction::Law, Alignment::Lawful, _) => false,
            (Faction::Chaos, Alignment::Chaotic, _) => false,
            (Faction::Neutral, _, MonsterBehavior::Social) => false,
            (Faction::Wild, _, MonsterBehavior::Social) => false,
            (Faction::Neutral, _, _) => true,
            (Faction::Wild, _, _) => true,
            _ => true,
        };

        if behavior == MonsterBehavior::Social && !faction_hostile {
            state
                .log
                .push(format!("{} keeps distance and observes you.", state.monsters[idx].name));
            events.push(Event::DialogueAdvanced {
                speaker: state.monsters[idx].name.clone(),
                quest_state: state.progression.quest_state,
            });
            continue;
        }

        if behavior == MonsterBehavior::Caster
            && faction_hostile
            && resolve_monster_projectile_strike(state, idx, &equipment_profile, rng, events)
        {
            continue;
        }

        if monster_pos.manhattan_distance(player_pos) == 1 && faction_hostile {
            let rolled = rng.range_inclusive_i32(
                state.monsters[idx].stats.attack_min,
                state.monsters[idx].stats.attack_max,
            );
            let block_bonus =
                status_magnitude(state, "block_bonus").max(0) + equipment_profile.block_bonus;
            let defense_total = state.player.stats.defense + equipment_profile.defense_bonus;
            let mitigated = (rolled - defense_total - block_bonus).max(1);
            let damage = state.player.stats.apply_damage(mitigated);
            let remaining_hp = state.player.stats.hp;
            let monster_name = state.monsters[idx].name.clone();

            state.log.push(format!("{} hits you for {} damage.", monster_name, damage));
            events.push(Event::MonsterAttacked { monster_id, damage, remaining_hp });
            if block_bonus > 0 {
                consume_status(state, "block_bonus");
            }

            let riposte_bonus = status_magnitude(state, "riposte_ready").max(0);
            if riposte_bonus > 0
                && let Some(riposte_idx) = state.monsters.iter().position(|m| m.id == monster_id)
            {
                let riposte_damage = (state.player.stats.attack_min
                    + equipment_profile.attack_min_bonus
                    + (equipment_profile.to_hit_bonus / 2)
                    + riposte_bonus
                    - state.monsters[riposte_idx].stats.defense)
                    .max(1);
                let applied = state.monsters[riposte_idx].stats.apply_damage(riposte_damage);
                let riposte_remaining = state.monsters[riposte_idx].stats.hp;
                events.push(Event::Attacked {
                    monster_id,
                    damage: applied,
                    remaining_hp: riposte_remaining,
                });
                if !state.monsters[riposte_idx].stats.is_alive() {
                    let _ = remove_monster_with_drops(state, riposte_idx, events);
                    state.monsters_defeated += 1;
                    events.push(Event::MonsterDefeated { monster_id });
                }
                consume_status(state, "riposte_ready");
            }

            if !state.player.stats.is_alive() {
                mark_player_defeated(state, monster_name, events);
                break;
            }
            continue;
        }

        let candidate = if behavior == MonsterBehavior::Skirmisher
            && monster_pos.manhattan_distance(player_pos) <= 2
        {
            Position {
                x: monster_pos.x - (player_pos.x - monster_pos.x).signum(),
                y: monster_pos.y - (player_pos.y - monster_pos.y).signum(),
            }
        } else {
            next_monster_step(monster_pos, player_pos)
        };
        let candidate_blocked = !state.bounds.contains(candidate)
            || !state.tile_is_walkable(candidate)
            || candidate == state.player.position
            || is_monster_occupied_except(state, candidate, monster_id);
        if candidate_blocked {
            continue;
        }

        state.monsters[idx].position = candidate;
        events.push(Event::MonsterMoved { monster_id, from: monster_pos, to: candidate });
    }
}

fn advance_time(state: &mut GameState, turn_minutes: u64, events: &mut Vec<Event>) {
    state.clock.turn += 1;
    state.clock.minutes += turn_minutes;
    events.push(Event::TurnAdvanced { turn: state.clock.turn, minutes: state.clock.minutes });
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::BTreeSet;

    struct FixedRng {
        rolls: Vec<i32>,
        index: usize,
    }

    impl FixedRng {
        fn new(rolls: Vec<i32>) -> Self {
            Self { rolls, index: 0 }
        }
    }

    impl RandomSource for FixedRng {
        fn range_inclusive_i32(&mut self, min: i32, max: i32) -> i32 {
            let value = self.rolls.get(self.index).copied().unwrap_or(min);
            self.index += 1;
            value.clamp(min, max)
        }
    }

    fn arena_test_site_definition() -> SiteMapDefinition {
        let width = 64usize;
        let height = 16usize;
        let mut rows = vec!["#".repeat(width); height];
        for y in 3..13 {
            let mut chars: Vec<char> = rows[y].chars().collect();
            for cell in chars.iter_mut().take(62).skip(2) {
                *cell = '.';
            }
            rows[y] = chars.into_iter().collect();
        }
        for y in [7usize, 8usize] {
            let mut chars: Vec<char> = rows[y].chars().collect();
            chars[0] = 'X';
            chars[1] = 'P';
            chars[2] = 'P';
            rows[y] = chars.into_iter().collect();
        }

        let mut site_grid = Vec::with_capacity(width * height);
        for row in &rows {
            for glyph in row.chars() {
                let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };
                match glyph {
                    'X' => {
                        cell.aux = SITE_AUX_EXIT_ARENA;
                    }
                    'P' => {
                        cell.flags |= TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
                    }
                    '#' => {
                        cell.flags |= TILE_FLAG_BLOCK_MOVE;
                    }
                    _ => {}
                }
                site_grid.push(cell);
            }
        }

        SiteMapDefinition {
            map_id: 1,
            level_index: 0,
            source: "test/arena.map".to_string(),
            environment: LegacyEnvironment::Arena,
            semantic: MapSemanticKind::Site,
            spawn: Position { x: 2, y: 7 },
            rows,
            site_grid,
        }
    }

    fn closed_portcullis_count(state: &GameState) -> usize {
        state
            .site_grid
            .iter()
            .filter(|cell| {
                (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
            })
            .count()
    }

    fn countryside_state(width: i32, height: i32, terrain: CountryTerrainKind) -> GameState {
        let mut state = GameState::new(MapBounds { width, height });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::Country;
        state.map_rows = vec![".".repeat(width as usize); height as usize];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); (width * height) as usize];
        state.country_grid = CountryGrid {
            width,
            height,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: terrain,
                    current_terrain: terrain,
                    aux: 0,
                    status: 0
                };
                (width * height) as usize
            ],
        };
        state
    }

    #[test]
    fn wait_advances_turn_and_time() {
        let mut state = GameState::default();
        let mut rng = FixedRng::new(vec![]);
        let out = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(out.turn, 1);
        assert_eq!(out.minutes, 6);
        assert_eq!(state.clock.turn, 1);
        assert_eq!(state.clock.minutes, 6);
    }

    #[test]
    fn movement_is_blocked_out_of_bounds() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 0, y: 0 };
        let mut rng = FixedRng::new(vec![]);

        let out = step(&mut state, Command::Move(Direction::West), &mut rng);
        assert_eq!(state.player.position, Position { x: 0, y: 0 });
        assert!(out.events.iter().any(|event| matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn guard_marker_spawns_interactive_guard_monster() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec![".G.".to_string(), "...".to_string(), "...".to_string()];
        state.site_grid = vec![TileSiteCell::default(); 9];
        let spawned = state.spawn_guard_monsters_from_markers();

        assert_eq!(spawned, 1);
        assert_eq!(state.map_glyph_at(Position { x: 1, y: 0 }), '.');
        assert!(state.tile_is_walkable(Position { x: 1, y: 0 }));
        assert!(state.monsters.iter().any(|monster| monster.position == Position { x: 1, y: 0 }));
    }

    #[test]
    fn moving_into_guard_monster_triggers_bump_attack() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec![".G.".to_string(), "...".to_string(), "...".to_string()];
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.spawn_guard_monsters_from_markers();
        let mut rng = FixedRng::new(vec![4, 1]);

        let out = step(&mut state, Command::Move(Direction::North), &mut rng);
        assert_eq!(state.player.position, Position { x: 1, y: 1 });
        assert!(out.events.iter().any(|event| matches!(event, Event::Attacked { .. })));
        assert!(!out.events.iter().any(|event| matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn attack_is_deterministic_with_injected_rng() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.stats.attack_min = 2;
        state.player.stats.attack_max = 5;
        state.spawn_monster(
            "rat",
            Position { x: 3, y: 2 },
            Stats { hp: 6, max_hp: 6, attack_min: 1, attack_max: 2, defense: 1 },
        );
        let mut rng = FixedRng::new(vec![4, 1, 4]);

        let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);
        assert_eq!(state.monsters[0].stats.hp, 3);

        let out = step(&mut state, Command::Attack(Direction::East), &mut rng);
        assert!(state.monsters.is_empty());
        assert!(out.events.iter().any(|event| matches!(event, Event::MonsterDefeated { .. })));
        assert!(!out.events.iter().any(|event| matches!(event, Event::VictoryAchieved)));
        assert_eq!(state.status, SessionStatus::InProgress);
    }

    #[test]
    fn pickup_drop_and_inventory_capacity_are_enforced() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.inventory_capacity = 1;
        state.place_item("potion", state.player.position);
        state.place_item("scroll", state.player.position);
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Pickup, &mut rng);
        assert_eq!(state.player.inventory.len(), 1);
        assert_eq!(state.ground_items.len(), 1);

        let full = step(&mut state, Command::Pickup, &mut rng);
        assert!(
            full.events.iter().any(|event| matches!(event, Event::InventoryFull { capacity: 1 }))
        );

        let _ = step(&mut state, Command::Drop { slot: 0 }, &mut rng);
        assert!(state.player.inventory.is_empty());
        assert_eq!(state.ground_items.len(), 2);

        let bad_drop = step(&mut state, Command::Drop { slot: 9 }, &mut rng);
        assert!(
            bad_drop.events.iter().any(|event| matches!(event, Event::InvalidDropSlot { slot: 9 }))
        );
    }

    #[test]
    fn two_handed_weapon_prevents_shield_auto_equip() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.place_item("Victrix", state.player.position);
        state.place_item("heater shield", state.player.position);
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Pickup, &mut rng);
        let _ = step(&mut state, Command::Pickup, &mut rng);

        assert!(state.player.equipment.weapon_hand.is_some());
        assert!(state.player.equipment.ready_hand.is_some());
        assert!(
            state.player.equipment.shield.is_none(),
            "two-handed weapon should block shield slot"
        );
    }

    #[test]
    fn legacy_inventory_command_reports_items_and_ground() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.inventory.push(Item::new(9, "practice blade"));
        state.carry_burden = 3;
        state.place_item("ground-ration", state.player.position);
        let mut rng = FixedRng::new(vec![]);

        let out = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let out_show = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);

        let note = out.events.iter().find_map(|event| match event {
            Event::LegacyHandled { token, note, .. } if token == "i" => Some(note.as_str()),
            _ => None,
        });
        let note = note.expect("inventory note should be present");
        assert!(note.contains("Inventory action"));
        assert!(state.pending_inventory_interaction.is_some());
        assert!(out_show.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "inventory" && note.contains("practice blade")
        )));
        assert!(
            state.log.iter().any(|line| line.contains("Pack:") && line.contains("practice blade"))
        );
        assert!(
            state.log.iter().all(|line| !line.contains("inventory mode viewed")),
            "placeholder inventory note should not appear"
        );
    }

    #[test]
    fn legacy_inventory_command_reports_empty_pack_without_placeholder() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        let mut rng = FixedRng::new(vec![]);

        let out = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);

        let note = out.events.iter().find_map(|event| match event {
            Event::LegacyHandled { token, note, .. } if token == "i" => Some(note.as_str()),
            _ => None,
        });
        let note = note.expect("inventory note should be present");
        assert!(note.contains("Inventory action"));
        assert!(state.pending_inventory_interaction.is_some());
        assert!(!note.contains("inventory mode viewed"));
    }

    #[test]
    fn inventory_l_looks_selected_slot_item_not_pack_listing() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        let mut weapon = Item::new(9, "practice blade");
        weapon.known = true;
        weapon.truename = "fine longsword".to_string();
        state.player.inventory.push(weapon);
        state.player.equipment.ready_hand = Some(9);
        state.player.equipment.weapon_hand = Some(9);
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let show = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);
        let look = step(&mut state, Command::Legacy { token: "l".to_string() }, &mut rng);

        assert!(show.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "inventory" && note.starts_with("Pack")
        )));
        assert!(look.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "inventory" && note.starts_with("It's fine longsword")
        )));
        assert!(
            state.log.iter().any(|line| line.starts_with("It's fine longsword")),
            "slot inspection should be visible in timeline"
        );
    }

    #[test]
    fn inventory_show_pack_is_visible_and_non_advancing() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.inventory.push(Item::new(9, "practice blade"));
        let baseline_turn = state.clock.turn;
        let baseline_minutes = state.clock.minutes;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);

        assert_eq!(state.clock.turn, baseline_turn);
        assert_eq!(state.clock.minutes, baseline_minutes);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "inventory" && note.starts_with("Pack:")
        )));
        assert!(state.log.iter().any(|line| line.starts_with("Pack:")));
    }

    #[test]
    fn monsters_attack_player_and_can_defeat() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.player.stats.hp = 3;
        state.player.stats.max_hp = 3;
        state.spawn_monster(
            "fang",
            Position { x: 4, y: 3 },
            Stats { hp: 5, max_hp: 5, attack_min: 4, attack_max: 4, defense: 0 },
        );
        let mut rng = FixedRng::new(vec![4]);

        let out = step(&mut state, Command::Wait, &mut rng);
        assert!(out.events.iter().any(|event| matches!(event, Event::MonsterAttacked { .. })));
        assert!(out.events.iter().any(|event| matches!(event, Event::PlayerDefeated)));
        assert_eq!(state.status, SessionStatus::Lost);
        assert_eq!(state.player.stats.hp, 0);
        assert_eq!(state.death_source.as_deref(), Some("fang"));
        assert!(state.log.iter().any(|line| line.contains("Killed by fang.")));

        let ignored = step(&mut state, Command::Wait, &mut rng);
        assert!(ignored.events.iter().any(|event| matches!(
            event,
            Event::CommandIgnoredTerminal { status: SessionStatus::Lost }
        )));
    }

    #[test]
    fn status_effects_tick_and_expire() {
        let mut state = GameState::default();
        state.player.stats.hp = 5;
        state.player.stats.max_hp = 5;
        state.status_effects.push(StatusEffect {
            id: "poison".to_string(),
            remaining_turns: 2,
            magnitude: 1,
        });
        let mut rng = FixedRng::new(vec![]);

        let first = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(state.player.stats.hp, 4);
        assert_eq!(state.status_effects.len(), 1);
        assert!(first.events.iter().any(|event| matches!(
            event,
            Event::StatusTick { effect_id, remaining_turns: 1, .. } if effect_id == "poison"
        )));

        let second = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(state.player.stats.hp, 3);
        assert!(state.status_effects.is_empty());
        assert!(second.events.iter().any(|event| matches!(
            event,
            Event::StatusExpired { effect_id } if effect_id == "poison"
        )));
    }

    #[test]
    fn legacy_world_mode_and_hunt_commands_apply_modeled_effects() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        assert_eq!(state.world_mode, WorldMode::DungeonCity);

        let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
        assert_eq!(state.world_mode, WorldMode::Countryside);

        let before_items = state.ground_items.len();
        let out = step(&mut state, Command::Legacy { token: "H".to_string() }, &mut rng);
        assert_eq!(state.ground_items.len(), before_items + 1);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, fully_modeled: true, .. } if token == "H"
        )));
    }

    #[test]
    fn countryside_movement_applies_terrain_time_bonus() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::Country;
        state.map_rows = vec!["...".to_string(); 3];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); 9];
        state.country_grid = CountryGrid {
            width: 3,
            height: 3,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Plains,
                    current_terrain: CountryTerrainKind::Plains,
                    aux: 0,
                    status: 0,
                };
                9
            ],
        };
        let mountain_idx = 1;
        state.country_grid.cells[mountain_idx].base_terrain = CountryTerrainKind::Mountains;
        state.country_grid.cells[mountain_idx].current_terrain = CountryTerrainKind::Mountains;

        state.player.position = Position { x: 0, y: 0 };
        let mut rng = FixedRng::new(vec![100]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 1, y: 0 });
        assert_eq!(out.minutes, 120);
        assert_eq!(state.clock.minutes, 120);
    }

    #[test]
    fn countryside_movement_can_spawn_encounter() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::Country;
        state.map_rows = vec!["...".to_string(); 3];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); 9];
        state.country_grid = CountryGrid {
            width: 3,
            height: 3,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Plains,
                    current_terrain: CountryTerrainKind::Plains,
                    aux: 0,
                    status: 0,
                };
                9
            ],
        };
        state.encounter_monsters = vec!["wolf".to_string()];
        state.player.position = Position { x: 0, y: 0 };

        let mut rng = FixedRng::new(vec![1, 0]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 1, y: 0 });
        assert_eq!(state.monsters.len(), 1);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "encounter"
        )));
    }

    #[test]
    fn poppy_event_sets_navigation_lost_non_terminal() {
        let mut state = countryside_state(3, 3, CountryTerrainKind::Plains);
        state.player.position = Position { x: 0, y: 0 };
        let mut rng = FixedRng::new(vec![1, 100]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert!(state.navigation_lost);
        assert_eq!(state.status, SessionStatus::InProgress);
        assert!(
            state.log.iter().any(|line| line.contains("poppies") || line.contains("disoriented"))
        );
    }

    #[test]
    fn lost_movement_randomizes_direction() {
        let mut state = countryside_state(3, 3, CountryTerrainKind::Plains);
        state.player.position = Position { x: 1, y: 1 };
        state.navigation_lost = true;
        state.known_sites.push(Position { x: 1, y: 0 });
        state.known_sites.push(Position { x: 2, y: 1 });
        state.known_sites.push(Position { x: 1, y: 2 });
        state.known_sites.push(Position { x: 0, y: 1 });
        let mut rng = FixedRng::new(vec![0, 250, 100]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 1, y: 0 });
        assert!(state.log.iter().any(|line| line.contains("strike out randomly")));
    }

    #[test]
    fn lost_state_clears_when_visibility_conditions_met() {
        let mut state = countryside_state(3, 3, CountryTerrainKind::Plains);
        state.player.position = Position { x: 1, y: 1 };
        state.navigation_lost = true;
        state.precipitation = 0;
        state.known_sites.push(Position { x: 2, y: 1 });
        let mut rng = FixedRng::new(vec![2, 250, 100]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert!(!state.navigation_lost);
        assert!(state.log.iter().any(|line| line.contains("Now you know where you are")));
    }

    #[test]
    fn chaos_sea_unprepared_can_be_fatal() {
        let mut state = countryside_state(3, 3, CountryTerrainKind::ChaosSea);
        state.player.position = Position { x: 1, y: 1 };
        state.player.stats.hp = 12;
        state.player.stats.max_hp = 12;
        state.progression.priest_rank = 0;
        state.progression.quests.sorcerors.rank = 0;
        let mut rng = FixedRng::new(vec![250, 100]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.status, SessionStatus::Lost);
        assert_eq!(state.death_source.as_deref(), Some("immersion in raw Chaos"));
    }

    #[test]
    fn chaos_sea_protection_survives_once() {
        let mut state = countryside_state(3, 3, CountryTerrainKind::ChaosSea);
        state.player.position = Position { x: 1, y: 1 };
        state.progression.priest_rank = 1;
        state.chaos_protection_consumed = false;
        let mut rng = FixedRng::new(vec![250, 100]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.status, SessionStatus::InProgress);
        assert!(state.chaos_protection_consumed);
    }

    #[test]
    fn over_enchant_can_explode_item() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut item = Item::new(1, "unstable sword");
        item.family = ItemFamily::Weapon;
        item.plus = 13;
        item.usef = "I_NORMAL_WEAPON".to_string();
        state.player.inventory.push(item);
        state.player.equipment.weapon_hand = Some(1);
        state.player.equipment.ready_hand = Some(1);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "enchantment".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.player.inventory.iter().all(|entry| entry.id != 1));
        assert!(state.log.iter().any(|line| line.contains("explode")));
    }

    #[test]
    fn bless_can_disintegrate_strongly_cursed_item() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut item = Item::new(1, "cursed amulet");
        item.family = ItemFamily::Thing;
        item.blessing = -3;
        state.player.inventory.push(item);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "blessing".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.player.inventory.is_empty());
        assert!(state.log.iter().any(|line| line.contains("disintegrates")));
    }

    #[test]
    fn decurse_failure_branch_preserves_curse() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut item = Item::new(1, "cursed ring");
        item.family = ItemFamily::Ring;
        item.blessing = -3;
        item.used = true;
        state.player.inventory.push(item);
        state.player.equipment.ring_1 = Some(1);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "dispelling".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        let blessed = state.player.inventory.first().map(|entry| entry.blessing).unwrap_or(0);
        assert!(blessed < 0);
        assert!(state.log.iter().any(|line| line.contains("dark laughter")));
    }

    #[test]
    fn countryside_encounter_does_not_spawn_on_city_or_village_cells() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::Country;
        state.map_rows = vec!["...".to_string(); 3];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); 9];
        state.country_grid = CountryGrid {
            width: 3,
            height: 3,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Plains,
                    current_terrain: CountryTerrainKind::Plains,
                    aux: 0,
                    status: 0,
                };
                9
            ],
        };
        state.country_grid.cells[1].base_terrain = CountryTerrainKind::City;
        state.country_grid.cells[1].current_terrain = CountryTerrainKind::City;
        state.player.position = Position { x: 0, y: 0 };

        let mut rng = FixedRng::new(vec![1, 0]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 1, y: 0 });
        assert!(state.monsters.is_empty());
        assert!(out.events.iter().all(|event| !matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "encounter"
        )));
    }

    #[test]
    fn countryside_encounter_requires_country_semantic_context() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::City;
        state.map_rows = vec!["...".to_string(); 3];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); 9];
        state.country_grid = CountryGrid {
            width: 3,
            height: 3,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Plains,
                    current_terrain: CountryTerrainKind::Plains,
                    aux: 0,
                    status: 0,
                };
                9
            ],
        };
        state.player.position = Position { x: 0, y: 0 };

        let mut rng = FixedRng::new(vec![1, 0]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 1, y: 0 });
        assert!(state.monsters.is_empty());
        assert!(out.events.iter().all(|event| !matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "encounter"
        )));
    }

    #[test]
    fn countryside_encounter_filters_passive_monster_aliases() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.world_mode = WorldMode::Countryside;
        state.environment = LegacyEnvironment::Countryside;
        state.map_binding.semantic = MapSemanticKind::Country;
        state.map_rows = vec!["...".to_string(); 3];
        state.country_map_rows = state.map_rows.clone();
        state.country_site_grid = vec![TileSiteCell::default(); 9];
        state.country_grid = CountryGrid {
            width: 3,
            height: 3,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Plains,
                    current_terrain: CountryTerrainKind::Plains,
                    aux: 0,
                    status: 0,
                };
                9
            ],
        };
        state.encounter_monsters = vec!["sheep".to_string()];
        state.player.position = Position { x: 0, y: 0 };

        let mut rng = FixedRng::new(vec![1, 0]);
        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.monsters.len(), 1);
        assert_ne!(state.monsters[0].name.to_ascii_lowercase(), "sheep");
    }

    #[test]
    fn wizard_wish_flow_is_interactive_and_commits_on_enter() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let start_turn = state.clock.turn;
        let start_minutes = state.clock.minutes;
        let start_gold = state.gold;
        let mut rng = FixedRng::new(vec![]);

        let begin = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        assert!(state.pending_wizard_interaction.is_some());
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);
        assert!(begin.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "^x"
        )));

        let _ = step(&mut state, Command::Legacy { token: "wealth".to_string() }, &mut rng);
        assert!(state.pending_wizard_interaction.is_some());
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);

        let commit = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(state.pending_wizard_interaction.is_none());
        assert!(state.gold > start_gold);
        assert_eq!(commit.turn, start_turn + 1);
        assert_eq!(commit.minutes, start_minutes + 5);
    }

    #[test]
    fn wizard_wish_get_item_opens_picker_and_never_yields_placeholder_items() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let start_turn = state.clock.turn;
        let start_minutes = state.clock.minutes;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "get item".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);
        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishAcquisitionKindSelect { cheated: true, .. })
        ));

        let _ = step(&mut state, Command::Legacy { token: ")".to_string() }, &mut rng);
        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishAcquisitionItemSelect {
                cheated: true,
                kind: WishItemKind::Weapon
            })
        ));
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);

        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
        let commit = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert_eq!(commit.turn, start_turn + 1);
        assert_eq!(commit.minutes, start_minutes + 5);
        assert_eq!(state.player.inventory.len(), 1);
        assert!(state.player.inventory[0].name.len() > 2);
        assert!(!state.player.inventory[0].name.contains("wishforged"));
        assert!(!state.player.inventory[0].name.contains("acquired trinket"));
    }

    #[test]
    fn wizard_wish_unknown_phrase_returns_classic_stupid_response() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ = step(
            &mut state,
            Command::Legacy { token: "totally unknown wish phrase".to_string() },
            &mut rng,
        );
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert!(state.log.iter().any(|line| line.contains("You feel stupid")));
    }

    #[test]
    fn wizard_wish_acquisition_non_cheated_random_kind_grants_real_item() {
        let mut state = GameState::default();
        state.progression.guild_rank = 4;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "get item".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishAcquisitionKindSelect { cheated: false, .. })
        ));

        let _ = step(&mut state, Command::Legacy { token: ")".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert_eq!(state.player.inventory.len(), 1);
        assert!(!state.player.inventory[0].name.contains("wishforged"));
        assert!(!state.player.inventory[0].name.contains("acquired trinket"));
    }

    #[test]
    fn wizard_wish_artifact_is_rejected_when_not_cheated() {
        let mut state = GameState::default();
        state.progression.guild_rank = 4;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ =
            step(&mut state, Command::Legacy { token: "acquire artifact".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishAcquisitionKindSelect { cheated: false, .. })
        ));

        let _ = step(&mut state, Command::Legacy { token: "&".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert!(state.player.inventory.is_empty());
        assert!(state.log.iter().any(|line| line.contains("You feel stupid")));
    }

    #[test]
    fn wizard_wish_acquisition_direct_hint_skips_picker_when_unique() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ = step(
            &mut state,
            Command::Legacy { token: "acquire food ration".to_string() },
            &mut rng,
        );
        let commit = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert_eq!(state.player.inventory.len(), 1);
        assert!(state.player.inventory[0].name.to_ascii_lowercase().contains("food ration"));
        assert_eq!(commit.minutes, 5);
    }

    #[test]
    fn wizard_wish_direct_item_name_victrix_resolves_without_stupid_message() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "Victrix".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert!(
            state.player.inventory.iter().any(|item| item.name == "Victrix"),
            "direct item-name wish should grant Victrix"
        );
        assert!(!state.log.iter().any(|line| line.contains("You feel stupid")));
    }

    #[test]
    fn wizard_wish_char_by_char_victrix_commit_grants_item_without_prompt_spam() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let log_len_after_open = state.log.len();

        for token in ["V", "i", "c", "t", "r", "i", "x"] {
            let _ = step(&mut state, Command::Legacy { token: token.to_string() }, &mut rng);
        }
        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishTextEntry { .. })
        ));
        assert_eq!(state.wizard_input_buffer, "Victrix");
        assert_eq!(
            state.log.len(),
            log_len_after_open,
            "typing into wish prompt should not add per-key log lines"
        );

        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert!(state.pending_wizard_interaction.is_none());
        assert!(
            state.player.inventory.iter().any(|item| item.name == "Victrix"),
            "char-by-char wish entry should grant Victrix"
        );
        assert!(!state.log.iter().any(|line| line.contains("You feel stupid")));
    }

    #[test]
    fn wizard_wish_text_entry_typing_does_not_spam_log() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^x".to_string() }, &mut rng);
        let log_len_after_open = state.log.len();

        let _ = step(&mut state, Command::Legacy { token: "v".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "c".to_string() }, &mut rng);

        assert!(matches!(
            state.pending_wizard_interaction,
            Some(WizardInteraction::WishTextEntry { .. })
        ));
        assert_eq!(state.wizard_input_buffer, "vic");
        assert_eq!(
            state.log.len(),
            log_len_after_open,
            "typing into wizard text prompts should not append a log line per keystroke"
        );
    }

    #[test]
    fn legacy_city_services_dialogue_and_donation_update_world_state() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        state.player.position = Position { x: 3, y: 3 };
        state.topology.country_rampart_position = Some(Position { x: 3, y: 3 });
        let mut country_rows = vec![".......".to_string(); 7];
        country_rows[3].replace_range(3..4, "O");
        state.country_map_rows = country_rows;
        state.country_site_grid = vec![TileSiteCell::default(); 49];
        let mut country_cells = vec![
            CountryCell {
                glyph: '.',
                base_terrain: CountryTerrainKind::Road,
                current_terrain: CountryTerrainKind::Road,
                aux: 0,
                status: 0,
            };
            49
        ];
        country_cells[24] = CountryCell {
            glyph: 'O',
            base_terrain: CountryTerrainKind::City,
            current_terrain: CountryTerrainKind::City,
            aux: 0,
            status: 0,
        };
        state.country_grid = CountryGrid { width: 7, height: 7, cells: country_cells };

        let mut rng = FixedRng::new(vec![]);
        let start_gold = state.gold;

        let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "G".to_string() }, &mut rng);

        assert_eq!(state.world_mode, WorldMode::DungeonCity);
        assert!(state.known_sites.len() >= 2);
        assert!(state.gold < start_gold);
        assert!(out.events.iter().any(|event| matches!(event, Event::EconomyUpdated { .. })));
    }

    #[test]
    fn country_entry_opens_caves_site_binding() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.country_grid = CountryGrid {
            width: 5,
            height: 5,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Road,
                    current_terrain: CountryTerrainKind::Road,
                    aux: 0,
                    status: 0,
                };
                25
            ],
        };
        state.country_grid.cells[12] = CountryCell {
            glyph: '*',
            base_terrain: CountryTerrainKind::Caves,
            current_terrain: CountryTerrainKind::Caves,
            aux: 0,
            status: 0,
        };
        state.site_maps = vec![SiteMapDefinition {
            map_id: 2,
            level_index: 0,
            source: "test-caves.map".to_string(),
            environment: LegacyEnvironment::Caves,
            semantic: MapSemanticKind::Site,
            spawn: Position { x: 1, y: 1 },
            rows: vec![".....".to_string(); 5],
            site_grid: vec![TileSiteCell::default(); 25],
        }];
        let (_note, handled) = resolve_enter_country_site(&mut state);

        assert!(handled);
        assert_eq!(state.environment, LegacyEnvironment::Caves);
        assert_eq!(state.map_binding.map_id, 2);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::Site);
    }

    #[test]
    fn country_entry_opens_volcano_site_binding() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.country_grid = CountryGrid {
            width: 5,
            height: 5,
            cells: vec![
                CountryCell {
                    glyph: '.',
                    base_terrain: CountryTerrainKind::Road,
                    current_terrain: CountryTerrainKind::Road,
                    aux: 0,
                    status: 0,
                };
                25
            ],
        };
        state.country_grid.cells[12] = CountryCell {
            glyph: '!',
            base_terrain: CountryTerrainKind::Volcano,
            current_terrain: CountryTerrainKind::Volcano,
            aux: 0,
            status: 0,
        };
        state.site_maps = vec![SiteMapDefinition {
            map_id: 4,
            level_index: 0,
            source: "test-volcano.map".to_string(),
            environment: LegacyEnvironment::Volcano,
            semantic: MapSemanticKind::Site,
            spawn: Position { x: 1, y: 1 },
            rows: vec![".....".to_string(); 5],
            site_grid: vec![TileSiteCell::default(); 25],
        }];
        let (_note, handled) = resolve_enter_country_site(&mut state);

        assert!(handled);
        assert_eq!(state.environment, LegacyEnvironment::Volcano);
        assert_eq!(state.map_binding.map_id, 4);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::Site);
    }

    #[test]
    fn give_command_uses_item_prompt_when_inventory_present() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        state.player.inventory.push(Item {
            id: 1,
            name: "offering dagger".to_string(),
            family: ItemFamily::Thing,
            ..Item::default()
        });
        let mut rng = FixedRng::new(vec![]);

        let open = step(&mut state, Command::Legacy { token: "G".to_string() }, &mut rng);
        assert_eq!(open.minutes, 0);
        assert!(state.pending_item_prompt.is_some());

        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        assert!(state.pending_item_prompt.is_none());
        assert!(state.player.inventory.is_empty());
        assert!(state.progression.deity_favor > 0);
    }

    #[test]
    fn wizard_victory_disables_high_score_eligibility() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^g".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "y".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "Q".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "y".to_string() }, &mut rng);
        assert_eq!(state.status, SessionStatus::Won);
        assert_eq!(state.progression.victory_trigger, Some(VictoryTrigger::QuitConfirmed));
        assert_eq!(state.progression.ending, EndingKind::Victory);
        assert!(!state.progression.high_score_eligible);
        assert!(out.events.iter().any(|event| matches!(event, Event::EndingResolved { .. })));
    }

    #[test]
    fn quest_completion_does_not_trigger_victory_state() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.progression.quest_state = LegacyQuestState::Completed;
        state.progression.main_quest.stage = LegacyQuestState::Completed;
        let mut rng = FixedRng::new(vec![]);

        let out = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(state.status, SessionStatus::InProgress);
        assert!(out.events.iter().all(|event| !matches!(event, Event::VictoryAchieved)));
    }

    #[test]
    fn legacy_q_cancel_keeps_session_in_progress() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        let mut rng = FixedRng::new(vec![]);
        let _ = step(&mut state, Command::Legacy { token: "Q".to_string() }, &mut rng);
        assert_eq!(state.pending_quit_interaction, Some(QuitInteraction::ConfirmQuit));
        let _ = step(&mut state, Command::Legacy { token: "n".to_string() }, &mut rng);
        assert_eq!(state.pending_quit_interaction, None);
        assert_eq!(state.status, SessionStatus::InProgress);
    }

    #[test]
    fn quit_with_adept_rank_yields_total_winner_ending() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.progression.adept_rank = 1;
        let mut rng = FixedRng::new(vec![]);
        let _ = step(&mut state, Command::Legacy { token: "Q".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "y".to_string() }, &mut rng);
        assert_eq!(state.status, SessionStatus::Won);
        assert_eq!(state.progression.ending, EndingKind::TotalWinner);
        assert_eq!(state.progression.victory_trigger, Some(VictoryTrigger::QuitConfirmed));
        assert!(out.events.iter().any(|event| matches!(event, Event::EndingResolved { .. })));
    }

    #[test]
    fn wizard_pending_interaction_does_not_advance_turn_or_run_monsters() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.wizard.enabled = true;
        state.player.position = Position { x: 2, y: 2 };
        state.spawn_monster(
            "rat",
            Position { x: 3, y: 2 },
            Stats { hp: 9, max_hp: 9, attack_min: 1, attack_max: 2, defense: 0 },
        );
        let start_hp = state.player.stats.hp;
        let start_turn = state.clock.turn;
        let start_minutes = state.clock.minutes;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^k".to_string() }, &mut rng);
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);
        assert_eq!(state.player.stats.hp, start_hp);

        let _ = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);
        assert_eq!(state.player.stats.hp, start_hp);
    }

    #[test]
    fn wizard_status_editor_sets_bits_but_blocks_cheated_bit_mutation() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "^k".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "5".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!((state.legacy_status_flags & (1u64 << 5)) != 0);

        let _ = step(&mut state, Command::Legacy { token: "s".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "18".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!((state.legacy_status_flags & LEGACY_STATUS_CHEATED) != 0);
    }

    #[test]
    fn wizard_stat_editor_applies_value_and_recomputes_combat() {
        let mut state = GameState::default();
        state.wizard.enabled = true;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "#".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: " ".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "20".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert_eq!(state.attributes.strength, 20);
        assert!(state.player.stats.attack_max >= state.player.stats.attack_min + 1);
    }

    #[test]
    fn options_command_cycles_runtime_toggles() {
        let mut state = GameState::default();
        let mut rng = FixedRng::new(vec![]);
        let before_pickup = state.options.pickup;
        let before_confirm = state.options.confirm;
        let before_searchnum = state.options.searchnum;

        let _ = step(&mut state, Command::Legacy { token: "O".to_string() }, &mut rng);
        assert_ne!(state.options.pickup, before_pickup);
        assert_ne!(state.options.confirm, before_confirm);
        assert_ne!(state.options.searchnum, before_searchnum);
    }

    #[test]
    fn character_creation_applies_archetype_and_alignment() {
        let mut state = GameState::default();
        let creation = CharacterCreation {
            name: "TestHero".to_string(),
            archetype_id: "mage".to_string(),
            alignment: Alignment::Chaotic,
        };
        apply_character_creation(&mut state, &creation);
        assert_eq!(state.player_name, "TestHero");
        assert_eq!(state.progression.alignment, Alignment::Chaotic);
        assert!(state.spellbook.max_mana >= 140);
        assert!(state.gold >= 200);
    }

    #[test]
    fn legacy_questionnaire_profile_uses_reference_scoring() {
        let answers = LegacyQuestionnaireAnswers {
            bench_press_lbs: 120,
            pretty_dumb: true,
            can_ride_bicycle: true,
            can_tie_shoes_blindfolded: true,
            sexual_preference: 'm',
            ..LegacyQuestionnaireAnswers::default()
        };
        let profile = derive_legacy_questionnaire_profile(&answers);
        assert_eq!(profile.strength, 9);
        assert_eq!(profile.iq, 4);
        assert_eq!(profile.agility, 9);
        assert_eq!(profile.dexterity, 6);
        assert_eq!(profile.constitution, 13);
        assert_eq!(profile.power, 3);
        assert_eq!(profile.preference, 'm');

        let creation = derive_legacy_questionnaire_creation("LegacyHero".to_string(), &answers);
        assert_eq!(creation.creation.archetype_id, "fighter");
        assert_eq!(creation.creation.alignment, Alignment::Neutral);
    }

    #[test]
    fn applying_legacy_questionnaire_profile_updates_runtime_stats() {
        let mut state = GameState::default();
        let creation = CharacterCreation {
            name: "Caster".to_string(),
            archetype_id: "mage".to_string(),
            alignment: Alignment::Lawful,
        };
        apply_character_creation(&mut state, &creation);

        let answers = LegacyQuestionnaireAnswers {
            bench_press_lbs: 60,
            took_iq_test: true,
            iq_score: 180,
            took_undergraduate_exam: true,
            undergraduate_percentile: 95,
            took_graduate_exam: true,
            graduate_percentile: 90,
            can_ride_bicycle: true,
            can_tie_shoes_blindfolded: true,
            plays_video_games: true,
            gets_high_scores: true,
            typing_speed_wpm: 100,
            miles_can_run: 8,
            animals_react_oddly: true,
            can_see_auras: true,
            out_of_body_experience: true,
            cast_spell: true,
            spell_worked: true,
            has_esp: true,
            has_pk: true,
            believes_in_ghosts: true,
            sexual_preference: 'f',
            ..LegacyQuestionnaireAnswers::default()
        };
        let profile = derive_legacy_questionnaire_profile(&answers);
        apply_legacy_questionnaire_profile(&mut state, profile);

        assert_eq!(state.progression.alignment, Alignment::Neutral);
        assert_eq!(state.progression.law_chaos_score, 0);
        assert!(state.spellbook.max_mana > 160);
        assert!(state.player.stats.attack_max >= state.player.stats.attack_min + 1);
        assert!(state.player.stats.max_hp >= 12);
    }

    #[test]
    fn order_talk_realigns_lawful_and_advances_quest() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        let mut events = Vec::new();

        let (_line, _fully_modeled) = apply_talk_command(&mut state, &mut events);

        assert_eq!(state.progression.alignment, Alignment::Lawful);
        assert_eq!(state.progression.quest_state, LegacyQuestState::Active);
        assert!(events.iter().any(|event| matches!(
            event,
            Event::ProgressionUpdated { alignment: Alignment::Lawful, .. }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            Event::QuestAdvanced { state: LegacyQuestState::Active, .. }
        )));
    }

    #[test]
    fn service_talk_outputs_are_specific_for_all_guild_and_service_sites() {
        let cases = [
            (SITE_AUX_SERVICE_SHOP, ["merchant", "prices"]),
            (SITE_AUX_SERVICE_ARMORER, ["armorer", "mail"]),
            (SITE_AUX_SERVICE_CLUB, ["club", "stewards"]),
            (SITE_AUX_SERVICE_GYM, ["gym", "drills"]),
            (SITE_AUX_SERVICE_HEALER, ["healer", "wound"]),
            (SITE_AUX_SERVICE_CASINO, ["casino", "chips"]),
            (SITE_AUX_SERVICE_COMMANDANT, ["commandant", "bucket"]),
            (SITE_AUX_SERVICE_DINER, ["diner", "coffee"]),
            (SITE_AUX_SERVICE_CRAPS, ["dice", "games"]),
            (SITE_AUX_SERVICE_TAVERN, ["tavern", "ale"]),
            (SITE_AUX_SERVICE_PAWN_SHOP, ["pawnbroker", "bargain"]),
            (SITE_AUX_SERVICE_BROTHEL, ["madam", "room"]),
            (SITE_AUX_SERVICE_CONDO, ["condo", "lockbox"]),
            (SITE_AUX_SERVICE_BANK, ["banker", "account"]),
            (SITE_AUX_SERVICE_MERC_GUILD, ["quartermaster", "contracts"]),
            (SITE_AUX_SERVICE_THIEVES, ["fence", "guild"]),
            (SITE_AUX_SERVICE_COLLEGE, ["collegium", "studies"]),
            (SITE_AUX_SERVICE_SORCERORS, ["sorceror", "research"]),
            (SITE_AUX_SERVICE_CASTLE, ["castellan", "court"]),
            (SITE_AUX_SERVICE_ORDER, ["order", "conduct"]),
            (SITE_AUX_SERVICE_PALACE, ["chamberlain", "palace"]),
            (SITE_AUX_SERVICE_TEMPLE, ["prayer", "temple"]),
            (SITE_AUX_SERVICE_CHARITY, ["charity", "stewards"]),
            (SITE_AUX_SERVICE_MONASTERY, ["monastery", "wardens"]),
            (SITE_AUX_SERVICE_ARENA, ["arena", "officials"]),
        ];

        for (aux, expected_terms) in cases {
            let mut state = GameState::new(MapBounds { width: 3, height: 3 });
            state.player.position = Position { x: 1, y: 1 };
            state.site_grid = vec![TileSiteCell::default(); 9];
            state.city_site_grid = state.site_grid.clone();
            state.site_grid[4].aux = aux;
            state.city_site_grid[4].aux = aux;
            let mut events = Vec::new();
            let (line, _fully_modeled) = apply_talk_command(&mut state, &mut events);
            let line = line.to_ascii_lowercase();
            assert!(
                !line.contains("audience held")
                    && !line.contains("dialogue resolved with")
                    && !line.contains("you exchange a few words with")
                    && !line.contains("points you toward service and duty"),
                "service aux {aux} produced generic placeholder output: {line}"
            );
            assert!(
                expected_terms.iter().any(|needle| line.contains(needle)),
                "service aux {aux} line did not include expected terms {:?}: {line}",
                expected_terms
            );
        }
    }

    #[test]
    fn interactive_castle_order_temple_audience_lines_are_specific() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        let mut rng = FixedRng::new(vec![]);

        state.site_grid[4].aux = SITE_AUX_SERVICE_CASTLE;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_CASTLE;
        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "2".to_string() }, &mut rng);
        let castle_line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(castle_line.contains("castellan") || castle_line.contains("court"));
        assert!(!castle_line.contains("audience held"));
        assert!(!castle_line.contains("dialogue resolved with"));
        let _ = step(&mut state, Command::Legacy { token: "x".to_string() }, &mut rng);

        state.site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "3".to_string() }, &mut rng);
        let order_line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(order_line.contains("order") || order_line.contains("oath"));
        assert!(!order_line.contains("audience held"));
        assert!(!order_line.contains("dialogue resolved with"));
        let _ = step(&mut state, Command::Legacy { token: "x".to_string() }, &mut rng);

        state.site_grid[4].aux = SITE_AUX_SERVICE_TEMPLE;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_TEMPLE;
        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "2".to_string() }, &mut rng);
        let temple_line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(temple_line.contains("prayer") || temple_line.contains("temple"));
        assert!(!temple_line.contains("dialogue resolved with"));
    }

    #[test]
    fn merc_contract_sets_specific_legion_objective() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_MERC_GUILD;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_MERC_GUILD;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "2".to_string() }, &mut rng);
        let objective = state.progression.main_quest.objective.to_ascii_lowercase();
        let line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();

        assert!(objective.contains("legion"));
        assert!(objective.contains("centurion") || objective.contains("regalia"));
        assert!(line.contains("accepted legion contract"));
    }

    #[test]
    fn tavern_rumor_purchase_sets_actionable_quest_objective() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_TAVERN;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_TAVERN;
        state.gold = 100;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "3".to_string() }, &mut rng);

        let line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        let objective = state.progression.main_quest.objective.to_ascii_lowercase();
        assert_eq!(state.progression.quest_state, LegacyQuestState::Active);
        assert!(!objective.trim().is_empty(), "tavern rumor should establish a concrete objective");
        assert!(line.contains("rumor"));
        assert!(line.contains("quest"));
    }

    #[test]
    fn objective_adapters_are_read_only_and_deterministic() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.progression.quest_state = LegacyQuestState::Active;
        state.progression.main_quest.objective =
            "Report to the Mercenary Guild for your first contract.".to_string();
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.site_grid[4].aux = SITE_AUX_SERVICE_MERC_GUILD;
        let before = state.clone();

        let first_active = active_objective_snapshot(&state);
        let first_journal = objective_journal(&state);
        let first_hints = objective_map_hints(&state);
        let second_active = active_objective_snapshot(&state);
        let second_journal = objective_journal(&state);
        let second_hints = objective_map_hints(&state);

        assert_eq!(state, before);
        assert_eq!(first_active, second_active);
        assert_eq!(first_journal, second_journal);
        assert_eq!(first_hints, second_hints);
    }

    #[test]
    fn objective_map_hints_include_service_site_when_present() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.progression.quest_state = LegacyQuestState::Active;
        state.progression.main_quest.objective =
            "Return to the Order hall and report to the LawBringer.".to_string();
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.site_grid[3].aux = SITE_AUX_SERVICE_ORDER;

        let hints = objective_map_hints(&state);
        assert!(hints.contains(&Position { x: 0, y: 1 }));
    }

    #[test]
    fn objective_map_hints_bias_to_walkable_approach_near_door() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.progression.quest_state = LegacyQuestState::Active;
        state.progression.main_quest.objective = "Report to the castle.".to_string();
        state.player.position = Position { x: 0, y: 0 };
        state.map_rows = vec!["...".to_string(), ".-.".to_string(), "...".to_string()];
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.site_grid[4].aux = SITE_AUX_SERVICE_CASTLE;
        state.site_grid[4].flags = TILE_FLAG_BLOCK_MOVE;

        let hints = objective_map_hints(&state);
        assert!(hints.contains(&Position { x: 1, y: 0 }));
        assert!(!hints.contains(&Position { x: 1, y: 1 }));
    }

    #[test]
    fn tavern_rumor_purchase_uses_overhear_wording_without_placeholder_framing() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_TAVERN;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_TAVERN;
        state.gold = 100;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "3".to_string() }, &mut rng);

        let line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(line.contains("you overhear a rumor"));
        assert!(!line.contains("starts a wider quest"));
        assert!(!line.contains("tavern keeper shares a rumor"));
    }

    #[test]
    fn armorer_chain_mail_purchase_creates_armor_and_auto_equips() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARMORER;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARMORER;
        state.gold = 200;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        assert_eq!(state.player.inventory.len(), 1);
        let item = &state.player.inventory[0];
        assert_eq!(item.family, ItemFamily::Armor);
        assert_eq!(state.player.equipment.armor, Some(item.id));
        let line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(line.contains("chain mail"));
    }

    #[test]
    fn pawn_shop_buy_oddity_uses_catalog_item_name_not_placeholder() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_PAWN_SHOP;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_PAWN_SHOP;
        state.gold = 100;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        let line = state.log.last().cloned().unwrap_or_default().to_ascii_lowercase();
        assert!(state.player.inventory.len() == 1, "pawn buy should add one item");
        assert!(
            !line.contains("pawned oddity"),
            "pawn buy should report actual catalog item name, got: {line}"
        );
        assert!(
            !state.player.inventory[0].name.eq_ignore_ascii_case("pawned oddity"),
            "inventory item should not use placeholder name"
        );
        assert!(state.player.inventory[0].known, "pawn purchases should be identified stock");
    }

    #[test]
    fn castle_talk_assigns_goblin_king_quest_first() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_CASTLE;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_CASTLE;
        let mut events = Vec::new();

        let (line, _fully_modeled) = apply_talk_command(&mut state, &mut events);
        let line = line.to_ascii_lowercase();
        let objective = state.progression.main_quest.objective.to_ascii_lowercase();

        assert!(line.contains("goblin king"));
        assert!(objective.contains("goblin king"));
        assert!(state.progression.quests.castle.rank >= 1);
    }

    #[test]
    fn order_talk_references_justiciar_or_star_gem_duty() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ORDER;
        state.progression.quests.order.rank = 4;
        state.progression.alignment = Alignment::Lawful;
        state.progression.law_chaos_score = 8;
        let mut events = Vec::new();

        let (line, _fully_modeled) = apply_talk_command(&mut state, &mut events);
        let line = line.to_ascii_lowercase();

        assert!(line.contains("star gem") || line.contains("justiciar"));
    }

    #[test]
    fn arena_service_does_not_apply_immediate_monster_hit() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        let mut rng = FixedRng::new(vec![2]);

        let out = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);

        assert_eq!(state.environment, LegacyEnvironment::Arena);
        assert_eq!(state.map_binding.map_id, 1);
        assert_eq!(state.player.stats.hp, 20);
        assert_eq!(state.monsters.len(), 1);
        assert!(state.monsters[0].name.contains(" the "));
        assert!(state.monsters[0].name.contains("pencil-necked geek"));
        assert!(out.events.iter().all(|event| !matches!(event, Event::MonsterAttacked { .. })));
    }

    #[test]
    fn arena_roster_uses_legacy_identity_names() {
        let (first_name, _) = arena_rival_profile(0, 1);
        let (grunt_name, _) = arena_rival_profile(4, 1);

        assert!(first_name.contains("pencil-necked geek"));
        assert!(grunt_name.contains("grunt"));
        assert!(grunt_name.contains(" the "));
        assert!(!grunt_name.starts_with("arena "));
    }

    #[test]
    fn arena_menu_start_closes_interaction_and_enters_match() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        let mut rng = FixedRng::new(vec![2]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Arena));

        let out = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        assert_eq!(state.pending_site_interaction, None);
        assert_eq!(state.environment, LegacyEnvironment::Arena);
        assert!(state.progression.arena_match_active);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("arranging a match")
        )));
        assert!(closed_portcullis_count(&state) > 0);
    }

    #[test]
    fn arena_challenger_death_drops_opener_and_gate_stays_closed() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.player.stats.attack_min = 50;
        state.player.stats.attack_max = 50;
        let mut rng = FixedRng::new(vec![50]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
        assert!(closed_portcullis_count(&state) > 0);

        let challenger_pos = state.monsters.first().map(|m| m.position).expect("arena challenger");
        state.player.position = Position { x: challenger_pos.x - 1, y: challenger_pos.y };
        let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);

        assert!(state.monsters.is_empty());
        assert!(
            state.ground_items.iter().any(|entry| entry.item.usef == "I_RAISE_PORTCULLIS"),
            "arena challenger should drop portcullis opener"
        );
        assert!(closed_portcullis_count(&state) > 0, "gate should remain closed until opener use");
    }

    #[test]
    fn arena_opener_activation_raises_all_portcullises() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.player.stats.attack_min = 50;
        state.player.stats.attack_max = 50;
        let mut rng = FixedRng::new(vec![50]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
        let challenger_pos = state.monsters.first().map(|m| m.position).expect("arena challenger");
        state.player.position = Position { x: challenger_pos.x - 1, y: challenger_pos.y };
        let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);
        assert!(closed_portcullis_count(&state) > 0);

        let opener_pos = state
            .ground_items
            .iter()
            .find(|entry| entry.item.usef == "I_RAISE_PORTCULLIS")
            .map(|entry| entry.position)
            .expect("opener drop");
        state.player.position = opener_pos;
        let _ = step(&mut state, Command::Pickup, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);

        assert_eq!(closed_portcullis_count(&state), 0);
    }

    #[test]
    fn arena_open_portcullis_gateway_allows_exit_back_to_city() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.player.stats.attack_min = 50;
        state.player.stats.attack_max = 50;
        let mut rng = FixedRng::new(vec![50]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);
        let challenger_pos = state.monsters.first().map(|m| m.position).expect("arena challenger");
        state.player.position = Position { x: challenger_pos.x - 1, y: challenger_pos.y };
        let _ = step(&mut state, Command::Attack(Direction::East), &mut rng);
        let opener_pos = state
            .ground_items
            .iter()
            .find(|entry| entry.item.usef == "I_RAISE_PORTCULLIS")
            .map(|entry| entry.position)
            .expect("opener drop");
        state.player.position = opener_pos;
        let _ = step(&mut state, Command::Pickup, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "i".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        assert_eq!(closed_portcullis_count(&state), 0);

        state.player.position = Position { x: 2, y: 7 };
        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);

        assert_eq!(
            state.environment,
            LegacyEnvironment::City,
            "expected arena exit after walking onto raised gateway; pos=({}, {}), map_id={}",
            state.player.position.x,
            state.player.position.y,
            state.map_binding.map_id
        );
        assert_eq!(state.map_binding.semantic, MapSemanticKind::City);
        assert!(state.log.iter().any(|line| line.contains("left the arena")));
    }

    #[test]
    fn arena_menu_accepts_legacy_letter_choices() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        let mut rng = FixedRng::new(vec![2]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let register = step(&mut state, Command::Legacy { token: "r".to_string() }, &mut rng);

        assert_eq!(state.progression.arena_rank, 1);
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Arena));
        assert!(register.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("Selected option 2")
        )));

        let start = step(&mut state, Command::Legacy { token: "y".to_string() }, &mut rng);
        assert_eq!(state.pending_site_interaction, None);
        assert_eq!(state.environment, LegacyEnvironment::Arena);
        assert!(state.progression.arena_match_active);
        assert!(start.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("arranging a match")
        )));
    }

    #[test]
    fn arena_menu_rejects_restart_while_match_active() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.progression.arena_rank = 1;
        state.progression.arena_opponent = 3;
        state.progression.arena_match_active = true;
        state.spawn_monster(
            "arena goblin",
            Position { x: 5, y: 4 },
            Stats { hp: 8, max_hp: 8, attack_min: 2, attack_max: 3, defense: 1 },
        );
        let monster_count_before = state.monsters.len();
        let mut events = Vec::new();

        let note = apply_site_interaction_choice(
            &mut state,
            SiteInteractionKind::Arena,
            1,
            &mut events,
            true,
        );

        assert!(note.contains("already in the games"));
        assert_eq!(state.monsters.len(), monster_count_before);
        assert!(state.progression.arena_match_active);
    }

    #[test]
    fn arena_exit_tile_returns_player_to_city_context() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_maps = vec![arena_test_site_definition()];
        state.site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        state.city_site_grid[4].aux = SITE_AUX_SERVICE_ARENA;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        assert_eq!(state.environment, LegacyEnvironment::Arena);
        state.player.position = Position { x: 1, y: 7 };

        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);

        assert_eq!(state.environment, LegacyEnvironment::City);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::City);
        assert_eq!(state.player.position, Position { x: 1, y: 1 });
        assert!(state.monsters.is_empty(), "arena rival should not persist into city context");
    }

    #[test]
    fn activating_city_view_clears_transient_hostiles() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.city_map_rows = vec!["...".to_string(), "...".to_string(), "...".to_string()];
        state.city_site_grid = vec![TileSiteCell::default(); 9];
        state.country_map_rows = state.city_map_rows.clone();
        state.country_site_grid = state.city_site_grid.clone();
        state.activate_country_view();
        state.spawn_monster(
            "sheep",
            Position { x: 2, y: 1 },
            Stats { hp: 4, max_hp: 4, attack_min: 1, attack_max: 1, defense: 0 },
        );
        assert_eq!(state.monsters.len(), 1);

        state.activate_city_view();

        assert_eq!(state.environment, LegacyEnvironment::City);
        assert!(state.monsters.is_empty());
    }

    #[test]
    fn altar_prayer_accepts_matching_alignment_and_sets_patron() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.progression.alignment = Alignment::Lawful;
        state.progression.law_chaos_score = 6;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_ALTAR_ODIN;
        state.city_site_grid[4].aux = SITE_AUX_ALTAR_ODIN;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        assert_eq!(state.progression.patron_deity, DEITY_ID_ODIN);
        assert!(state.progression.priest_rank >= 1);
        assert!(state.progression.deity_favor >= 3);
    }

    #[test]
    fn altar_prayer_to_hostile_deity_triggers_sacrilege() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.options.interactive_sites = true;
        state.progression.alignment = Alignment::Lawful;
        state.progression.patron_deity = DEITY_ID_ODIN;
        state.progression.priest_rank = 2;
        state.progression.deity_favor = 16;
        state.player.position = Position { x: 1, y: 1 };
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[4].aux = SITE_AUX_ALTAR_SET;
        state.city_site_grid[4].aux = SITE_AUX_ALTAR_SET;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        assert_eq!(state.progression.patron_deity, 0);
        assert_eq!(state.progression.priest_rank, 0);
        assert_eq!(state.progression.deity_favor, 0);
    }

    #[test]
    fn door_open_and_close_commands_toggle_walkability() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["...".to_string(), "..-".to_string(), "...".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;
        state.city_site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;

        assert!(!state.tile_is_walkable(Position { x: 2, y: 1 }));
        let mut rng = FixedRng::new(vec![]);
        let _ = step(&mut state, Command::Legacy { token: "o".to_string() }, &mut rng);
        assert_eq!(state.map_glyph_at(Position { x: 2, y: 1 }), '/');
        assert!(state.tile_is_walkable(Position { x: 2, y: 1 }));

        let _ = step(&mut state, Command::Legacy { token: "c".to_string() }, &mut rng);
        assert_eq!(state.map_glyph_at(Position { x: 2, y: 1 }), '-');
        assert!(!state.tile_is_walkable(Position { x: 2, y: 1 }));
    }

    #[test]
    fn bumping_closed_door_opens_and_steps_forward() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["...".to_string(), "..-".to_string(), "...".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;
        state.city_site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;

        let mut rng = FixedRng::new(vec![]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 2, y: 1 });
        assert_eq!(state.map_glyph_at(Position { x: 2, y: 1 }), '/');
        assert!(out.events.iter().any(|event| matches!(event, Event::Moved { .. })));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "step"
        )));
    }

    #[test]
    fn stepping_on_service_tile_triggers_interaction() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let start_gold = state.gold;

        let mut rng = FixedRng::new(vec![]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 2, y: 1 });
        assert!(state.gold < start_gold);
        assert!(out.events.iter().any(|event| matches!(event, Event::EconomyUpdated { .. })));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "step"
        )));
    }

    #[test]
    fn stepping_on_service_tile_opens_interactive_menu_when_enabled() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let start_gold = state.gold;

        let mut rng = FixedRng::new(vec![]);
        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, Position { x: 2, y: 1 });
        assert_eq!(state.gold, start_gold, "stepping should open menu before applying purchase");
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "interaction"
        )));
    }

    #[test]
    fn interactive_site_menu_accepts_numeric_choice_via_legacy_token() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);
        let gold_before = state.gold;
        let out = step(&mut state, Command::Legacy { token: "1".to_string() }, &mut rng);

        assert!(state.gold < gold_before);
        assert!(state.player.inventory.iter().any(|item| item.name == "food ration"));
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::EconomyUpdated { source, .. } if source == "shop"
        )));
    }

    #[test]
    fn jail_doors_are_openable_with_open_command() {
        let mut state = GameState::new(MapBounds { width: 3, height: 3 });
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["...".to_string(), "..J".to_string(), "...".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 9];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;
        state.city_site_grid[5].flags = TILE_FLAG_BLOCK_MOVE;
        let mut rng = FixedRng::new(vec![]);

        assert!(!state.tile_is_walkable(Position { x: 2, y: 1 }));
        let _ = step(&mut state, Command::Legacy { token: "o".to_string() }, &mut rng);
        assert_eq!(state.map_glyph_at(Position { x: 2, y: 1 }), '/');
        assert!(state.tile_is_walkable(Position { x: 2, y: 1 }));
    }

    #[test]
    fn pending_interaction_blocks_non_choice_commands_until_closed() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));

        let out_pending = step(&mut state, Command::Move(Direction::West), &mut rng);
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));
        assert_eq!(state.player.position, Position { x: 2, y: 1 });
        assert!(out_pending.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("prompt active")
        )));

        let out_close = step(&mut state, Command::Legacy { token: "q".to_string() }, &mut rng);
        assert_eq!(state.pending_site_interaction, None);
        assert!(out_close.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("closed")
        )));

        let out_move = step(&mut state, Command::Move(Direction::West), &mut rng);
        assert_eq!(state.player.position, Position { x: 1, y: 1 });
        assert!(out_move.events.iter().any(|event| matches!(
            event,
            Event::Moved { from, to }
                if *from == Position { x: 2, y: 1 } && *to == Position { x: 1, y: 1 }
        )));
    }

    #[test]
    fn pending_interaction_hint_is_not_spammed_in_log() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));

        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);
        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);
        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);

        let hint_count = state
            .log
            .iter()
            .filter(|line| line.contains("Site prompt active: choose a bracketed option"))
            .count();
        assert_eq!(hint_count, 0);
    }

    #[test]
    fn entering_interactive_site_does_not_log_menu_prompt_lines() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 1].aux = SITE_AUX_SERVICE_TEMPLE;
        state.city_site_grid[1 * 4 + 1].aux = SITE_AUX_SERVICE_TEMPLE;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);

        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Temple));
        assert!(state.log.iter().all(|line| {
            !line.contains("Temple: [")
                && !line.contains("Site prompt active:")
                && !line.contains("Temple prompt active:")
        }));
    }

    #[test]
    fn invalid_modal_input_does_not_append_prompt_hint_noise_to_timeline() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.pending_site_interaction = Some(SiteInteractionKind::Temple);
        let before_len = state.log.len();
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Move(Direction::West), &mut rng);

        assert_eq!(state.log.len(), before_len);
        assert!(state.log.iter().all(|line| !line.contains("prompt active")));
    }

    #[test]
    fn sanitize_legacy_prompt_noise_preserves_real_outcomes() {
        let mut log = vec![
            "You move.".to_string(),
            "Site prompt active: choose a bracketed option, or press q/x to close.".to_string(),
            "Wish text: Victrix_".to_string(),
            "Dropped ration.".to_string(),
        ];

        sanitize_legacy_prompt_noise(&mut log);

        assert_eq!(log, vec!["You move.".to_string(), "Dropped ration.".to_string()]);
    }

    #[test]
    fn interactive_site_menu_accepts_letter_alias_choice() {
        let mut state = GameState::new(MapBounds { width: 4, height: 3 });
        state.options.interactive_sites = true;
        state.player.position = Position { x: 1, y: 1 };
        state.map_rows = vec!["....".to_string(), "....".to_string(), "....".to_string()];
        state.city_map_rows = state.map_rows.clone();
        state.map_binding.semantic = MapSemanticKind::City;
        state.site_grid = vec![TileSiteCell::default(); 12];
        state.city_site_grid = state.site_grid.clone();
        state.site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        state.city_site_grid[1 * 4 + 2].aux = SITE_AUX_SERVICE_SHOP;
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);
        let gold_before = state.gold;
        let out = step(&mut state, Command::Legacy { token: "r".to_string() }, &mut rng);

        assert!(state.gold < gold_before);
        assert!(state.player.inventory.iter().any(|item| item.name == "food ration"));
        assert_eq!(state.pending_site_interaction, Some(SiteInteractionKind::Shop));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "interaction" && note.contains("Selected option 1")
        )));
    }

    #[test]
    fn trap_triggers_and_can_be_disarmed() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        let trap_pos = Position { x: state.player.position.x + 1, y: state.player.position.y };
        state.traps = vec![Trap {
            id: 99,
            position: trap_pos,
            damage: 2,
            effect_id: "poison".to_string(),
            armed: true,
        }];

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);
        assert!(state.player.stats.hp < state.player.stats.max_hp);
        assert!(state.status_effects.iter().any(|effect| effect.id == "poison"));

        state.player.position = Position { x: trap_pos.x - 1, y: trap_pos.y };
        state.traps[0].armed = true;
        let _ = step(&mut state, Command::Legacy { token: "D".to_string() }, &mut rng);
        assert!(!state.traps[0].armed);
    }

    #[test]
    fn lethal_trap_sets_death_source() {
        let mut state = GameState::new(MapBounds { width: 5, height: 5 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.stats.hp = 2;
        state.player.stats.max_hp = 2;
        state.traps = vec![Trap {
            id: 7,
            position: state.player.position,
            damage: 5,
            effect_id: "acid".to_string(),
            armed: true,
        }];
        let mut rng = FixedRng::new(vec![]);

        let out = step(&mut state, Command::Wait, &mut rng);

        assert_eq!(state.status, SessionStatus::Lost);
        assert_eq!(state.death_source.as_deref(), Some("acid trap"));
        assert!(out.events.iter().any(|event| matches!(event, Event::PlayerDefeated)));
    }

    #[test]
    fn spellcasting_consumes_mana_and_applies_effects() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        let mana_before = state.spellbook.mana;
        state.spawn_monster(
            "imp-mage",
            Position { x: state.player.position.x + 2, y: state.player.position.y },
            Stats { hp: 5, max_hp: 5, attack_min: 1, attack_max: 1, defense: 0 },
        );

        let open = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        assert_eq!(state.spellbook.mana, mana_before);
        let _ = step(&mut state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(state.spellbook.mana < mana_before);
        assert!(open.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. } if token == "m" && note.starts_with("Cast Spell:")
        )));
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, fully_modeled: true }
                if token == "m" && note.starts_with("cast spell#")
        )));
    }

    #[test]
    fn magic_command_reports_when_no_known_spells() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        for spell in &mut state.spellbook.spells {
            spell.known = false;
        }

        let out = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);

        assert!(state.pending_spell_interaction.is_none());
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "m" && note.contains("don't know any spells")
        )));
    }

    #[test]
    fn spell_prompt_is_non_advancing_until_enter_commit() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        let start_turn = state.clock.turn;
        let start_minutes = state.clock.minutes;
        let mana_before = state.spellbook.mana;

        let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
        assert_eq!(state.clock.turn, start_turn);
        assert_eq!(state.clock.minutes, start_minutes);
        assert_eq!(state.spellbook.mana, mana_before);

        let choose = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(state.pending_spell_interaction.is_none());
        assert!(state.pending_targeting_interaction.is_some());
        assert_eq!(choose.turn, start_turn);
        assert_eq!(choose.minutes, start_minutes);

        let commit = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);
        assert!(state.pending_targeting_interaction.is_none());
        assert_eq!(commit.turn, start_turn + 1);
        assert_eq!(commit.minutes, start_minutes + 20);
        assert!(state.spellbook.mana < mana_before);
    }

    #[test]
    fn fear_blocks_spellcasting_attempt() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        state.status_effects.push(StatusEffect {
            id: "fear".to_string(),
            remaining_turns: 2,
            magnitude: 1,
        });
        let mana_before = state.spellbook.mana;

        let out = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);

        assert!(state.pending_spell_interaction.is_none());
        assert_eq!(state.spellbook.mana, mana_before);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "m" && note.contains("too afraid")
        )));
    }

    #[test]
    fn lunarity_negative_can_block_cast_with_contrary_moon_message() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        for spell in &mut state.spellbook.spells {
            spell.known = true;
        }
        state.progression.lunarity = -1;
        state.spellbook.mana = 15;

        let _ = step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "magic missile".to_string() }, &mut rng);
        let out = step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

        assert_eq!(state.spellbook.mana, 15);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, .. }
                if token == "m" && note.contains("contrary moon")
        )));
    }

    #[test]
    fn carry_burden_blocks_movement_when_over_limit() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        let mut rng = FixedRng::new(vec![]);
        state.carry_burden = (state.player.inventory_capacity as i32) * 20;
        let pos_before = state.player.position;

        let out = step(&mut state, Command::Move(Direction::East), &mut rng);
        assert_eq!(state.player.position, pos_before);
        assert!(out.events.iter().any(|event| matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn move_into_adjacent_monster_triggers_attack_not_block() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut rng = FixedRng::new(vec![3]);
        let target = Position { x: state.player.position.x + 1, y: state.player.position.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
        );

        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert!(out.events.iter().any(|event| {
            matches!(event, Event::Attacked { .. } | Event::MonsterDefeated { .. })
        }));
        assert!(out.events.iter().all(|event| !matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn move_into_adjacent_monster_does_not_change_position() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut rng = FixedRng::new(vec![3]);
        let start = state.player.position;
        let target = Position { x: start.x + 1, y: start.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
        );

        let _ = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(state.player.position, start);
    }

    #[test]
    fn move_into_adjacent_monster_uses_move_time_budget() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut rng = FixedRng::new(vec![3]);
        let target = Position { x: state.player.position.x + 1, y: state.player.position.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
        );

        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert_eq!(out.minutes, 5);
        assert_eq!(state.clock.minutes, 5);
    }

    #[test]
    fn overburdened_player_can_still_bump_attack_if_monster_adjacent() {
        let mut state = GameState::new(MapBounds { width: 7, height: 7 });
        let mut rng = FixedRng::new(vec![3]);
        state.carry_burden = (state.player.inventory_capacity as i32) * 20;
        let target = Position { x: state.player.position.x + 1, y: state.player.position.y };
        state.spawn_monster(
            "rat",
            target,
            Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
        );

        let out = step(&mut state, Command::Move(Direction::East), &mut rng);

        assert!(out.events.iter().any(|event| {
            matches!(event, Event::Attacked { .. } | Event::MonsterDefeated { .. })
        }));
        assert!(out.events.iter().all(|event| !matches!(event, Event::MoveBlocked { .. })));
    }

    #[test]
    fn social_lawful_monster_respects_lawful_alignment() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.progression.alignment = Alignment::Lawful;
        state.spawn_monster(
            "oracle-priest",
            Position { x: state.player.position.x + 1, y: state.player.position.y },
            Stats { hp: 8, max_hp: 8, attack_min: 2, attack_max: 2, defense: 1 },
        );
        let mut rng = FixedRng::new(vec![]);
        let hp_before = state.player.stats.hp;
        let out = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(state.player.stats.hp, hp_before);
        assert!(out.events.iter().any(|event| matches!(event, Event::DialogueAdvanced { .. })));
    }

    #[test]
    fn caster_monster_projectile_hits_player_when_los_clear() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.stats.hp = 30;
        state.player.stats.max_hp = 30;
        state.player.stats.defense = 0;
        let monster_id = state.spawn_monster(
            "warlock",
            Position { x: 6, y: 2 },
            Stats { hp: 10, max_hp: 10, attack_min: 6, attack_max: 6, defense: 0 },
        );
        if let Some(monster) = state.monsters.iter_mut().find(|monster| monster.id == monster_id) {
            monster.behavior = MonsterBehavior::Caster;
            monster.faction = Faction::Wild;
        }

        let mut rng = FixedRng::new(vec![0, 6]);
        let hp_before = state.player.stats.hp;
        let out = step(&mut state, Command::Wait, &mut rng);

        assert!(state.player.stats.hp < hp_before);
        assert!(out.events.iter().any(|event| matches!(event, Event::MonsterAttacked { .. })));
        assert!(state.log.iter().any(|line| line.contains("magic missile")));
    }

    #[test]
    fn caster_monster_projectile_is_blocked_by_portcullis() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.player.position = Position { x: 2, y: 2 };
        state.player.stats.hp = 30;
        state.player.stats.max_hp = 30;
        let monster_id = state.spawn_monster(
            "warlock",
            Position { x: 6, y: 2 },
            Stats { hp: 10, max_hp: 10, attack_min: 6, attack_max: 6, defense: 0 },
        );
        if let Some(monster) = state.monsters.iter_mut().find(|monster| monster.id == monster_id) {
            monster.behavior = MonsterBehavior::Caster;
            monster.faction = Faction::Wild;
        }
        let blocker_index = (2 * state.bounds.width + 4) as usize;
        if let Some(cell) = state.site_grid.get_mut(blocker_index) {
            cell.flags |= TILE_FLAG_BLOCK_MOVE | TILE_FLAG_PORTCULLIS;
        }
        let _ = state.set_map_glyph_at(Position { x: 4, y: 2 }, '=');
        state.city_site_grid = state.site_grid.clone();

        let mut rng = FixedRng::new(vec![0, 6]);
        let hp_before = state.player.stats.hp;
        let out = step(&mut state, Command::Wait, &mut rng);

        assert_eq!(state.player.stats.hp, hp_before);
        assert!(out.events.iter().all(|event| !matches!(event, Event::MonsterAttacked { .. })));
        assert!(state.log.iter().any(|line| line.contains("blocked")));
    }

    #[test]
    fn equipped_weapon_increases_attack_damage_output() {
        let mut baseline = GameState::new(MapBounds { width: 9, height: 9 });
        baseline.player.position = Position { x: 4, y: 4 };
        baseline.player.stats.attack_min = 4;
        baseline.player.stats.attack_max = 4;
        baseline.spawn_monster(
            "dummy",
            Position { x: 5, y: 4 },
            Stats { hp: 30, max_hp: 30, attack_min: 1, attack_max: 1, defense: 0 },
        );
        let mut rng = FixedRng::new(vec![4]);
        let out = step(&mut baseline, Command::Attack(Direction::East), &mut rng);
        let base_damage = out
            .events
            .iter()
            .find_map(|event| match event {
                Event::Attacked { damage, .. } => Some(*damage),
                _ => None,
            })
            .unwrap_or(0);

        let mut armed = GameState::new(MapBounds { width: 9, height: 9 });
        armed.player.position = Position { x: 4, y: 4 };
        armed.player.stats.attack_min = 4;
        armed.player.stats.attack_max = 4;
        armed.place_item("Victrix", armed.player.position);
        let mut rng_arm = FixedRng::new(vec![]);
        let _ = step(&mut armed, Command::Pickup, &mut rng_arm);
        armed.spawn_monster(
            "dummy",
            Position { x: 5, y: 4 },
            Stats { hp: 80, max_hp: 80, attack_min: 1, attack_max: 1, defense: 0 },
        );
        let mut rng_attack = FixedRng::new(vec![4]);
        let out_armed = step(&mut armed, Command::Attack(Direction::East), &mut rng_attack);
        let armed_damage = out_armed
            .events
            .iter()
            .find_map(|event| match event {
                Event::Attacked { damage, .. } => Some(*damage),
                _ => None,
            })
            .unwrap_or(0);

        assert!(armed_damage > base_damage, "weapon should increase outgoing damage");
    }

    #[test]
    fn equipped_armor_reduces_incoming_damage() {
        let mut baseline = GameState::new(MapBounds { width: 9, height: 9 });
        baseline.player.position = Position { x: 4, y: 4 };
        baseline.player.stats.hp = 40;
        baseline.player.stats.max_hp = 40;
        baseline.spawn_monster(
            "dummy",
            Position { x: 5, y: 4 },
            Stats { hp: 30, max_hp: 30, attack_min: 8, attack_max: 8, defense: 0 },
        );
        let mut rng = FixedRng::new(vec![8]);
        let _ = step(&mut baseline, Command::Wait, &mut rng);
        let baseline_hp = baseline.player.stats.hp;

        let mut armored = GameState::new(MapBounds { width: 9, height: 9 });
        armored.player.position = Position { x: 4, y: 4 };
        armored.player.stats.hp = 40;
        armored.player.stats.max_hp = 40;
        armored.place_item("full plate mail", armored.player.position);
        armored.place_item("tower shield", armored.player.position);
        let mut rng_equip = FixedRng::new(vec![]);
        let _ = step(&mut armored, Command::Pickup, &mut rng_equip);
        let _ = step(&mut armored, Command::Pickup, &mut rng_equip);
        armored.spawn_monster(
            "dummy",
            Position { x: 5, y: 4 },
            Stats { hp: 30, max_hp: 30, attack_min: 8, attack_max: 8, defense: 0 },
        );
        let mut rng_hit = FixedRng::new(vec![8]);
        let _ = step(&mut armored, Command::Wait, &mut rng_hit);
        let armored_hp = armored.player.stats.hp;

        assert!(armored_hp > baseline_hp, "armor/shield should mitigate incoming damage");
    }

    #[test]
    fn potions_can_heal_and_harm() {
        let mut state = GameState::new(MapBounds { width: 9, height: 9 });
        state.player.stats.max_hp = 30;
        state.player.stats.hp = 10;
        state.player.inventory.push(Item {
            id: 1,
            name: "potion of healing".to_string(),
            family: ItemFamily::Potion,
            usef: "I_HEAL".to_string(),
            ..Item::default()
        });
        state.player.inventory.push(Item {
            id: 2,
            name: "potion of poison".to_string(),
            family: ItemFamily::Potion,
            usef: "I_POISON_FOOD".to_string(),
            aux: 5,
            ..Item::default()
        });
        let mut rng = FixedRng::new(vec![]);

        let _ = step(&mut state, Command::Legacy { token: "q".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        assert!(state.player.stats.hp > 10, "healing potion should recover hp");
        let hp_after_heal = state.player.stats.hp;

        let _ = step(&mut state, Command::Legacy { token: "q".to_string() }, &mut rng);
        let _ = step(&mut state, Command::Legacy { token: "a".to_string() }, &mut rng);
        assert!(
            state.player.stats.hp < hp_after_heal,
            "harmful potion should reduce hp or apply harmful status"
        );
    }

    #[test]
    fn rings_provide_magic_resistance_effects() {
        let mut baseline = GameState::new(MapBounds { width: 9, height: 9 });
        baseline.player.position = Position { x: 4, y: 4 };
        baseline.player.stats.hp = 30;
        baseline.player.stats.max_hp = 30;
        baseline.traps.push(Trap {
            id: 1,
            position: baseline.player.position,
            damage: 6,
            effect_id: "poison".to_string(),
            armed: true,
        });
        let mut rng_base = FixedRng::new(vec![]);
        let _ = step(&mut baseline, Command::Wait, &mut rng_base);
        let hp_baseline = baseline.player.stats.hp;

        let mut ringed = GameState::new(MapBounds { width: 9, height: 9 });
        ringed.player.position = Position { x: 4, y: 4 };
        ringed.player.stats.hp = 30;
        ringed.player.stats.max_hp = 30;
        ringed.place_item("ring of poison resistance", ringed.player.position);
        let mut rng_pick = FixedRng::new(vec![]);
        let _ = step(&mut ringed, Command::Pickup, &mut rng_pick);
        ringed.traps.push(Trap {
            id: 2,
            position: ringed.player.position,
            damage: 6,
            effect_id: "poison".to_string(),
            armed: true,
        });
        let mut rng_ringed = FixedRng::new(vec![]);
        let _ = step(&mut ringed, Command::Wait, &mut rng_ringed);
        let hp_ringed = ringed.player.stats.hp;

        assert!(hp_ringed > hp_baseline, "ring magic should improve magical/poison survivability");
    }

    #[test]
    fn item_usef_dispatch_covers_legacy_catalog_without_fallbacks() {
        let unique_usef: BTreeSet<String> = legacy_item_templates()
            .iter()
            .map(|template| template.usef.trim().to_string())
            .filter(|usef| !usef.is_empty())
            .collect();

        let mut missing = Vec::new();
        for usef in unique_usef {
            let mut state = GameState::new(MapBounds { width: 9, height: 9 });
            state.player.position = Position { x: 4, y: 4 };
            state.spawn_monster(
                "target dummy",
                Position { x: 5, y: 4 },
                Stats { hp: 8, max_hp: 8, attack_min: 1, attack_max: 1, defense: 0 },
            );
            state.place_item("food ration", Position { x: 4, y: 5 });
            state.traps.push(Trap {
                id: 77,
                position: Position { x: 4, y: 4 },
                damage: 1,
                effect_id: "poison".to_string(),
                armed: true,
            });

            let mut events = Vec::new();
            let item = Item {
                id: 9999,
                name: format!("probe-{usef}"),
                usef: usef.clone(),
                family: ItemFamily::Thing,
                ..Item::default()
            };
            let note = apply_item_usef_effect(&mut state, &item, &mut events);
            if note.contains("unrecognized item effect") || note.contains("modeled fallback") {
                missing.push(usef);
            }
        }

        assert!(
            missing.is_empty(),
            "legacy usef handlers missing explicit runtime mapping: {:?}",
            missing
        );
    }

    fn direction_strategy() -> impl Strategy<Value = Direction> {
        prop_oneof![
            Just(Direction::North),
            Just(Direction::South),
            Just(Direction::East),
            Just(Direction::West),
        ]
    }

    fn command_strategy() -> impl Strategy<Value = Command> {
        prop_oneof![
            Just(Command::Wait),
            direction_strategy().prop_map(Command::Move),
            direction_strategy().prop_map(Command::Attack),
            Just(Command::Pickup),
            (0usize..20).prop_map(|slot| Command::Drop { slot }),
        ]
    }

    proptest! {
        #[test]
        fn prop_time_advances_per_command(seed in any::<u64>(), commands in prop::collection::vec(command_strategy(), 0..128)) {
            let mut state = GameState::default();
            let mut rng = DeterministicRng::seeded(seed);
            let start_turn = state.clock.turn;
            let start_minutes = state.clock.minutes;

            for command in &commands {
                let _ = step(&mut state, command.clone(), &mut rng);
            }

            // Time advances only while session is in progress and remains monotonic.
            prop_assert!(state.clock.turn >= start_turn);
            prop_assert!(state.clock.minutes >= start_minutes);
            prop_assert!(state.clock.minutes <= start_minutes + (commands.len() as u64 * 180));
        }

        #[test]
        fn prop_player_remains_in_bounds_after_moves(seed in any::<u64>(), moves in prop::collection::vec(direction_strategy(), 0..256)) {
            let mut state = GameState::new(MapBounds { width: 21, height: 13 });
            let mut rng = DeterministicRng::seeded(seed);

            for direction in moves {
                let _ = step(&mut state, Command::Move(direction), &mut rng);
                prop_assert!(state.bounds.contains(state.player.position));
            }
        }
    }
}
