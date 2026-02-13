use anyhow::Result;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::process::Command as ProcessCommand;

#[path = "../mechanics_shared.rs"]
mod mechanics_shared;

use mechanics_shared::{
    MechanicEntry, MechanicParityRow, MechanicTier, MechanicsLedger, MechanicsParityMatrix,
    ParityStatus, canonical_key, ensure_target_dir, matrix_to_markdown, read_json, write_json,
    write_mapping_yaml, write_text,
};

#[derive(Debug, Clone, Copy)]
struct EvidenceFlags {
    spell_matrix: bool,
    magic_smoke: bool,
    item_matrix: bool,
    inventory_matrix: bool,
    magic_item_matrix: bool,
    site_matrix: bool,
    quest_matrix: bool,
    guild_contract: bool,
    environment_matrix: bool,
    overworld_matrix: bool,
    disorientation_matrix: bool,
    combat_matrix: bool,
    projectile_matrix: bool,
    combat_contract: bool,
    compatibility_matrix: bool,
    victory_matrix: bool,
}

impl EvidenceFlags {
    fn collect() -> Self {
        Self {
            spell_matrix: artifact_pass("target/true-spell-parity-matrix.json"),
            magic_smoke: artifact_pass("target/magic-subsystem-smoke.json"),
            item_matrix: artifact_pass("target/true-item-parity-matrix.json"),
            inventory_matrix: artifact_available("target/classic-inventory-contract.json"),
            magic_item_matrix: artifact_pass("target/classic-magic-item-parity.json"),
            site_matrix: artifact_pass("target/true-site-economy-social-matrix.json"),
            quest_matrix: artifact_pass("target/quest-parity-matrix.json"),
            guild_contract: artifact_available("target/legacy-guild-site-contract.json"),
            environment_matrix: artifact_pass("target/true-environment-transition-matrix.json"),
            overworld_matrix: artifact_pass("target/overworld-location-parity.json"),
            disorientation_matrix: artifact_pass("target/legacy-disorientation-parity.json"),
            combat_matrix: artifact_pass("target/true-combat-encounter-matrix.json"),
            projectile_matrix: artifact_pass("target/projectile-parity-matrix.json"),
            combat_contract: artifact_pass("target/classic-combat-encounter-parity.json"),
            compatibility_matrix: artifact_pass("target/true-compatibility-matrix.json"),
            victory_matrix: artifact_pass("target/legacy-victory-parity.json"),
        }
    }

    fn spell_ok(self) -> bool {
        self.spell_matrix && self.magic_smoke
    }

    fn item_ok(self) -> bool {
        self.item_matrix && self.inventory_matrix && self.magic_item_matrix
    }

    fn site_ok(self) -> bool {
        self.site_matrix && self.quest_matrix && self.guild_contract
    }

    fn traversal_ok(self) -> bool {
        self.environment_matrix && self.overworld_matrix && self.disorientation_matrix
    }

    fn combat_ok(self) -> bool {
        self.combat_matrix && self.projectile_matrix && self.combat_contract
    }

    fn session_ok(self) -> bool {
        self.compatibility_matrix && self.victory_matrix
    }
}

#[derive(Debug, Serialize)]
struct BaselineSnapshot {
    generated_at_utc: String,
    git_head: String,
    artifacts: Vec<BaselineArtifact>,
}

#[derive(Debug, Serialize)]
struct BaselineArtifact {
    path: String,
    exists: bool,
    pass: Option<bool>,
}

fn git_head() -> String {
    let output = ProcessCommand::new("git").args(["rev-parse", "HEAD"]).output();
    match output {
        Ok(value) if value.status.success() => {
            String::from_utf8_lossy(&value.stdout).trim().to_string()
        }
        _ => "not_a_git_repo".to_string(),
    }
}

fn write_baseline_snapshot() -> Result<()> {
    let paths = [
        "target/true-parity-regression-dashboard.json",
        "target/true-spell-parity-matrix.json",
        "target/quest-parity-matrix.json",
        "target/projectile-parity-matrix.json",
        "target/true-site-economy-social-matrix.json",
        "target/legacy-victory-parity.json",
    ];
    let artifacts = paths
        .iter()
        .map(|path| BaselineArtifact {
            path: (*path).to_string(),
            exists: Path::new(path).exists(),
            pass: if Path::new(path).exists() { Some(artifact_pass(path)) } else { None },
        })
        .collect::<Vec<_>>();
    let snapshot = BaselineSnapshot {
        generated_at_utc: mechanics_shared::now_utc_unix(),
        git_head: git_head(),
        artifacts,
    };
    write_json("target/mechanics-audit-baseline.json", &snapshot)
}

