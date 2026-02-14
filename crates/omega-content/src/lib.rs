use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result, anyhow, bail};
use omega_core::{
    COUNTRY_SITE_CASTLE, COUNTRY_SITE_CAVES, COUNTRY_SITE_CITY, COUNTRY_SITE_DRAGON_LAIR,
    COUNTRY_SITE_MAGIC_ISLE, COUNTRY_SITE_NONE, COUNTRY_SITE_PALACE, COUNTRY_SITE_STARPEAK,
    COUNTRY_SITE_TEMPLE, COUNTRY_SITE_VILLAGE, COUNTRY_SITE_VOLCANO, CountryCell, CountryGrid,
    CountryTerrainKind, GameMode, GameState, LegacyEnvironment, MapBinding, MapBounds,
    MapSemanticKind, Position, SITE_AUX_ALTAR_ATHENA, SITE_AUX_ALTAR_DESTINY,
    SITE_AUX_ALTAR_HECATE, SITE_AUX_ALTAR_ODIN, SITE_AUX_ALTAR_SET, SITE_AUX_EXIT_ARENA,
    SITE_AUX_EXIT_COUNTRYSIDE, SITE_AUX_SERVICE_ARENA, SITE_AUX_SERVICE_ARMORER,
    SITE_AUX_SERVICE_BANK, SITE_AUX_SERVICE_BROTHEL, SITE_AUX_SERVICE_CASINO,
    SITE_AUX_SERVICE_CASTLE, SITE_AUX_SERVICE_CHARITY, SITE_AUX_SERVICE_CLUB,
    SITE_AUX_SERVICE_COLLEGE, SITE_AUX_SERVICE_COMMANDANT, SITE_AUX_SERVICE_CONDO,
    SITE_AUX_SERVICE_CRAPS, SITE_AUX_SERVICE_DINER, SITE_AUX_SERVICE_GYM, SITE_AUX_SERVICE_HEALER,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_MONASTERY, SITE_AUX_SERVICE_ORDER,
    SITE_AUX_SERVICE_PAWN_SHOP, SITE_AUX_SERVICE_SHOP, SITE_AUX_SERVICE_SORCERORS,
    SITE_AUX_SERVICE_TAVERN, SITE_AUX_SERVICE_TEMPLE, SITE_AUX_SERVICE_THIEVES, SiteMapDefinition,
    TILE_FLAG_BLOCK_MOVE, TILE_FLAG_NO_CITY_MOVE, TILE_FLAG_PORTCULLIS, TILE_FLAG_SECRET,
    TileSiteCell,
};
use serde::{Deserialize, Serialize};

