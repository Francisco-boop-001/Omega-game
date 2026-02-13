use anyhow::{Context, Result, bail};
use omega_content::bootstrap_game_state_from_default_content;
use omega_core::{
    Command, DeterministicRng, Event, SITE_AUX_SERVICE_ARENA, SITE_AUX_SERVICE_ARMORER,
    SITE_AUX_SERVICE_BANK, SITE_AUX_SERVICE_BROTHEL, SITE_AUX_SERVICE_CASINO,
    SITE_AUX_SERVICE_CASTLE, SITE_AUX_SERVICE_CHARITY, SITE_AUX_SERVICE_CLUB,
    SITE_AUX_SERVICE_COLLEGE, SITE_AUX_SERVICE_COMMANDANT, SITE_AUX_SERVICE_CONDO,
    SITE_AUX_SERVICE_CRAPS, SITE_AUX_SERVICE_DINER, SITE_AUX_SERVICE_GYM, SITE_AUX_SERVICE_HEALER,
    SITE_AUX_SERVICE_MERC_GUILD, SITE_AUX_SERVICE_MONASTERY, SITE_AUX_SERVICE_ORDER,
    SITE_AUX_SERVICE_PALACE, SITE_AUX_SERVICE_PAWN_SHOP, SITE_AUX_SERVICE_SHOP,
    SITE_AUX_SERVICE_SORCERORS, SITE_AUX_SERVICE_TAVERN, SITE_AUX_SERVICE_TEMPLE,
    SITE_AUX_SERVICE_THIEVES, step,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TalkClarityCheck {
    id: String,
    aux: i32,
    passed: bool,
    details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TalkClarityReport {
    total: usize,
    passed: usize,
    failed: usize,
    pass: bool,
    checks: Vec<TalkClarityCheck>,
}

fn markdown(report: &TalkClarityReport) -> String {
    let mut out = Vec::new();
    out.push("# Guild Service Talk Clarity".to_string());
    out.push(String::new());
    out.push(format!("- total: `{}`", report.total));
    out.push(format!("- passed: `{}`", report.passed));
    out.push(format!("- failed: `{}`", report.failed));
    out.push(format!("- status: `{}`", if report.pass { "PASS" } else { "FAIL" }));
    out.push(String::new());
    out.push("| Check | Aux | Status | Details |".to_string());
    out.push("|---|---:|---|---|".to_string());
    for check in &report.checks {
        out.push(format!(
            "| {} | {} | {} | {} |",
            check.id,
            check.aux,
            if check.passed { "PASS" } else { "FAIL" },
            check.details.replace('|', "\\|")
        ));
    }
    out.push(String::new());
    out.join("\n")
}

fn run_talk_check(id: &str, aux: i32, seed: u64) -> Result<TalkClarityCheck> {
    let (mut state, _) = bootstrap_game_state_from_default_content().context("bootstrap state")?;
    let mut rng = DeterministicRng::seeded(seed);
    state.options.interactive_sites = true;
    state.gold = 900;
    state.progression.deity_favor = 8;

    let site = state
        .tile_site_at_mut(state.player.position)
        .with_context(|| format!("missing site cell at player position for {id}"))?;
    site.aux = aux;

    let outcome = step(&mut state, Command::Legacy { token: "t".to_string() }, &mut rng);
    let talk_note = outcome.events.iter().rev().find_map(|event| {
        if let Event::LegacyHandled { token, note, .. } = event
            && token == "t"
        {
            return Some(note.clone());
        }
        None
    });
    let line = talk_note.unwrap_or_else(|| state.log.last().cloned().unwrap_or_default());
    let lower = line.to_ascii_lowercase();
    let generic_noise = [
        "audience held",
        "dialogue resolved with",
        "quest hooks processed",
        "you exchange a few words with",
        "points you toward service and duty",
    ];
    let no_placeholder = !generic_noise.iter().any(|needle| lower.contains(needle));
    let legacy_handled = outcome.events.iter().any(|event| {
        matches!(
            event,
            Event::LegacyHandled { token, .. } if token == "t"
        )
    });
    let passed = legacy_handled && no_placeholder && !line.trim().is_empty();
    let details = format!(
        "legacy_handled={} no_placeholder={} note={}",
        legacy_handled, no_placeholder, line
    );

    Ok(TalkClarityCheck { id: id.to_string(), aux, passed, details })
}

fn main() -> Result<()> {
    let services = [
        ("shop", SITE_AUX_SERVICE_SHOP),
        ("bank", SITE_AUX_SERVICE_BANK),
        ("merc_guild", SITE_AUX_SERVICE_MERC_GUILD),
        ("temple", SITE_AUX_SERVICE_TEMPLE),
        ("college", SITE_AUX_SERVICE_COLLEGE),
        ("sorcerors", SITE_AUX_SERVICE_SORCERORS),
        ("castle", SITE_AUX_SERVICE_CASTLE),
        ("order", SITE_AUX_SERVICE_ORDER),
        ("charity", SITE_AUX_SERVICE_CHARITY),
        ("arena", SITE_AUX_SERVICE_ARENA),
        ("thieves", SITE_AUX_SERVICE_THIEVES),
        ("palace", SITE_AUX_SERVICE_PALACE),
        ("monastery", SITE_AUX_SERVICE_MONASTERY),
        ("armorer", SITE_AUX_SERVICE_ARMORER),
        ("club", SITE_AUX_SERVICE_CLUB),
        ("gym", SITE_AUX_SERVICE_GYM),
        ("healer", SITE_AUX_SERVICE_HEALER),
        ("casino", SITE_AUX_SERVICE_CASINO),
        ("commandant", SITE_AUX_SERVICE_COMMANDANT),
        ("diner", SITE_AUX_SERVICE_DINER),
        ("craps", SITE_AUX_SERVICE_CRAPS),
        ("tavern", SITE_AUX_SERVICE_TAVERN),
        ("pawn_shop", SITE_AUX_SERVICE_PAWN_SHOP),
        ("brothel", SITE_AUX_SERVICE_BROTHEL),
        ("condo", SITE_AUX_SERVICE_CONDO),
    ];

    let mut checks = Vec::with_capacity(services.len());
    for (idx, (id, aux)) in services.into_iter().enumerate() {
        checks.push(run_talk_check(id, aux, 0xC0DE_5000 + idx as u64)?);
    }

    let total = checks.len();
    let passed = checks.iter().filter(|check| check.passed).count();
    let failed = total.saturating_sub(passed);
    let report = TalkClarityReport { total, passed, failed, pass: failed == 0, checks };

    if !Path::new("target").exists() {
        fs::create_dir_all("target").context("create target directory")?;
    }
    fs::write(
        "target/guild-service-talk-clarity.json",
        serde_json::to_string_pretty(&report).context("serialize guild talk clarity report")?,
    )
    .context("write target/guild-service-talk-clarity.json")?;
    fs::write("target/guild-service-talk-clarity.md", markdown(&report))
        .context("write target/guild-service-talk-clarity.md")?;

    println!(
        "guild service talk clarity: total={} passed={} failed={}",
        report.total, report.passed, report.failed
    );
    if !report.pass {
        bail!("guild service talk clarity failed");
    }
    Ok(())
}