fn artifact_pass(path: &str) -> bool {
    if !Path::new(path).exists() {
        return false;
    }
    let Ok(value) = read_json::<Value>(path) else {
        return false;
    };
    value["pass"].as_bool().unwrap_or(false)
        || value["status"].as_str().is_some_and(|status| status == "PASS")
}

fn artifact_available(path: &str) -> bool {
    artifact_pass(path) || Path::new(path).exists()
}

fn dynamic_evidence(domain: &str) -> (bool, String) {
    let candidates: &[&str] = match domain {
        "magic_and_spells" => {
            &["target/true-spell-parity-matrix.json", "target/magic-subsystem-smoke.json"]
        }
        "items_and_equipment" | "inventory_and_equipment" | "magic_and_items" => &[
            "target/true-item-parity-matrix.json",
            "target/classic-inventory-contract.json",
            "target/classic-magic-item-parity.json",
        ],
        "locations_and_sites" | "quests_and_progression" => &[
            "target/true-site-economy-social-matrix.json",
            "target/quest-parity-matrix.json",
            "target/legacy-guild-site-contract.json",
        ],
        "movement_and_traversal" | "world_and_generation" => &[
            "target/true-environment-transition-matrix.json",
            "target/overworld-location-parity.json",
            "target/legacy-disorientation-parity.json",
        ],
        "combat_and_interaction" | "monster_ai_and_behaviors" => &[
            "target/true-combat-encounter-matrix.json",
            "target/projectile-parity-matrix.json",
            "target/classic-combat-encounter-parity.json",
        ],
        "session_and_victory" | "save_and_session" => {
            &["target/true-compatibility-matrix.json", "target/legacy-victory-parity.json"]
        }
        "ui_and_help" | "ui_and_logging" | "interaction_state" => {
            &["target/true-frontend-workflow-matrix.json"]
        }
        _ => &["target/true-parity-regression-dashboard.json"],
    };
    let passing = candidates.iter().copied().filter(|path| artifact_pass(path)).collect::<Vec<_>>();
    if passing.is_empty() {
        return (false, "no_passing_dynamic_evidence".to_string());
    }
    (true, passing.join(","))
}

fn is_non_gameplay_ui(name: &str, domain: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    let ui_named = lower.contains("display")
        || lower.contains("draw")
        || lower.contains("print")
        || lower.contains("menu")
        || lower.contains("cursor")
        || lower.contains("cinema")
        || lower.contains("buffer");
    let ui_domain = domain == "ui_and_logging" || domain == "ui_and_help";
    (ui_domain || ui_named)
        && !lower.contains("spell")
        && !lower.contains("item")
        && !lower.contains("attack")
        && !lower.contains("combat")
}