pub const LEGACY_MAP_VERSION: u8 = 2;
pub const MAX_MAP_DIMENSION: usize = 256;
pub const LEGACY_CITY_MAP_ID: u16 = 3;
pub const LEGACY_COUNTRY_MAP_ID: u16 = 0;
pub const LEGACY_RAMPART_START: Position = Position { x: 62, y: 20 };

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentPack {
    pub id: String,
    pub maps: Vec<LegacyMap>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContentPackId {
    Classic,
    Modern,
}

impl ContentPackId {
    pub fn as_str(self) -> &'static str {
        match self {
            ContentPackId::Classic => "classic",
            ContentPackId::Modern => "modern",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyMap {
    pub source: String,
    pub map_id: u16,
    pub width: u16,
    pub height: u16,
    pub levels: Vec<MapLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MapLevel {
    pub level_index: u16,
    pub rows: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    pub source: Option<String>,
    pub map_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationReport {
    pub map_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootstrapDiagnostics {
    pub map_source: String,
    pub player_spawn_source: String,
    pub monster_spawns: usize,
    pub item_spawns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyCatalogEntry {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum LegacyItemFamily {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyItemPrototype {
    pub source_index: usize,
    pub family: LegacyItemFamily,
    pub family_index: u16,
    pub id_expr: String,
    pub id_value: u16,
    pub weight: i32,
    pub plus: i32,
    pub charge: i32,
    pub dmg: i32,
    pub hit: i32,
    pub aux: i32,
    pub number: i32,
    pub fragility: i32,
    pub basevalue: i64,
    pub known: u8,
    pub used: u8,
    pub blessing: i32,
    pub item_type: String,
    pub uniqueness: String,
    pub usef: String,
    pub level: u8,
    pub objchar: String,
    pub objstr: String,
    pub truename: String,
    pub cursestr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyItemCatalogs {
    pub scrolls: Vec<LegacyCatalogEntry>,
    pub potions: Vec<LegacyCatalogEntry>,
    pub foods: Vec<LegacyCatalogEntry>,
    pub weapons: Vec<LegacyCatalogEntry>,
    pub armor: Vec<LegacyCatalogEntry>,
    pub shields: Vec<LegacyCatalogEntry>,
    pub cloaks: Vec<LegacyCatalogEntry>,
    pub boots: Vec<LegacyCatalogEntry>,
    pub rings: Vec<LegacyCatalogEntry>,
    pub sticks: Vec<LegacyCatalogEntry>,
    pub artifacts: Vec<LegacyCatalogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegacyCatalogs {
    pub spells: Vec<LegacyCatalogEntry>,
    pub monsters: Vec<LegacyCatalogEntry>,
    pub traps: Vec<LegacyCatalogEntry>,
    pub city_sites: Vec<LegacyCatalogEntry>,
    pub items: LegacyItemCatalogs,
}

pub fn legacy_catalogs() -> LegacyCatalogs {
    static CATALOGS: OnceLock<LegacyCatalogs> = OnceLock::new();
    CATALOGS.get_or_init(build_legacy_catalogs).clone()
}

pub fn legacy_item_prototypes() -> Vec<LegacyItemPrototype> {
    static ITEMS: OnceLock<Vec<LegacyItemPrototype>> = OnceLock::new();
    ITEMS.get_or_init(parse_item_prototypes).clone()
}

const LEGACY_SPELL_C: &str = include_str!("../../../archive/legacy-c-runtime/2026-02-06/spell.c");
const LEGACY_MINIT_H: &str = include_str!("../../../archive/legacy-c-runtime/2026-02-06/minit.h");
const LEGACY_IINIT_H: &str = include_str!("../../../archive/legacy-c-runtime/2026-02-06/iinit.h");
const LEGACY_DEFS_H: &str = include_str!("../../../archive/legacy-c-runtime/2026-02-06/defs.h");

fn build_legacy_catalogs() -> LegacyCatalogs {
    LegacyCatalogs {
        spells: parse_spell_catalog(),
        monsters: parse_monster_catalog(),
        traps: parse_trap_catalog(),
        city_sites: parse_city_site_catalog(),
        items: parse_item_catalogs(),
    }
}

fn parse_spell_catalog() -> Vec<LegacyCatalogEntry> {
    let mut entries = Vec::new();
    let Some(start) = LEGACY_SPELL_C.find("static char *spell_names[]") else {
        return entries;
    };
    let tail = &LEGACY_SPELL_C[start..];
    let Some(end) = tail.find("};") else {
        return entries;
    };
    let block = &tail[..end];
    for name in parse_quoted_fragments(block) {
        if let Some(name) = sanitize_catalog_name(&name) {
            entries.push(LegacyCatalogEntry { id: (entries.len() + 1) as u16, name });
        }
    }
    entries
}

fn parse_monster_catalog() -> Vec<LegacyCatalogEntry> {
    let mut entries = Vec::new();
    for line in LEGACY_MINIT_H.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("{ NULL,") {
            continue;
        }
        let quoted = parse_quoted_fragments(trimmed);
        let Some(name) = quoted.first().and_then(|value| sanitize_catalog_name(value)) else {
            continue;
        };
        entries.push(LegacyCatalogEntry { id: (entries.len() + 1) as u16, name });
    }
    entries
}

fn parse_item_catalogs() -> LegacyItemCatalogs {
    let prototypes = legacy_item_prototypes();
    let mut scrolls = Vec::new();
    let mut potions = Vec::new();
    let mut foods = Vec::new();
    let mut weapons = Vec::new();
    let mut armor = Vec::new();
    let mut shields = Vec::new();
    let mut cloaks = Vec::new();
    let mut boots = Vec::new();
    let mut rings = Vec::new();
    let mut sticks = Vec::new();
    let mut artifacts = Vec::new();

    for proto in prototypes {
        let name = sanitize_catalog_name(&proto.truename)
            .or_else(|| sanitize_catalog_name(&proto.objstr))
            .unwrap_or_else(|| proto.truename.clone());
        let entry = LegacyCatalogEntry { id: proto.family_index + 1, name };
        match proto.family {
            LegacyItemFamily::Scroll => scrolls.push(entry),
            LegacyItemFamily::Potion => potions.push(entry),
            LegacyItemFamily::Food => foods.push(entry),
            LegacyItemFamily::Weapon => weapons.push(entry),
            LegacyItemFamily::Armor => armor.push(entry),
            LegacyItemFamily::Shield => shields.push(entry),
            LegacyItemFamily::Cloak => cloaks.push(entry),
            LegacyItemFamily::Boots => boots.push(entry),
            LegacyItemFamily::Ring => rings.push(entry),
            LegacyItemFamily::Stick => sticks.push(entry),
            LegacyItemFamily::Artifact => artifacts.push(entry),
            LegacyItemFamily::Thing | LegacyItemFamily::Cash | LegacyItemFamily::Corpse => {}
        }
    }

    LegacyItemCatalogs {
        scrolls,
        potions,
        foods,
        weapons,
        armor,
        shields,
        cloaks,
        boots,
        rings,
        sticks,
        artifacts,
    }
}

fn parse_item_prototypes() -> Vec<LegacyItemPrototype> {
    let family_bases = legacy_item_family_base_ids();
    let mut entries = Vec::new();

    for (line_number, line) in LEGACY_IINIT_H.lines().enumerate() {
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
        let Some((family, family_index)) = parse_family_and_index(&id_expr) else {
            continue;
        };
        let Some(base_id) = family_bases.get(&family).copied() else {
            continue;
        };
        let id_value = base_id.saturating_add(family_index);

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
        let Some(known) = parse_u8_token(&fields[10]) else {
            continue;
        };
        let Some(used) = parse_u8_token(&fields[11]) else {
            continue;
        };
        let Some(blessing) = parse_i32_token(&fields[12]) else {
            continue;
        };
        let Some(level) = parse_u8_token(&fields[16]) else {
            continue;
        };
        let objstr = parse_string_token(&fields[18]).unwrap_or_default();
        let truename = parse_string_token(&fields[19]).unwrap_or_default();
        let cursestr = parse_string_token(&fields[20]).unwrap_or_default();

        entries.push(LegacyItemPrototype {
            source_index: line_number + 1,
            family,
            family_index,
            id_expr,
            id_value,
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
            item_type: fields[13].trim().to_string(),
            uniqueness: fields[14].trim().to_string(),
            usef: fields[15].trim().to_string(),
            level,
            objchar: fields[17].trim().to_string(),
            objstr,
            truename,
            cursestr,
        });
    }

    entries
}

fn legacy_item_family_base_ids() -> HashMap<LegacyItemFamily, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();
    for line in LEGACY_DEFS_H.lines() {
        let Some((macro_name, value_raw)) = parse_define(line.trim()) else {
            continue;
        };
        if !macro_name.starts_with("NUM") {
            continue;
        }
        let number = value_raw
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse::<u16>()
            .ok();
        if let Some(parsed) = number {
            values.insert(macro_name.to_string(), parsed);
        }
    }

    let num_things = *values.get("NUMTHINGS").unwrap_or(&31);
    let num_foods = *values.get("NUMFOODS").unwrap_or(&16);
    let num_scrolls = *values.get("NUMSCROLLS").unwrap_or(&24);
    let num_potions = *values.get("NUMPOTIONS").unwrap_or(&18);
    let num_weapons = *values.get("NUMWEAPONS").unwrap_or(&41);
    let num_armor = *values.get("NUMARMOR").unwrap_or(&17);
    let num_shields = *values.get("NUMSHIELDS").unwrap_or(&8);
    let num_cloaks = *values.get("NUMCLOAKS").unwrap_or(&7);
    let num_boots = *values.get("NUMBOOTS").unwrap_or(&7);
    let num_rings = *values.get("NUMRINGS").unwrap_or(&9);
    let num_sticks = *values.get("NUMSTICKS").unwrap_or(&17);
    let num_artifacts = *values.get("NUMARTIFACTS").unwrap_or(&26);

    let mut bases = HashMap::new();
    let thing = 0u16;
    let food = thing + num_things;
    let scroll = food + num_foods;
    let potion = scroll + num_scrolls;
    let weapon = potion + num_potions;
    let armor = weapon + num_weapons;
    let shield = armor + num_armor;
    let cloak = shield + num_shields;
    let boots = cloak + num_cloaks;
    let ring = boots + num_boots;
    let stick = ring + num_rings;
    let artifact = stick + num_sticks;
    let cash = artifact + num_artifacts;
    let corpse = cash + 1;

    bases.insert(LegacyItemFamily::Thing, thing);
    bases.insert(LegacyItemFamily::Food, food);
    bases.insert(LegacyItemFamily::Scroll, scroll);
    bases.insert(LegacyItemFamily::Potion, potion);
    bases.insert(LegacyItemFamily::Weapon, weapon);
    bases.insert(LegacyItemFamily::Armor, armor);
    bases.insert(LegacyItemFamily::Shield, shield);
    bases.insert(LegacyItemFamily::Cloak, cloak);
    bases.insert(LegacyItemFamily::Boots, boots);
    bases.insert(LegacyItemFamily::Ring, ring);
    bases.insert(LegacyItemFamily::Stick, stick);
    bases.insert(LegacyItemFamily::Artifact, artifact);
    bases.insert(LegacyItemFamily::Cash, cash);
    bases.insert(LegacyItemFamily::Corpse, corpse);
    bases
}

fn parse_trap_catalog() -> Vec<LegacyCatalogEntry> {
    let mut entries = Vec::new();
    for line in LEGACY_DEFS_H.lines() {
        let trimmed = line.trim();
        let Some((macro_name, value)) = parse_define(trimmed) else {
            continue;
        };
        if !macro_name.starts_with("L_TRAP_") {
            continue;
        }
        let Ok(raw_id) = value.parse::<u16>() else {
            continue;
        };
        let suffix = macro_name.trim_start_matches("L_TRAP_");
        entries.push((raw_id, prettify_macro_suffix(suffix)));
    }
    entries.sort_by_key(|(id, _)| *id);
    entries
        .into_iter()
        .enumerate()
        .map(|(idx, (_, name))| LegacyCatalogEntry { id: (idx + 1) as u16, name })
        .collect()
}

fn parse_city_site_catalog() -> Vec<LegacyCatalogEntry> {
    let mut entries = Vec::new();
    for line in LEGACY_DEFS_H.lines() {
        let trimmed = line.trim();
        let Some((macro_name, value)) = parse_define(trimmed) else {
            continue;
        };
        if !macro_name.starts_with("L_") || macro_name.starts_with("L_TRAP_") {
            continue;
        }
        let Ok(raw_id) = value.parse::<u16>() else {
            continue;
        };
        if !(7..=36).contains(&raw_id) {
            continue;
        }
        let suffix = macro_name.trim_start_matches("L_");
        entries.push((raw_id, prettify_macro_suffix(suffix)));
    }
    entries.sort_by_key(|(id, _)| *id);
    entries
        .into_iter()
        .enumerate()
        .map(|(idx, (_, name))| LegacyCatalogEntry { id: (idx + 1) as u16, name })
        .collect()
}

fn parse_define(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    if parts.next()? != "#define" {
        return None;
    }
    let macro_name = parts.next()?;
    let value = parts.next()?;
    Some((macro_name, value))
}

fn parse_family_and_index(expr: &str) -> Option<(LegacyItemFamily, u16)> {
    let expr = expr.trim();
    if let Some(index) = expr.strip_prefix("THINGID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Thing, index));
    }
    if let Some(index) = expr.strip_prefix("FOODID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Food, index));
    }
    if let Some(index) = expr.strip_prefix("SCROLLID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Scroll, index));
    }
    if let Some(index) = expr.strip_prefix("POTIONID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Potion, index));
    }
    if let Some(index) = expr.strip_prefix("WEAPONID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Weapon, index));
    }
    if let Some(index) = expr.strip_prefix("ARMORID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Armor, index));
    }
    if let Some(index) = expr.strip_prefix("SHIELDID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Shield, index));
    }
    if let Some(index) = expr.strip_prefix("CLOAKID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Cloak, index));
    }
    if let Some(index) = expr.strip_prefix("BOOTID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Boots, index));
    }
    if let Some(index) = expr.strip_prefix("RINGID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Ring, index));
    }
    if let Some(index) = expr.strip_prefix("STICKID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Stick, index));
    }
    if let Some(index) = expr.strip_prefix("ARTIFACTID+").and_then(|v| v.parse::<u16>().ok()) {
        return Some((LegacyItemFamily::Artifact, index));
    }
    if expr == "CASHID" {
        return Some((LegacyItemFamily::Cash, 0));
    }
    if expr == "CORPSEID" {
        return Some((LegacyItemFamily::Corpse, 0));
    }
    None
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

fn parse_quoted_fragments(raw: &str) -> Vec<String> {
    raw.split('"').skip(1).step_by(2).map(ToString::to_string).collect()
}

fn sanitize_catalog_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "(null)" || trimmed == "?" {
        return None;
    }
    Some(trimmed.replace('_', " "))
}

fn prettify_macro_suffix(raw: &str) -> String {
    raw.to_ascii_lowercase().replace('_', " ")
}

fn legacy_item_name_pool(catalogs: &LegacyCatalogs) -> Vec<String> {
    let mut names = Vec::new();
    names.extend(catalogs.items.scrolls.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.potions.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.foods.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.weapons.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.armor.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.shields.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.cloaks.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.boots.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.rings.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.sticks.iter().map(|entry| entry.name.clone()));
    names.extend(catalogs.items.artifacts.iter().map(|entry| entry.name.clone()));
    names
}

fn city_site_lookup(catalogs: &LegacyCatalogs) -> HashMap<String, u16> {
    catalogs
        .city_sites
        .iter()
        .map(|entry| (entry.name.to_ascii_lowercase(), entry.id))
        .collect::<HashMap<_, _>>()
}

fn site_id(lookup: &HashMap<String, u16>, name: &str) -> u16 {
    lookup.get(&name.to_ascii_lowercase()).copied().unwrap_or(0)
}

#[derive(Debug, Clone, Copy)]
enum CityDoorState {
    Open,
    Closed,
}

#[derive(Debug, Clone, Copy)]
struct CityGenericAssignment {
    site_name: Option<&'static str>,
    aux: i32,
    door_state: CityDoorState,
}

fn city_assignment_seed(rows: &[String]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for row in rows {
        for byte in row.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash ^ ((rows.len() as u64) << 32)
}

fn shuffle_city_permutation(seed: u64) -> Vec<usize> {
    let mut permutation: Vec<usize> = (0..64).collect();
    let mut state = seed;
    for _ in 0..500 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = ((state >> 32) as usize) % permutation.len();
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let k = ((state >> 32) as usize) % permutation.len();
        permutation.swap(j, k);
    }
    permutation
}

fn residence_site_for_slot(slot: usize) -> &'static str {
    match slot % 6 {
        0 => "hovel",
        5 => "mansion",
        _ => "house",
    }
}

fn city_generic_assignment(permutation: &[usize], index: usize) -> CityGenericAssignment {
    let Some(slot) = permutation.get(index).copied() else {
        return CityGenericAssignment {
            site_name: Some("house"),
            aux: 0,
            door_state: CityDoorState::Closed,
        };
    };

    match slot {
        0 => CityGenericAssignment {
            site_name: Some("armorer"),
            aux: SITE_AUX_SERVICE_ARMORER,
            door_state: CityDoorState::Open,
        },
        1 => CityGenericAssignment {
            site_name: Some("club"),
            aux: SITE_AUX_SERVICE_CLUB,
            door_state: CityDoorState::Open,
        },
        2 => CityGenericAssignment {
            site_name: Some("gym"),
            aux: SITE_AUX_SERVICE_GYM,
            door_state: CityDoorState::Open,
        },
        3 => CityGenericAssignment {
            site_name: Some("thieves guild"),
            aux: SITE_AUX_SERVICE_THIEVES,
            door_state: CityDoorState::Closed,
        },
        4 => CityGenericAssignment {
            site_name: Some("healer"),
            aux: SITE_AUX_SERVICE_HEALER,
            door_state: CityDoorState::Open,
        },
        5 => CityGenericAssignment {
            site_name: Some("casino"),
            aux: SITE_AUX_SERVICE_CASINO,
            door_state: CityDoorState::Open,
        },
        6 | 9 | 20 => CityGenericAssignment {
            site_name: Some("commandant"),
            aux: SITE_AUX_SERVICE_COMMANDANT,
            door_state: CityDoorState::Open,
        },
        7 => CityGenericAssignment {
            site_name: Some("diner"),
            aux: SITE_AUX_SERVICE_DINER,
            door_state: CityDoorState::Open,
        },
        8 => CityGenericAssignment {
            site_name: Some("crap"),
            aux: SITE_AUX_SERVICE_CRAPS,
            door_state: CityDoorState::Open,
        },
        10 => CityGenericAssignment {
            site_name: Some("alchemist"),
            aux: SITE_AUX_SERVICE_SORCERORS,
            door_state: CityDoorState::Open,
        },
        11 => CityGenericAssignment {
            site_name: Some("dpw"),
            aux: SITE_AUX_SERVICE_ORDER,
            door_state: CityDoorState::Open,
        },
        12 => CityGenericAssignment {
            site_name: Some("library"),
            aux: SITE_AUX_SERVICE_COLLEGE,
            door_state: CityDoorState::Open,
        },
        13 => CityGenericAssignment {
            site_name: Some("pawn shop"),
            aux: SITE_AUX_SERVICE_PAWN_SHOP,
            door_state: CityDoorState::Open,
        },
        14 => CityGenericAssignment {
            site_name: Some("condo"),
            aux: SITE_AUX_SERVICE_CONDO,
            door_state: CityDoorState::Open,
        },
        15 => CityGenericAssignment {
            site_name: Some("brothel"),
            aux: SITE_AUX_SERVICE_BROTHEL,
            door_state: CityDoorState::Closed,
        },
        16 => CityGenericAssignment {
            site_name: Some("monastery"),
            aux: SITE_AUX_SERVICE_MONASTERY,
            door_state: CityDoorState::Open,
        },
        21 => CityGenericAssignment {
            site_name: Some("tavern"),
            aux: SITE_AUX_SERVICE_TAVERN,
            door_state: CityDoorState::Open,
        },
        _ => CityGenericAssignment {
            site_name: Some(residence_site_for_slot(slot)),
            aux: 0,
            door_state: CityDoorState::Closed,
        },
    }
}

fn city_tile_from_glyph(
    glyph: char,
    lookup: &HashMap<String, u16>,
    permutation: &[usize],
    generic_index: &mut usize,
) -> TileSiteCell {
    let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };

    let mut apply_door_state = |door_state: CityDoorState| match door_state {
        CityDoorState::Open => {
            cell.glyph = '/';
            cell.flags &= !TILE_FLAG_BLOCK_MOVE;
        }
        CityDoorState::Closed => {
            cell.glyph = '-';
            cell.flags |= TILE_FLAG_BLOCK_MOVE;
        }
    };

    match glyph {
        'p' | '!' | 'I' | 'E' | 'e' | 'x' | 'K' => {
            let assignment = city_generic_assignment(permutation, *generic_index);
            *generic_index = (*generic_index).saturating_add(1);
            if let Some(site_name) = assignment.site_name {
                cell.site_id = site_id(lookup, site_name);
            }
            cell.aux = assignment.aux;
            apply_door_state(assignment.door_state);
        }
        'g' => {
            cell.site_id = site_id(lookup, "garden");
            cell.glyph = '.';
        }
        'y' => {
            cell.site_id = site_id(lookup, "cemetary");
            cell.glyph = '.';
        }
        't' => {
            cell.site_id = site_id(lookup, "temple");
            cell.aux = SITE_AUX_SERVICE_TEMPLE;
            cell.glyph = '.';
        }
        'R' => {
            cell.site_id = site_id(lookup, "raise portcullis");
            cell.flags |= TILE_FLAG_NO_CITY_MOVE;
            cell.glyph = '.';
        }
        '7' => {
            cell.site_id = site_id(lookup, "portcullis");
            cell.flags |= TILE_FLAG_NO_CITY_MOVE | TILE_FLAG_PORTCULLIS | TILE_FLAG_BLOCK_MOVE;
            cell.glyph = '.';
        }
        'C' => {
            cell.site_id = site_id(lookup, "college");
            cell.aux = SITE_AUX_SERVICE_COLLEGE;
            apply_door_state(CityDoorState::Open);
        }
        's' => {
            cell.site_id = site_id(lookup, "sorcerors");
            cell.aux = SITE_AUX_SERVICE_SORCERORS;
            apply_door_state(CityDoorState::Open);
        }
        'M' => {
            cell.site_id = site_id(lookup, "merc guild");
            cell.aux = SITE_AUX_SERVICE_MERC_GUILD;
            apply_door_state(CityDoorState::Open);
        }
        'c' => {
            cell.site_id = site_id(lookup, "castle");
            cell.aux = SITE_AUX_SERVICE_CASTLE;
            apply_door_state(CityDoorState::Open);
        }
        'P' => {
            cell.site_id = site_id(lookup, "order");
            cell.aux = SITE_AUX_SERVICE_ORDER;
            apply_door_state(CityDoorState::Open);
        }
        'H' => {
            cell.site_id = site_id(lookup, "charity");
            cell.aux = SITE_AUX_SERVICE_CHARITY;
            apply_door_state(CityDoorState::Open);
        }
        'A' => {
            cell.site_id = site_id(lookup, "arena");
            cell.aux = SITE_AUX_SERVICE_ARENA;
            apply_door_state(CityDoorState::Open);
        }
        'J' => {
            cell.site_id = site_id(lookup, "jail");
            apply_door_state(CityDoorState::Closed);
        }
        'B' => {
            cell.site_id = site_id(lookup, "bank");
            cell.aux = SITE_AUX_SERVICE_BANK;
            apply_door_state(CityDoorState::Open);
        }
        'i' => {
            cell.site_id = site_id(lookup, "tourist");
            cell.aux = SITE_AUX_SERVICE_SHOP;
            apply_door_state(CityDoorState::Open);
        }
        '2' => {
            cell.aux = SITE_AUX_ALTAR_ODIN;
            cell.glyph = '_';
        }
        '3' => {
            cell.aux = SITE_AUX_ALTAR_SET;
            cell.glyph = '_';
        }
        '4' => {
            cell.aux = SITE_AUX_ALTAR_ATHENA;
            cell.glyph = '_';
        }
        '5' => {
            cell.aux = SITE_AUX_ALTAR_HECATE;
            cell.glyph = '_';
        }
        '6' => {
            cell.aux = SITE_AUX_ALTAR_DESTINY;
            cell.glyph = '_';
        }
        'X' => {
            cell.site_id = site_id(lookup, "countryside");
            cell.aux = SITE_AUX_EXIT_COUNTRYSIDE;
            cell.glyph = '.';
        }
        'v' | 'S' | 'V' | '%' | ',' => {
            cell.site_id = site_id(lookup, "vault");
            cell.flags |= TILE_FLAG_NO_CITY_MOVE | TILE_FLAG_SECRET;
            cell.glyph = '.';
        }
        '^' => {
            cell.flags |= TILE_FLAG_SECRET;
            cell.glyph = '.';
        }
        '#' | '*' | '"' | '~' | '=' | '-' | 'D' => {
            cell.flags |= TILE_FLAG_BLOCK_MOVE;
            if glyph != '-' && glyph != 'D' {
                cell.glyph = '#';
            }
        }
        '?' | '>' | 'T' | '.' | 'h' | 'j' | 'u' | 'U' | '$' => {
            cell.glyph = '.';
        }
        _ => {}
    }
    cell
}

fn build_city_site_grid(rows: &mut [String], lookup: &HashMap<String, u16>) -> Vec<TileSiteCell> {
    let permutation = shuffle_city_permutation(city_assignment_seed(rows));
    let mut grid = Vec::new();
    let mut generic_index = 0usize;
    for row in rows.iter_mut() {
        let mut normalized = String::with_capacity(row.len());
        for glyph in row.chars() {
            let cell = city_tile_from_glyph(glyph, lookup, &permutation, &mut generic_index);
            normalized.push(cell.glyph);
            grid.push(cell);
        }
        *row = normalized;
    }
    grid
}

fn country_terrain_from_glyph(glyph: char) -> (CountryTerrainKind, CountryTerrainKind, u8) {
    match glyph {
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
    }
}

fn build_country_grid(rows: &[String], width: i32, height: i32) -> CountryGrid {
    let mut cells = Vec::new();
    for row in rows {
        for glyph in row.chars() {
            let (base_terrain, current_terrain, aux) = country_terrain_from_glyph(glyph);
            cells.push(CountryCell { glyph, base_terrain, current_terrain, aux, status: 0 });
        }
    }
    CountryGrid { width, height, cells }
}

fn build_country_site_grid(rows: &[String]) -> Vec<TileSiteCell> {
    let mut grid = Vec::new();
    for row in rows {
        for glyph in row.chars() {
            grid.push(country_site_tile_from_glyph(glyph));
        }
    }
    grid
}

fn country_site_tile_from_glyph(glyph: char) -> TileSiteCell {
    let (base_terrain, _, aux) = country_terrain_from_glyph(glyph);
    let site_id = match base_terrain {
        CountryTerrainKind::City => COUNTRY_SITE_CITY,
        CountryTerrainKind::Village => COUNTRY_SITE_VILLAGE,
        CountryTerrainKind::Temple => COUNTRY_SITE_TEMPLE,
        CountryTerrainKind::Castle => COUNTRY_SITE_CASTLE,
        CountryTerrainKind::Palace => COUNTRY_SITE_PALACE,
        CountryTerrainKind::Caves => COUNTRY_SITE_CAVES,
        CountryTerrainKind::Volcano => COUNTRY_SITE_VOLCANO,
        CountryTerrainKind::DragonLair => COUNTRY_SITE_DRAGON_LAIR,
        CountryTerrainKind::StarPeak => COUNTRY_SITE_STARPEAK,
        CountryTerrainKind::MagicIsle => COUNTRY_SITE_MAGIC_ISLE,
        _ => COUNTRY_SITE_NONE,
    };
    TileSiteCell { glyph, site_id, aux: i32::from(aux), flags: 0 }
}

fn village_tile_from_glyph(glyph: char) -> TileSiteCell {
    let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };
    match glyph {
        'x' | 'H' | 'g' => {
            cell.aux = SITE_AUX_SERVICE_SHOP;
        }
        'C' => {
            cell.aux = SITE_AUX_SERVICE_CHARITY;
        }
        'S' => {
            cell.aux = SITE_AUX_SERVICE_MERC_GUILD;
        }
        'X' => {
            cell.aux = SITE_AUX_EXIT_COUNTRYSIDE;
        }
        '"' | '\'' | '~' | '+' | '#' => {
            cell.flags |= TILE_FLAG_BLOCK_MOVE;
        }
        _ => {}
    }
    cell
}

fn temple_tile_from_glyph(glyph: char) -> TileSiteCell {
    let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };
    match glyph {
        '8' | 'H' => {
            cell.aux = SITE_AUX_SERVICE_TEMPLE;
        }
        'X' => {
            cell.aux = SITE_AUX_EXIT_COUNTRYSIDE;
        }
        '#' => {
            cell.flags |= TILE_FLAG_BLOCK_MOVE;
        }
        _ => {}
    }
    cell
}

fn generic_site_tile_from_glyph(glyph: char) -> TileSiteCell {
    let mut cell = TileSiteCell { glyph, site_id: 0, aux: 0, flags: 0 };
    match glyph {
        'X' => {
            cell.aux = SITE_AUX_EXIT_COUNTRYSIDE;
        }
        '#' | '"' | '~' | '+' | '=' => {
            cell.flags |= TILE_FLAG_BLOCK_MOVE;
        }
        _ => {}
    }
    cell
}

fn arena_tile_from_glyph(glyph: char) -> TileSiteCell {
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
    cell
}

fn build_site_grid_from_rows(
    rows: &[String],
    mapper: fn(char) -> TileSiteCell,
) -> Vec<TileSiteCell> {
    let mut grid = Vec::new();
    for row in rows {
        for glyph in row.chars() {
            grid.push(mapper(glyph));
        }
    }
    grid
}

fn map_spawn_for_id(map_id: u16) -> Option<Position> {
    match map_id {
        1 => Some(Position { x: 2, y: 7 }),    // arena
        2 => Some(Position { x: 2, y: 2 }),    // caves/circle
        4 => Some(Position { x: 32, y: 8 }),   // volcano/abyss
        5 => Some(Position { x: 32, y: 2 }),   // court/palace
        6 => Some(Position { x: 8, y: 0 }),    // dragon lair
        11 => Some(Position { x: 62, y: 14 }), // magic isle
        12 => Some(Position { x: 2, y: 8 }),   // skorch
        13 => Some(Position { x: 2, y: 9 }),   // star peak
        14 => Some(Position { x: 0, y: 6 }),   // starview
        15 => Some(Position { x: 63, y: 8 }),  // stormwat
        16 => Some(Position { x: 32, y: 15 }), // temple
        17 => Some(Position { x: 32, y: 15 }), // thaumari
        18 => Some(Position { x: 2, y: 2 }),   // whorfen
        19 => Some(Position { x: 39, y: 15 }), // woodmere
        _ => None,
    }
}

fn map_environment_for_id(map_id: u16) -> LegacyEnvironment {
    match map_id {
        1 => LegacyEnvironment::Arena,
        2 => LegacyEnvironment::Caves,
        4 => LegacyEnvironment::Volcano,
        5 => LegacyEnvironment::Castle,
        6 => LegacyEnvironment::DragonLair,
        11 => LegacyEnvironment::MagicIsle,
        13 => LegacyEnvironment::StarPeak,
        16 => LegacyEnvironment::Temple,
        12 | 14 | 15 | 17 | 18 | 19 => LegacyEnvironment::Village,
        _ => LegacyEnvironment::Unknown,
    }
}

fn build_site_map_definitions(pack: &ContentPack) -> Vec<SiteMapDefinition> {
    let mut maps = Vec::new();
    for map in &pack.maps {
        let Some(spawn) = map_spawn_for_id(map.map_id) else {
            continue;
        };
        let Some(level) = map.levels.first() else {
            continue;
        };
        let site_grid = match map.map_id {
            1 => build_site_grid_from_rows(&level.rows, arena_tile_from_glyph),
            12 | 14 | 15 | 17 | 18 | 19 => {
                build_site_grid_from_rows(&level.rows, village_tile_from_glyph)
            }
            16 => build_site_grid_from_rows(&level.rows, temple_tile_from_glyph),
            _ => build_site_grid_from_rows(&level.rows, generic_site_tile_from_glyph),
        };
        maps.push(SiteMapDefinition {
            map_id: map.map_id,
            level_index: level.level_index,
            source: map.source.clone(),
            environment: map_environment_for_id(map.map_id),
            semantic: MapSemanticKind::Site,
            spawn,
            rows: level.rows.clone(),
            site_grid,
        });
    }
    maps
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
}

pub fn content_pack_for_mode(mode: GameMode) -> ContentPackId {
    match mode {
        GameMode::Classic => ContentPackId::Classic,
        GameMode::Modern => ContentPackId::Modern,
    }
}

pub fn load_content_pack(pack_id: ContentPackId) -> Result<ContentPack> {
    let base =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("tools").join("libsrc");
    match pack_id {
        ContentPackId::Classic => {
            let mut pack = load_legacy_maps_from_dir(base)?;
            pack.id = "omega-classic-frozen".to_string();
            Ok(pack)
        }
        ContentPackId::Modern => {
            let modern_base = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("tools")
                .join("libsrc-modern");
            let mut pack = if modern_base.exists() {
                load_legacy_maps_from_dir(modern_base)?
            } else {
                load_legacy_maps_from_dir(base)?
            };
            pack.id = "omega-modern".to_string();
            Ok(pack)
        }
    }
}

pub fn load_default_content() -> Result<ContentPack> {
    load_content_pack(ContentPackId::Classic)
}

pub fn load_legacy_maps_from_dir(path: impl AsRef<Path>) -> Result<ContentPack> {
    let path = path.as_ref();
    let mut map_files: Vec<PathBuf> = fs::read_dir(path)
        .with_context(|| format!("reading legacy map directory {}", path.display()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|file| file.extension().and_then(|s| s.to_str()) == Some("map"))
        .collect();
    map_files.sort();

    if map_files.is_empty() {
        bail!("no .map files found under {}", path.display());
    }

    let mut maps = Vec::with_capacity(map_files.len());
    for map_file in map_files {
        maps.push(load_legacy_map_file(&map_file)?);
    }

    Ok(ContentPack { id: "omega-legacy-bootstrap".to_string(), maps })
}

pub fn load_legacy_map_file(path: impl AsRef<Path>) -> Result<LegacyMap> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path)
        .with_context(|| format!("reading legacy map file {}", path.display()))?;
    parse_legacy_map(&raw, path)
}

pub fn parse_legacy_map_from_str(raw: &str, source_label: &str) -> Result<LegacyMap> {
    parse_legacy_map(raw, Path::new(source_label))
}

pub fn validate_content_pack(pack: &ContentPack) -> ValidationReport {
    let mut issues = Vec::new();
    let mut seen_ids = HashSet::new();

    for map in &pack.maps {
        if !seen_ids.insert(map.map_id) {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                code: "duplicate_map_id".to_string(),
                message: format!("map id {} is reused", map.map_id),
                source: Some(map.source.clone()),
                map_id: Some(map.map_id),
            });
        }

        if map.width == 0 || map.width as usize > MAX_MAP_DIMENSION {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                code: "invalid_width".to_string(),
                message: format!(
                    "map width {} out of supported range 1..={}",
                    map.width, MAX_MAP_DIMENSION
                ),
                source: Some(map.source.clone()),
                map_id: Some(map.map_id),
            });
        }

        if map.height == 0 || map.height as usize > MAX_MAP_DIMENSION {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                code: "invalid_height".to_string(),
                message: format!(
                    "map height {} out of supported range 1..={}",
                    map.height, MAX_MAP_DIMENSION
                ),
                source: Some(map.source.clone()),
                map_id: Some(map.map_id),
            });
        }

        if map.levels.is_empty() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                code: "missing_levels".to_string(),
                message: "map has no levels".to_string(),
                source: Some(map.source.clone()),
                map_id: Some(map.map_id),
            });
        }

        for level in &map.levels {
            if level.rows.len() != map.height as usize {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    code: "invalid_level_height".to_string(),
                    message: format!(
                        "level {} has {} rows, expected {}",
                        level.level_index,
                        level.rows.len(),
                        map.height
                    ),
                    source: Some(map.source.clone()),
                    map_id: Some(map.map_id),
                });
            }

            for (row_idx, row) in level.rows.iter().enumerate() {
                if row.chars().count() != map.width as usize {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        code: "invalid_row_width".to_string(),
                        message: format!(
                            "level {} row {} has width {}, expected {}",
                            level.level_index,
                            row_idx + 1,
                            row.chars().count(),
                            map.width
                        ),
                        source: Some(map.source.clone()),
                        map_id: Some(map.map_id),
                    });
                }
            }
        }
    }

    let error_count = issues.iter().filter(|i| i.severity == ValidationSeverity::Error).count();
    let warning_count = issues.iter().filter(|i| i.severity == ValidationSeverity::Warning).count();

    ValidationReport { map_count: pack.maps.len(), error_count, warning_count, issues }
}

pub fn bootstrap_game_state_from_default_content() -> Result<(GameState, BootstrapDiagnostics)> {
    bootstrap_game_state_with_mode(GameMode::Classic)
}

pub fn bootstrap_game_state_with_mode(mode: GameMode) -> Result<(GameState, BootstrapDiagnostics)> {
    let pack = load_content_pack(content_pack_for_mode(mode))?;
    bootstrap_game_state_from_pack_with_mode(&pack, mode)
}

pub fn bootstrap_game_state_from_pack(
    pack: &ContentPack,
) -> Result<(GameState, BootstrapDiagnostics)> {
    bootstrap_game_state_from_pack_with_mode(pack, GameMode::Classic)
}

pub fn bootstrap_game_state_from_pack_with_mode(
    pack: &ContentPack,
    mode: GameMode,
) -> Result<(GameState, BootstrapDiagnostics)> {
    let city_map = pack
        .maps
        .iter()
        .find(|candidate| candidate.map_id == LEGACY_CITY_MAP_ID)
        .ok_or_else(|| anyhow!("content pack missing city map id {}", LEGACY_CITY_MAP_ID))?;
    let country_map =
        pack.maps.iter().find(|candidate| candidate.map_id == LEGACY_COUNTRY_MAP_ID).ok_or_else(
            || anyhow!("content pack missing country map id {}", LEGACY_COUNTRY_MAP_ID),
        )?;
    let city_level =
        city_map.levels.first().ok_or_else(|| anyhow!("map {} has no levels", city_map.map_id))?;
    let country_level = country_map
        .levels
        .first()
        .ok_or_else(|| anyhow!("map {} has no levels", country_map.map_id))?;

    let bounds = MapBounds { width: city_map.width as i32, height: city_map.height as i32 };
    let mut state = GameState::with_mode(mode, bounds);
    let catalogs = legacy_catalogs();
    let city_lookup = city_site_lookup(&catalogs);
    let item_name_pool = legacy_item_name_pool(&catalogs);
    state.encounter_monsters = catalogs.monsters.iter().map(|entry| entry.name.clone()).collect();
    state.world_mode = omega_core::WorldMode::DungeonCity;
    state.environment = LegacyEnvironment::City;
    state.city_map_id = city_map.map_id;
    state.city_level_index = city_level.level_index;
    state.city_map_source = city_map.source.clone();
    state.country_map_id = country_map.map_id;
    state.country_level_index = country_level.level_index;
    state.country_map_source = country_map.source.clone();
    state.map_binding = MapBinding {
        semantic: MapSemanticKind::City,
        map_id: state.city_map_id,
        level_index: state.city_level_index,
        source: state.city_map_source.clone(),
    };
    state.topology.city_site_id = 1;
    state.topology.country_region_id = 0;
    state.topology.dungeon_level = 0;

    let source_city_rows = city_level.rows.clone();
    let mut city_rows = source_city_rows.clone();
    let country_rows = &country_level.rows;

    state.city_site_grid = build_city_site_grid(&mut city_rows, &city_lookup);
    state.set_map_rows(city_rows.clone());
    state.city_map_rows = city_rows.clone();
    state.country_map_rows = country_rows.clone();
    state.country_site_grid = build_country_site_grid(country_rows);
    state.site_grid = state.city_site_grid.clone();
    state.country_grid =
        build_country_grid(country_rows, country_map.width as i32, country_map.height as i32);
    state.site_maps = build_site_map_definitions(pack);

    let country_rampart = find_country_rampart_position(country_rows).unwrap_or(Position {
        x: (country_map.width / 2) as i32,
        y: (country_map.height / 2) as i32,
    });
    state.topology.country_rampart_position = Some(country_rampart);

    let mut marker_spawn = None;
    let mut player_spawn_source = "legacy_rampart_start".to_string();

    let mut item_points = Vec::new();

    for (y, row) in source_city_rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let pos = Position { x: x as i32, y: y as i32 };
            match ch {
                '@' => {
                    if marker_spawn.is_none() {
                        marker_spawn = Some(pos);
                    }
                }
                '*' | '!' | '?' | '$' => {
                    item_points.push(pos);
                }
                _ => {}
            }
        }
    }

    let rampart_start = clamp_to_bounds(LEGACY_RAMPART_START, bounds);
    let player_pos = if is_walkable(&city_rows, rampart_start) {
        rampart_start
    } else if let Some(marker) = marker_spawn {
        player_spawn_source = "map_marker:@".to_string();
        clamp_to_bounds(marker, bounds)
    } else if let Some(fallback) = first_walkable(&city_rows) {
        player_spawn_source = "first_walkable".to_string();
        fallback
    } else {
        player_spawn_source = "default_center".to_string();
        Position { x: bounds.width / 2, y: bounds.height / 2 }
    }
    .to_owned();
    state.player.position = player_pos;

    for (idx, pos) in item_points.iter().enumerate() {
        let position = clamp_to_bounds(*pos, bounds);
        if position == state.player.position {
            continue;
        }
        let item_name =
            item_name_pool.get(idx % item_name_pool.len()).map(String::as_str).unwrap_or("item");
        state.place_item(item_name, position);
    }
    let _ = state.spawn_guard_monsters_from_markers();

    state.log.push(format!(
        "Loaded content map {} from {} ({}x{}, level {}).",
        city_map.map_id, city_map.source, city_map.width, city_map.height, city_level.level_index
    ));
    state.log.push(format!(
        "Loaded country map {} from {} ({}x{}, level {}).",
        country_map.map_id,
        country_map.source,
        country_map.width,
        country_map.height,
        country_level.level_index
    ));
    state.log.push("You pass through the massive gates of Rampart, the city.".to_string());
    state.log.push(
        "Rampart transit routes initialized with city/country semantic bindings.".to_string(),
    );

    let diagnostics = BootstrapDiagnostics {
        map_source: city_map.source.clone(),
        player_spawn_source,
        monster_spawns: state.monsters.len(),
        item_spawns: state.ground_items.len(),
    };

    Ok((state, diagnostics))
}