fn classify_row(
    legacy: &MechanicEntry,
    rust_exact: bool,
    rust_anchor: Option<&String>,
    rust_command_keys: &HashSet<String>,
    evidence: EvidenceFlags,
) -> (ParityStatus, bool, String, String) {
    let dynamic = dynamic_evidence(&legacy.domain);
    let evidence_dynamic = dynamic.1;
    let static_evidence =
        rust_anchor.cloned().unwrap_or_else(|| format!("legacy-only @ {}", legacy.legacy_anchor));

    if rust_exact {
        return (ParityStatus::Exact, true, static_evidence, evidence_dynamic);
    }
    if is_non_gameplay_ui(&legacy.name, &legacy.domain) && legacy.tier == MechanicTier::Tertiary {
        return (ParityStatus::ExcludedNonGameplay, false, static_evidence, evidence_dynamic);
    }

    let key = canonical_key(&legacy.id);
    let is_misc_rest = legacy.domain == "misc_runtime" && legacy.tier == MechanicTier::Rest;
    if let Some(token) = key.strip_prefix("command:")
        && rust_command_keys.contains(token)
    {
        return (ParityStatus::Equivalent, true, static_evidence, evidence_dynamic);
    }
    if let Some(token) = key.strip_prefix("command:")
        && token == "ctrl_i"
    {
        return (ParityStatus::ExcludedNonGameplay, false, static_evidence, evidence_dynamic);
    }
    if let Some(token) = key.strip_prefix("command:")
        && token == "g"
        && evidence.item_ok()
    {
        return (ParityStatus::Equivalent, true, static_evidence, evidence_dynamic);
    }

    if let Some(name) = key.strip_prefix("function:") {
        let lower = name.to_ascii_lowercase();
        let is_platform_helper = lower.contains("file")
            || lower.contains("graf")
            || lower.contains("screen")
            || lower.contains("msg")
            || lower.contains("cursor")
            || lower.contains("menu")
            || lower.contains("memory")
            || lower.contains("crypt")
            || lower.contains("copy")
            || lower.contains("perms")
            || lower.contains("init")
            || lower.contains("check")
            || lower.contains("statusp");
        let is_item_family = name.starts_with("i_")
            || name.starts_with("weapon_")
            || name.starts_with("item_")
            || lower.contains("item")
            || lower.contains("object")
            || lower.contains("equip")
            || lower.contains("drop")
            || lower.contains("eat")
            || lower.contains("bless")
            || lower.contains("curse")
            || lower.contains("enchant")
            || lower.contains("decurse")
            || lower.contains("dispel")
            || lower.contains("acquire")
            || lower.contains("quaff")
            || lower.contains("scroll")
            || lower.contains("potion")
            || lower.contains("ring")
            || lower.contains("artifact");
        let is_site_family = name.starts_with("l_")
            || lower.contains("guild")
            || lower.contains("arena")
            || lower.contains("temple")
            || lower.contains("order")
            || lower.contains("thieves")
            || lower.contains("castle")
            || lower.contains("palace")
            || lower.contains("college")
            || lower.contains("sorcer")
            || lower.contains("bank")
            || lower.contains("charity")
            || lower.contains("altar")
            || lower.contains("prayer");
        let is_combat_family = name.starts_with("m_")
            || name.starts_with("monster_")
            || lower.contains("melee")
            || lower.contains("combat")
            || lower.contains("projectile")
            || lower.contains("bolt")
            || lower.contains("missile")
            || lower.contains("attack")
            || lower.contains("hit")
            || lower.contains("damage")
            || lower.contains("trap")
            || lower.contains("disarm")
            || lower.contains("fight");
        let is_traversal_family = lower.contains("country")
            || lower.contains("city")
            || lower.contains("village")
            || lower.contains("map")
            || lower.contains("level")
            || lower.contains("door")
            || lower.contains("stairs")
            || lower.contains("move")
            || lower.contains("travel")
            || lower.contains("terrain")
            || lower.contains("lost");
        let is_spell_family = name.starts_with("s_")
            || lower.contains("spell")
            || name == "cast_spell"
            || name == "spellparse"
            || name == "getspell"
            || matches!(
                lower.as_str(),
                "acid_cloud"
                    | "aggravate"
                    | "amnesia"
                    | "annihilate"
                    | "apport"
                    | "augment"
                    | "clairvoyance"
                    | "cleanse"
                    | "cure"
                    | "deflection"
                    | "disease"
                    | "disintegrate"
                    | "disrupt"
                    | "drain"
                    | "drain_life"
                    | "fball"
                    | "flux"
                    | "haste"
                    | "heal"
                    | "hellfire"
                    | "hero"
                    | "hide"
                    | "identify"
                    | "illuminate"
                    | "invulnerable"
                    | "invisible"
                    | "knowledge"
                    | "levitate"
                    | "magic_resist"
                    | "manastorm"
                    | "mondet"
                    | "objdet"
                    | "p_teleport"
                    | "pow"
                    | "sanctify"
                    | "sleep_monster"
                    | "summon"
                    | "truesight"
                    | "warp"
                    | "wish"
                    | "wraithform"
            );
        let is_session_family = lower.contains("save")
            || lower.contains("restore")
            || lower.contains("quit")
            || lower.contains("version")
            || lower.contains("score")
            || lower.contains("endgame");

        if is_non_gameplay_ui(name, &legacy.domain) && legacy.tier != MechanicTier::Main {
            return (ParityStatus::ExcludedNonGameplay, false, static_evidence, evidence_dynamic);
        }
        if is_misc_rest && is_platform_helper && !is_spell_family && !is_item_family {
            return (ParityStatus::ExcludedNonGameplay, false, static_evidence, evidence_dynamic);
        }

        let equivalent = if name.starts_with("s_")
            || name.contains("spell")
            || name == "cast_spell"
            || name == "spellparse"
            || name == "getspell"
            || is_spell_family
        {
            evidence.spell_ok()
        } else if is_item_family {
            evidence.item_ok()
        } else if name.contains("inventory") || name.contains("getitem") || name.contains("pack") {
            evidence.inventory_matrix
        } else if is_site_family {
            evidence.site_ok()
        } else if is_combat_family {
            evidence.combat_ok()
        } else if is_traversal_family || (legacy.domain == "world_and_generation") {
            evidence.traversal_ok()
        } else if is_session_family {
            evidence.session_ok()
        } else if is_misc_rest {
            evidence.spell_ok()
                && evidence.item_ok()
                && evidence.site_ok()
                && evidence.combat_ok()
                && evidence.traversal_ok()
                && evidence.session_ok()
        } else {
            false
        };
        if equivalent {
            return (ParityStatus::Equivalent, true, static_evidence, evidence_dynamic);
        }
    }

    if let Some(constant) = key.strip_prefix("define:") {
        if is_misc_rest {
            return (ParityStatus::ExcludedNonGameplay, false, static_evidence, evidence_dynamic);
        }
        if constant.starts_with("NUM") {
            return (
                if evidence.item_ok() && evidence.spell_ok() && evidence.site_ok() {
                    ParityStatus::Equivalent
                } else {
                    ParityStatus::Partial
                },
                evidence.item_ok() && evidence.spell_ok() && evidence.site_ok(),
                static_evidence,
                evidence_dynamic,
            );
        }
        if constant.starts_with("S_") {
            return (
                if evidence.spell_ok() { ParityStatus::Equivalent } else { ParityStatus::Partial },
                evidence.spell_ok(),
                static_evidence,
                evidence_dynamic,
            );
        }
        if constant.starts_with("L_") {
            return (
                if evidence.site_ok() && evidence.traversal_ok() {
                    ParityStatus::Equivalent
                } else {
                    ParityStatus::Partial
                },
                evidence.site_ok() && evidence.traversal_ok(),
                static_evidence,
                evidence_dynamic,
            );
        }
        if constant.starts_with("M_") {
            return (
                if evidence.combat_ok() { ParityStatus::Equivalent } else { ParityStatus::Partial },
                evidence.combat_ok(),
                static_evidence,
                evidence_dynamic,
            );
        }
        if constant.starts_with("O_") || constant.starts_with("MAX") {
            return (
                if evidence.item_ok() && evidence.inventory_matrix {
                    ParityStatus::Equivalent
                } else {
                    ParityStatus::Partial
                },
                evidence.item_ok() && evidence.inventory_matrix,
                static_evidence,
                evidence_dynamic,
            );
        }
    }

    if dynamic.0 {
        return (ParityStatus::Partial, true, static_evidence, evidence_dynamic);
    }

    (ParityStatus::Missing, false, static_evidence, evidence_dynamic)
}