pub fn bootstrap_wizard_arena() -> Result<(GameState, BootstrapDiagnostics)> {
    let bounds = MapBounds { width: 50, height: 50 };
    let mut state = GameState::with_mode(GameMode::Modern, bounds);

    state.world_mode = omega_core::WorldMode::DungeonCity;
    state.environment = LegacyEnvironment::Arena;

    let mut rows = Vec::with_capacity(bounds.height as usize);
    for y in 0..bounds.height {
        let mut row = String::with_capacity(bounds.width as usize);
        for x in 0..bounds.width {
            let glyph = if x == 0
                || y == 0
                || x == bounds.width - 1
                || y == bounds.height - 1
                || (x + y) % 17 == 0
            {
                '#' // Perimeter or sparse walls
            } else if (x * y) % 13 == 0 {
                '~' // Water
            } else if (x + y) % 5 == 0 {
                '\"' // Grass
            } else {
                '.' // Stone/Floor
            };
            row.push(glyph);
        }
        rows.push(row);
    }

    state.set_map_rows(rows.clone());
    state.city_map_rows = rows.clone();

    // Initialize site grid with basic walkable/blocking flags
    let mut site_grid = Vec::with_capacity((bounds.width * bounds.height) as usize);
    for row in &rows {
        for glyph in row.chars() {
            let mut cell = TileSiteCell { glyph, ..TileSiteCell::default() };
            if glyph == '#' {
                cell.flags |= TILE_FLAG_BLOCK_MOVE;
            }
            site_grid.push(cell);
        }
    }
    state.site_grid = site_grid.clone();
    state.city_site_grid = site_grid;

    state.player.position = Position { x: 25, y: 25 };

    let diagnostics = BootstrapDiagnostics {
        map_source: "wizard_arena_generator".to_string(),
        player_spawn_source: "fixed_center".to_string(),
        monster_spawns: 0,
        item_spawns: 0,
    };

    Ok((state, diagnostics))
}

pub fn classic_content_fingerprint() -> Result<String> {
    let pack = load_content_pack(ContentPackId::Classic)?;
    let mut hash = 0xcbf29ce484222325u64;
    hash = fnv1a64_update(hash, pack.id.as_bytes());
    for map in &pack.maps {
        hash = fnv1a64_update(hash, map.source.as_bytes());
        hash = fnv1a64_update(hash, &map.map_id.to_le_bytes());
        hash = fnv1a64_update(hash, &map.width.to_le_bytes());
        hash = fnv1a64_update(hash, &map.height.to_le_bytes());
        for level in &map.levels {
            hash = fnv1a64_update(hash, &level.level_index.to_le_bytes());
            for row in &level.rows {
                hash = fnv1a64_update(hash, row.as_bytes());
            }
        }
    }
    Ok(format!("{hash:016x}"))
}

fn fnv1a64_update(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn first_walkable(rows: &[String]) -> Option<Position> {
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if is_city_walkable_glyph(ch) {
                return Some(Position { x: x as i32, y: y as i32 });
            }
        }
    }
    None
}

fn is_walkable(rows: &[String], pos: Position) -> bool {
    char_at(rows, pos).is_some_and(is_city_walkable_glyph)
}

fn is_city_walkable_glyph(glyph: char) -> bool {
    !matches!(glyph, '#' | '=' | '-' | 'D' | '7' | '"' | '*' | '~')
}