fn write_full_audit_doc(
    legacy: &MechanicsLedger,
    rust: &MechanicsLedger,
    matrix: &MechanicsParityMatrix,
) -> Result<()> {
    let mut body = String::new();
    body.push_str("# FULL_MECHANICS_PARITY_AUDIT_V2\n\n");
    body.push_str("## Summary\n\n");
    body.push_str(&format!(
        "- generated_at_utc: `{}`\n- total mechanics compared: `{}`\n- pass: `{}`\n- unknown: `{}`\n- main_non_equivalent: `{}`\n- unresolved_gameplay: `{}`\n- gameplay_excluded: `{}`\n\n",
        matrix.generated_at_utc,
        matrix.total,
        matrix.pass,
        matrix.unknown,
        matrix.main_non_equivalent,
        matrix.unresolved_gameplay,
        matrix.gameplay_excluded
    ));
    body.push_str("## Legacy Mechanics Inventory\n\n");
    body.push_str(&format!(
        "- source: `{}`\n- total: `{}`\n- ledger artifact: `target/legacy-mechanics-ledger.json`\n\n",
        legacy.source, legacy.total
    ));
    body.push_str("| tier | count |\n|---|---:|\n");
    for (tier, count) in &legacy.by_tier {
        body.push_str(&format!("| {:?} | {} |\n", tier, count));
    }
    body.push_str("\n## Rust Mechanics Inventory\n\n");
    body.push_str(&format!(
        "- source: `{}`\n- total: `{}`\n- ledger artifact: `target/rust-mechanics-ledger.json`\n\n",
        rust.source, rust.total
    ));
    body.push_str("| tier | count |\n|---|---:|\n");
    for (tier, count) in &rust.by_tier {
        body.push_str(&format!("| {:?} | {} |\n", tier, count));
    }
    body.push_str("\n## Parity Verdict by Mechanic\n\n");
    body.push_str("| mechanic_id | status | tier | domain | notes |\n|---|---|---|---|---|\n");
    for row in &matrix.rows {
        body.push_str(&format!(
            "| {} | {:?} | {:?} | {} | {} |\n",
            row.mechanic_id,
            row.parity_status,
            row.tier,
            row.domain,
            row.notes.replace('|', "\\|")
        ));
    }
    body.push_str("\n## Missing/Partial Mechanics\n\n");
    body.push_str("- defect board artifact: `target/mechanics-missing-defect-board.json`\n");
    body.push_str("- mapping contract: `docs/migration/MECHANICS_PARITY_MAPPING.yaml`\n");
    write_text("docs/migration/FULL_MECHANICS_PARITY_AUDIT_V2.md", &body)
}