fn char_at(rows: &[String], pos: Position) -> Option<char> {
    let y = usize::try_from(pos.y).ok()?;
    let row = rows.get(y)?;
    let x = usize::try_from(pos.x).ok()?;
    row.chars().nth(x)
}

fn clamp_to_bounds(pos: Position, bounds: MapBounds) -> Position {
    Position {
        x: pos.x.clamp(0, bounds.width.saturating_sub(1)),
        y: pos.y.clamp(0, bounds.height.saturating_sub(1)),
    }
}

fn find_country_rampart_position(rows: &[String]) -> Option<Position> {
    for (y, row) in rows.iter().enumerate() {
        for (x, glyph) in row.chars().enumerate() {
            if glyph == 'O' {
                return Some(Position { x: x as i32, y: y as i32 });
            }
        }
    }
    None
}

fn parse_legacy_map(raw: &str, source_path: &Path) -> Result<LegacyMap> {
    let source = source_path.display().to_string();
    let mut lines = raw.lines().enumerate().peekable();

    let (line_no, version_line) = next_line(&mut lines, &source, "version header (v2)")?;
    let version = version_line
        .strip_prefix('v')
        .ok_or_else(|| anyhow!("{source}:{line_no}: expected version header like `v2`"))?
        .parse::<u16>()
        .with_context(|| format!("{source}:{line_no}: invalid version number"))?;
    if version != LEGACY_MAP_VERSION as u16 {
        bail!("{source}:{line_no}: unsupported version {version}, expected {}", LEGACY_MAP_VERSION);
    }

    let (line_no, map_line) = next_line(&mut lines, &source, "map header")?;
    let map_id = map_line
        .strip_prefix("map ")
        .ok_or_else(|| anyhow!("{source}:{line_no}: expected `map <id>` header"))?
        .parse::<u16>()
        .with_context(|| format!("{source}:{line_no}: invalid map id"))?;

    let (line_no, dim_line) = next_line(&mut lines, &source, "map dimension header")?;
    let mut parts = dim_line.split_whitespace();
    let depth = parts
        .next()
        .ok_or_else(|| anyhow!("{source}:{line_no}: missing depth value"))?
        .parse::<u16>()
        .with_context(|| format!("{source}:{line_no}: invalid depth"))?;

    let width_height =
        parts.next().ok_or_else(|| anyhow!("{source}:{line_no}: missing width,height section"))?;

    let (width_text, height_text) = width_height
        .split_once(',')
        .ok_or_else(|| anyhow!("{source}:{line_no}: expected `<depth> <width>,<height>`"))?;

    let width =
        width_text.parse::<u16>().with_context(|| format!("{source}:{line_no}: invalid width"))?;
    let height = height_text
        .parse::<u16>()
        .with_context(|| format!("{source}:{line_no}: invalid height"))?;

    if width == 0 || width as usize > MAX_MAP_DIMENSION {
        bail!("{source}:{line_no}: width {width} out of supported range 1..={}", MAX_MAP_DIMENSION);
    }
    if height == 0 || height as usize > MAX_MAP_DIMENSION {
        bail!(
            "{source}:{line_no}: height {height} out of supported range 1..={}",
            MAX_MAP_DIMENSION
        );
    }
    if depth == 0 || depth as usize > MAX_MAP_DIMENSION {
        bail!("{source}:{line_no}: depth {depth} out of supported range 1..={}", MAX_MAP_DIMENSION);
    }

    let mut levels = Vec::with_capacity(depth as usize);
    for level_index in 0..depth {
        let mut rows = Vec::with_capacity(height as usize);
        for _ in 0..height {
            let (line_no, row) = next_line(&mut lines, &source, "map row")?;
            if row.chars().count() != width as usize {
                bail!(
                    "{source}:{line_no}: map row width {} does not match expected width {width}",
                    row.chars().count()
                );
            }
            rows.push(row.to_string());
        }

        let (line_no, sep) = next_line(&mut lines, &source, "level separator (`==`)")?;
        if !sep.starts_with('=') {
            bail!(
                "{source}:{line_no}: expected level separator line starting with '=' after level {}",
                level_index + 1
            );
        }

        levels.push(MapLevel { level_index, rows });
    }

    Ok(LegacyMap { source, map_id, width, height, levels })
}

fn next_line<'a, I>(
    lines: &mut std::iter::Peekable<I>,
    source: &str,
    expected: &str,
) -> Result<(usize, &'a str)>
where
    I: Iterator<Item = (usize, &'a str)>,
{
    lines
        .next()
        .map(|(i, line)| (i + 1, line))
        .ok_or_else(|| anyhow!("{source}: unexpected EOF while reading {expected}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{
        Command, CountryTerrainKind, DeterministicRng, Event, MapSemanticKind, MonsterBehavior,
        SITE_AUX_ALTAR_ATHENA, SITE_AUX_ALTAR_DESTINY, SITE_AUX_ALTAR_HECATE, SITE_AUX_ALTAR_ODIN,
        SITE_AUX_ALTAR_SET, SITE_AUX_EXIT_ARENA, SITE_AUX_SERVICE_BANK, WorldMode, step,
    };

    #[test]
    fn parses_legacy_map_fixture() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("tools")
            .join("libsrc")
            .join("city.map");

        let map = load_legacy_map_file(path).expect("city.map should parse");
        assert_eq!(map.map_id, 3);
        assert_eq!(map.width, 64);
        assert_eq!(map.height, 64);
        assert_eq!(map.levels.len(), 1);
        assert_eq!(map.levels[0].rows.len(), 64);
    }

    #[test]
    fn validates_loaded_legacy_pack() {
        let pack = load_default_content().expect("default legacy content should load");
        let report = validate_content_pack(&pack);
        assert!(!report.has_errors(), "legacy fixtures should validate");
        assert_eq!(report.warning_count, 0);
    }

    #[test]
    fn rejects_wrong_row_width() {
        let raw = "v2\nmap 7\n1 3,2\nabc\nxy\n==\n";
        let err = parse_legacy_map(raw, Path::new("broken.map")).expect_err("must fail");
        let message = err.to_string();
        assert!(message.contains("width"), "error should mention width: {message}");
    }

    #[test]
    fn bootstrap_from_default_content_produces_playable_state() {
        let (state, diagnostics) =
            bootstrap_game_state_from_default_content().expect("default content should bootstrap");
        assert!(state.bounds.width > 0 && state.bounds.height > 0);
        assert!(state.bounds.contains(state.player.position));
        assert_eq!(state.player.position, LEGACY_RAMPART_START);
        assert_eq!(state.topology.city_site_id, 1);
        assert!(diagnostics.map_source.ends_with("city.map"));
        assert_eq!(diagnostics.player_spawn_source, "legacy_rampart_start");
        assert!(
            state
                .monsters
                .iter()
                .any(|monster| monster.name.to_ascii_lowercase().contains("guard")),
            "bootstrap should materialize city guard markers as interactive guards"
        );
        assert!(state.monsters.iter().all(|monster| monster.behavior == MonsterBehavior::Social));
        assert_eq!(state.map_rows.len(), state.bounds.height as usize);
        assert!(state.map_rows.iter().any(|row| row.contains('#')));
        assert!(state.log.iter().any(|line| line.contains("Rampart")));
    }

    #[test]
    fn legacy_catalog_cardinalities_match_defs_contract() {
        let catalogs = legacy_catalogs();
        assert_eq!(catalogs.spells.len(), 42);
        assert_eq!(catalogs.monsters.len(), 151);
        assert_eq!(catalogs.traps.len(), 13);
        assert_eq!(catalogs.city_sites.len(), 30);
        assert_eq!(catalogs.items.scrolls.len(), 24);
        assert_eq!(catalogs.items.potions.len(), 18);
        assert_eq!(catalogs.items.foods.len(), 16);
        assert_eq!(catalogs.items.weapons.len(), 41);
        assert_eq!(catalogs.items.armor.len(), 17);
        assert_eq!(catalogs.items.shields.len(), 8);
        assert_eq!(catalogs.items.cloaks.len(), 7);
        assert_eq!(catalogs.items.boots.len(), 7);
        assert_eq!(catalogs.items.rings.len(), 9);
        assert_eq!(catalogs.items.sticks.len(), 17);
        assert_eq!(catalogs.items.artifacts.len(), 26);
    }

    #[test]
    fn legacy_item_prototypes_capture_struct_fields_and_unique_rows() {
        let items = legacy_item_prototypes();
        assert_eq!(items.len(), 223, "expected TOTALITEMS entries from iinit.h");

        let victrix = items
            .iter()
            .find(|item| item.truename == "Victrix")
            .expect("Victrix should exist in parsed prototypes");
        assert_eq!(victrix.family, LegacyItemFamily::Weapon);
        assert_eq!(victrix.dmg, 100);
        assert_eq!(victrix.hit, 10);
        assert_eq!(victrix.usef, "I_VICTRIX");
        assert_eq!(victrix.item_type, "THRUSTING");
        assert_eq!(victrix.objchar, "WEAPON");
        assert_eq!(victrix.id_expr, "WEAPONID+35");

        let corpse = items
            .iter()
            .find(|item| item.family == LegacyItemFamily::Corpse)
            .expect("corpse prototype should be captured");
        assert_eq!(corpse.id_expr, "CORPSEID");
        assert_eq!(corpse.usef, "I_CORPSE");

        let cash = items
            .iter()
            .find(|item| item.family == LegacyItemFamily::Cash)
            .expect("cash prototype should be captured");
        assert_eq!(cash.id_expr, "CASHID");
        assert_eq!(cash.usef, "I_NO_OP");
    }

    #[test]
    fn bootstrap_binds_city_and_country_models() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        assert_eq!(state.map_binding.semantic, MapSemanticKind::City);
        assert_eq!(state.city_map_id, LEGACY_CITY_MAP_ID);
        assert_eq!(state.country_map_id, LEGACY_COUNTRY_MAP_ID);
        assert!(!state.city_site_grid.is_empty());
        assert!(!state.country_site_grid.is_empty());
        assert_eq!(state.site_grid.len(), state.city_site_grid.len());
        assert_eq!(state.country_grid.width, state.country_map_rows[0].chars().count() as i32);
        assert_eq!(state.country_grid.height, state.country_map_rows.len() as i32);
        assert!(state.topology.country_rampart_position.is_some());
    }

    #[test]
    fn startup_state_is_safe_for_first_turn() {
        let (mut state, _) =
            bootstrap_game_state_from_default_content().expect("bootstrap content");
        let hp_before = state.player.stats.hp;
        let mut rng = DeterministicRng::seeded(0xAA55_2277);
        let _ = step(&mut state, Command::Wait, &mut rng);
        assert_eq!(state.player.stats.hp, hp_before, "startup should not take immediate damage");
    }

    #[test]
    fn city_portcullis_and_nocitymove_flags_are_enforced() {
        let (mut state, _) =
            bootstrap_game_state_from_default_content().expect("bootstrap content");
        let port_idx = state
            .city_site_grid
            .iter()
            .position(|cell| (cell.flags & TILE_FLAG_PORTCULLIS) != 0)
            .expect("city map should include a portcullis tile");
        let width = usize::try_from(state.bounds.width).expect("positive width");
        let port_pos = Position {
            x: i32::try_from(port_idx % width).expect("x"),
            y: i32::try_from(port_idx / width).expect("y"),
        };
        assert!(!state.tile_is_walkable(port_pos), "portcullis tile must block movement");

        let nocity_idx = state
            .city_site_grid
            .iter()
            .position(|cell| (cell.flags & TILE_FLAG_NO_CITY_MOVE) != 0)
            .expect("city map should include NOCITYMOVE tile");
        let nocity_pos = Position {
            x: i32::try_from(nocity_idx % width).expect("x"),
            y: i32::try_from(nocity_idx / width).expect("y"),
        };
        state.player.position = nocity_pos;
        let mut rng = DeterministicRng::seeded(0x11);
        let out = step(&mut state, Command::Legacy { token: "M".to_string() }, &mut rng);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::LegacyHandled { token, note, fully_modeled } if token == "M" && *fully_modeled && note.contains("NOCITYMOVE")
        )));
    }

    #[test]
    fn country_and_city_transitions_switch_semantic_binding() {
        let (mut state, _) =
            bootstrap_game_state_from_default_content().expect("bootstrap content");
        let city_pos = state.player.position;
        let mut rng = DeterministicRng::seeded(0x7755);

        let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::Country);
        assert_eq!(state.environment, LegacyEnvironment::Countryside);
        assert!(!state.country_map_rows.is_empty());
        assert_eq!(state.map_rows, state.country_map_rows);

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::City);
        assert_eq!(state.environment, LegacyEnvironment::City);
        assert_ne!(state.map_rows, state.city_map_rows);
        assert!(!state.map_rows.join("").contains('G'));
        assert_eq!(state.player.position, city_pos);
    }

    #[test]
    fn country_grid_captures_site_ids_and_aux() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let mut city_sites = 0;
        let mut village_aux = std::collections::BTreeSet::new();
        let mut temple_aux = std::collections::BTreeSet::new();
        let mut special_sites = 0;

        for (cell, site) in state.country_grid.cells.iter().zip(state.country_site_grid.iter()) {
            match cell.base_terrain {
                CountryTerrainKind::City => {
                    city_sites += 1;
                    assert_eq!(site.site_id, COUNTRY_SITE_CITY);
                }
                CountryTerrainKind::Village => {
                    village_aux.insert(cell.aux);
                    assert_eq!(site.site_id, COUNTRY_SITE_VILLAGE);
                }
                CountryTerrainKind::Temple => {
                    temple_aux.insert(cell.aux);
                    assert_eq!(site.site_id, COUNTRY_SITE_TEMPLE);
                }
                CountryTerrainKind::Castle
                | CountryTerrainKind::Palace
                | CountryTerrainKind::Caves
                | CountryTerrainKind::Volcano
                | CountryTerrainKind::DragonLair
                | CountryTerrainKind::StarPeak
                | CountryTerrainKind::MagicIsle => {
                    special_sites += 1;
                    assert_ne!(site.site_id, COUNTRY_SITE_NONE);
                }
                _ => {}
            }
        }

        assert!(city_sites > 0, "country map should include at least one city");
        assert_eq!(village_aux.len(), 6, "expected six unique village aux values");
        assert_eq!(temple_aux.len(), 6, "expected six unique temple aux values");
        assert!(special_sites > 0, "expected special sites in country map");
    }

    #[test]
    fn entering_village_loads_distinct_village_map() {
        let (mut state, _) =
            bootstrap_game_state_from_default_content().expect("bootstrap content");
        let mut rng = DeterministicRng::seeded(0x9911);

        let _ = step(&mut state, Command::Legacy { token: "<".to_string() }, &mut rng);
        let village_index = state
            .country_grid
            .cells
            .iter()
            .position(|cell| cell.base_terrain == CountryTerrainKind::Village)
            .expect("country map should contain at least one village");
        let width = usize::try_from(state.country_grid.width).expect("positive width");
        state.player.position = Position {
            x: i32::try_from(village_index % width).expect("x"),
            y: i32::try_from(village_index / width).expect("y"),
        };

        let _ = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        assert_eq!(state.world_mode, WorldMode::DungeonCity);
        assert_eq!(state.environment, LegacyEnvironment::Village);
        assert_eq!(state.map_binding.semantic, MapSemanticKind::Site);
        assert_ne!(state.map_rows, state.city_map_rows, "village map must not mirror Rampart");
    }

    #[test]
    fn entering_bank_tile_applies_bank_service_not_random_roll() {
        let (mut state, _) =
            bootstrap_game_state_from_default_content().expect("bootstrap content");
        let mut rng = DeterministicRng::seeded(0x5522);
        let bank_idx = state
            .city_site_grid
            .iter()
            .position(|cell| cell.aux == SITE_AUX_SERVICE_BANK)
            .expect("city should include bank service tile");
        let width = usize::try_from(state.bounds.width).expect("positive width");
        state.player.position = Position {
            x: i32::try_from(bank_idx % width).expect("x"),
            y: i32::try_from(bank_idx / width).expect("y"),
        };
        let gold_before = state.gold;

        let out = step(&mut state, Command::Legacy { token: ">".to_string() }, &mut rng);
        assert!(out.events.iter().any(|event| matches!(
            event,
            Event::EconomyUpdated { source, .. } if source == "bank"
        )));
        assert!(state.gold <= gold_before);
    }

    #[test]
    fn city_altars_map_to_all_deity_aux_markers() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let aux: std::collections::BTreeSet<i32> =
            state.city_site_grid.iter().map(|cell| cell.aux).collect();
        assert!(aux.contains(&SITE_AUX_ALTAR_ODIN));
        assert!(aux.contains(&SITE_AUX_ALTAR_SET));
        assert!(aux.contains(&SITE_AUX_ALTAR_ATHENA));
        assert!(aux.contains(&SITE_AUX_ALTAR_HECATE));
        assert!(aux.contains(&SITE_AUX_ALTAR_DESTINY));

        for deity_aux in [
            SITE_AUX_ALTAR_ODIN,
            SITE_AUX_ALTAR_SET,
            SITE_AUX_ALTAR_ATHENA,
            SITE_AUX_ALTAR_HECATE,
            SITE_AUX_ALTAR_DESTINY,
        ] {
            let idx = state
                .city_site_grid
                .iter()
                .position(|cell| cell.aux == deity_aux)
                .expect("altar aux tile must exist");
            let width = usize::try_from(state.bounds.width).expect("city width");
            let x = i32::try_from(idx % width).expect("x");
            let y = i32::try_from(idx / width).expect("y");
            assert_eq!(state.map_glyph_at(Position { x, y }), '_');
        }
    }

    #[test]
    fn city_generic_doors_are_materialized_and_placeholder_markers_erased() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let flattened = state.city_map_rows.join("");
        assert!(
            !flattened
                .chars()
                .any(|glyph| matches!(glyph, 'p' | '!' | 'I' | 'E' | 'e' | 'x' | 'K')),
            "runtime city map should not retain generic placeholder markers"
        );
        assert!(flattened.contains('/'), "city should include open door tiles");
        assert!(flattened.contains('-'), "city should include closed door tiles");
    }

    #[test]
    fn city_generic_assignments_cover_legacy_service_set() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let lookup = city_site_lookup(&legacy_catalogs());
        let required = [
            "armorer",
            "club",
            "gym",
            "thieves guild",
            "healer",
            "casino",
            "diner",
            "crap",
            "commandant",
            "tavern",
            "alchemist",
            "dpw",
            "library",
            "pawn shop",
            "condo",
            "brothel",
            "monastery",
        ];

        for name in required {
            let id = site_id(&lookup, name);
            assert_ne!(id, 0, "expected city-site catalog entry for `{name}`");
            assert!(
                state.city_site_grid.iter().any(|cell| cell.site_id == id),
                "expected runtime city assignment for `{name}`"
            );
        }
    }

    #[test]
    fn rampart_unique_services_are_not_collapsed_to_generic_aux() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let lookup = city_site_lookup(&legacy_catalogs());
        let expected = [
            ("armorer", SITE_AUX_SERVICE_ARMORER),
            ("club", SITE_AUX_SERVICE_CLUB),
            ("gym", SITE_AUX_SERVICE_GYM),
            ("healer", SITE_AUX_SERVICE_HEALER),
            ("casino", SITE_AUX_SERVICE_CASINO),
            ("commandant", SITE_AUX_SERVICE_COMMANDANT),
            ("diner", SITE_AUX_SERVICE_DINER),
            ("crap", SITE_AUX_SERVICE_CRAPS),
            ("tavern", SITE_AUX_SERVICE_TAVERN),
            ("pawn shop", SITE_AUX_SERVICE_PAWN_SHOP),
            ("brothel", SITE_AUX_SERVICE_BROTHEL),
            ("condo", SITE_AUX_SERVICE_CONDO),
        ];

        for (name, aux) in expected {
            let id = site_id(&lookup, name);
            assert!(
                state.city_site_grid.iter().any(|cell| cell.site_id == id && cell.aux == aux),
                "expected `{name}` to map to aux {aux} in runtime city assignments"
            );
        }
    }

    #[test]
    fn arena_site_map_uses_arena_exit_and_portcullis_semantics() {
        let (state, _) = bootstrap_game_state_from_default_content().expect("bootstrap content");
        let arena_map = state
            .site_maps
            .iter()
            .find(|candidate| candidate.map_id == 1)
            .expect("arena site map should be loaded from content");
        assert_eq!(arena_map.environment, LegacyEnvironment::Arena);
        assert!(arena_map.site_grid.iter().any(|cell| cell.aux == SITE_AUX_EXIT_ARENA));
        assert!(arena_map.site_grid.iter().any(|cell| {
            (cell.flags & TILE_FLAG_PORTCULLIS) != 0 && (cell.flags & TILE_FLAG_BLOCK_MOVE) != 0
        }));
    }
}