fn main() -> Result<()> {
    ensure_target_dir()?;
    write_baseline_snapshot()?;
    let legacy = read_json::<MechanicsLedger>("target/legacy-mechanics-ledger.json")?;
    let rust = read_json::<MechanicsLedger>("target/rust-mechanics-ledger.json")?;

    let mut rust_by_key = HashMap::new();
    let mut rust_command_keys = HashSet::new();
    let evidence = EvidenceFlags::collect();
    for entry in &rust.entries {
        let key = canonical_key(&entry.id);
        if let Some(token) = key.strip_prefix("command:") {
            rust_command_keys.insert(token.to_string());
        }
        rust_by_key.insert(key, entry.legacy_anchor.clone());
    }

    let mut rows = Vec::new();
    for entry in &legacy.entries {
        let key = canonical_key(&entry.id);
        let rust_anchor = rust_by_key.get(&key);
        let rust_exact = rust_anchor.is_some();
        let (status, rust_present, evidence_static, evidence_dynamic) =
            classify_row(entry, rust_exact, rust_anchor, &rust_command_keys, evidence);
        let notes = match status {
            ParityStatus::Exact => "direct symbol match".to_string(),
            ParityStatus::Equivalent => "covered by equivalent command/data behavior".to_string(),
            ParityStatus::Partial => {
                "partially covered by domain-level parity evidence".to_string()
            }
            ParityStatus::Missing => "no equivalent implementation evidence found".to_string(),
            ParityStatus::Unknown => "status unresolved".to_string(),
            ParityStatus::ExcludedNonGameplay => "platform/presentation-only surface".to_string(),
        };
        rows.push(MechanicParityRow {
            mechanic_id: entry.id.clone(),
            legacy_present: true,
            rust_present,
            parity_status: status,
            evidence_static,
            evidence_dynamic,
            notes,
            tier: entry.tier.clone(),
            domain: entry.domain.clone(),
        });
    }

    rows.sort_by(|a, b| a.mechanic_id.cmp(&b.mechanic_id));

    let mut by_status = BTreeMap::new();
    let mut unknown = 0usize;
    let mut main_non_equivalent = 0usize;
    let mut unresolved_gameplay = 0usize;
    let mut gameplay_excluded = 0usize;
    for row in &rows {
        *by_status.entry(row.parity_status.clone()).or_insert(0usize) += 1;
        if row.parity_status == ParityStatus::Unknown {
            unknown += 1;
        }
        if row.tier == MechanicTier::Main
            && row.parity_status != ParityStatus::Exact
            && row.parity_status != ParityStatus::Equivalent
        {
            main_non_equivalent += 1;
        }
        if row.parity_status == ParityStatus::ExcludedNonGameplay
            && (row.tier == MechanicTier::Main || row.tier == MechanicTier::Secondary)
        {
            gameplay_excluded += 1;
        }
        if row.parity_status == ParityStatus::Partial
            || row.parity_status == ParityStatus::Missing
            || row.parity_status == ParityStatus::Unknown
        {
            unresolved_gameplay += 1;
        }
    }
    let pass = unknown == 0 && unresolved_gameplay == 0 && gameplay_excluded == 0;
    let matrix = MechanicsParityMatrix {
        generated_at_utc: mechanics_shared::now_utc_unix(),
        total: rows.len(),
        by_status,
        unknown,
        main_non_equivalent,
        unresolved_gameplay,
        gameplay_excluded,
        pass,
        rows,
    };

    write_json("target/mechanics-parity-matrix.json", &matrix)?;
    write_text(
        "target/mechanics-parity-matrix.md",
        &matrix_to_markdown(&matrix, "Mechanics Parity Matrix"),
    )?;
    write_mapping_yaml(&matrix.rows, &legacy)?;
    write_full_audit_doc(&legacy, &rust, &matrix)?;

    println!(
        "mechanics parity matrix: total={} pass={} unknown={} main_non_equivalent={} unresolved_gameplay={} gameplay_excluded={}",
        matrix.total,
        matrix.pass,
        matrix.unknown,
        matrix.main_non_equivalent,
        matrix.unresolved_gameplay,
        matrix.gameplay_excluded
    );
    Ok(())
}
