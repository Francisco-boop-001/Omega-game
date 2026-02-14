use anyhow::{Context, Result};
use omega_bevy::{
    AppState, BevyKey, FrontendRuntime, build_runtime_app_with_options_and_mode, enqueue_input,
    runtime_frame, runtime_status,
};
use omega_content::bootstrap_game_state_with_mode;
use omega_core::{
    Alignment, CharacterCreation, GameMode, GameState, LegacyQuestionnaireAnswers,
    LegacyQuestionnaireProfile, apply_character_creation, apply_legacy_questionnaire_profile,
    default_character_archetypes, derive_legacy_questionnaire_creation,
};
use omega_save::decode_state_json_for_mode;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const CREATION_MODE_PROMPT: &str = "Creation mode: [1] direct archetype [2] questionnaire: ";

#[derive(Debug, Clone, PartialEq, Eq)]
struct CreationSelection {
    creation: CharacterCreation,
    legacy_profile: Option<LegacyQuestionnaireProfile>,
}

fn read_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush().context("flush stdout")?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).context("read stdin line")?;
    Ok(input.trim().to_string())
}

fn read_yes_no(prompt: &str) -> Result<bool> {
    loop {
        let value = read_line(prompt)?;
        match value.chars().next().map(|ch| ch.to_ascii_lowercase()) {
            Some('y') => return Ok(true),
            Some('n') => return Ok(false),
            _ => println!("Please answer y or n."),
        }
    }
}

fn read_i32(prompt: &str) -> Result<i32> {
    loop {
        let value = read_line(prompt)?;
        match value.parse::<i32>() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_percentile(prompt: &str) -> Result<i32> {
    loop {
        let percentile = read_i32(prompt)?;
        if percentile < 100 {
            return Ok(percentile);
        }
        println!("That's impossible!");
    }
}

fn read_preference(prompt: &str) -> Result<char> {
    loop {
        let value = read_line(prompt)?;
        match value.chars().next().map(|ch| ch.to_ascii_lowercase()) {
            Some('m' | 'f' | 'y' | 'n') => {
                return Ok(value.chars().next().unwrap().to_ascii_lowercase());
            }
            _ => println!("Please choose one of: m, f, y, n."),
        }
    }
}

fn questionnaire_character_creation(name: String) -> Result<CreationSelection> {
    println!("Questionnaire mode selected.");
    println!("OK, now try to answer the following questions honestly:");

    let bench_press_lbs = read_i32("How many pounds can you bench press? ")?;
    if ((bench_press_lbs - 120) / 30 + 9) > 18 {
        println!("Right, make me believe THAT's true.");
        println!("Even if it's true, I don't believe it.");
    }

    let took_iq_test = read_yes_no("Took an official IQ test? [yn] ")?;
    let iq_score = if took_iq_test { read_i32("So, whadja get? ")? } else { 0 };
    if took_iq_test && (iq_score / 10) > 18 {
        println!("I'm not convinced...");
        println!("If you're so smart, why aren't you rich?");
    }

    let took_undergraduate_exam = read_yes_no("Took Undergraduate entrance exams? [yn] ")?;
    let undergraduate_percentile =
        if took_undergraduate_exam { read_percentile("So, what percentile? ")? } else { 0 };

    let took_graduate_exam = read_yes_no("Took Graduate entrance exams? [yn] ")?;
    let graduate_percentile =
        if took_graduate_exam { read_percentile("So, what percentile? ")? } else { 0 };

    let pretty_dumb = if !(took_iq_test || took_undergraduate_exam || took_graduate_exam) {
        read_yes_no("Pretty dumb, aren't you? [yn] ")?
    } else {
        false
    };
    if !(took_iq_test || took_undergraduate_exam || took_graduate_exam) {
        if pretty_dumb {
            println!("I thought so...");
        } else {
            println!("Well, not *that* dumb.");
        }
    }

    let can_dance = read_yes_no("Can you dance? [yn] ")?;
    let dance_well = if can_dance { read_yes_no("Well? [yn] ")? } else { false };

    let has_martial_training =
        read_yes_no("Do you have training in a martial art or gymnastics? [yn] ")?;
    let has_dan_rank = if has_martial_training {
        read_yes_no("Do you have dan rank or equivalent? [yn] ")?
    } else {
        false
    };

    let plays_field_sport = read_yes_no("Do you play some field sport? [yn] ")?;
    let good_field_sport =
        if plays_field_sport { read_yes_no("Are you good? [yn] ")? } else { false };

    let does_caving_or_mountaineering = read_yes_no("Do you cave, mountaineer, etc.? [yn] ")?;
    let skates_or_skis = read_yes_no("Do you skate or ski? [yn] ")?;
    let good_at_skating_or_skiing =
        if skates_or_skis { read_yes_no("Well? [yn] ")? } else { false };
    let physically_handicapped = read_yes_no("Are you physically handicapped? [yn] ")?;
    let accident_prone = read_yes_no("Are you accident prone? [yn] ")?;
    let can_ride_bicycle = read_yes_no("Can you ride a bicycle? [yn] ")?;

    let plays_video_games = read_yes_no("Do you play video games? [yn] ")?;
    let gets_high_scores =
        if plays_video_games { read_yes_no("Do you get high scores? [yn] ")? } else { false };
    let archer_fencer_marksman = read_yes_no("Are you an archer, fencer, or marksman? [yn] ")?;
    let good_marksman =
        if archer_fencer_marksman { read_yes_no("A good one? [yn] ")? } else { false };
    let picked_lock = read_yes_no("Have you ever picked a lock? [yn] ")?;
    if picked_lock {
        println!("Really? Well, the police are being notified.");
        println!("Really? Well, the police are being notified..");
        println!("Really? Well, the police are being notified...");
        println!("Really? Well, the police are being notified....");
        println!("Really? Well, the police are being notified.... done!");
    }
    let typing_speed_wpm = read_i32("What's your typing speed (words per minute)? ")?;
    if typing_speed_wpm > 124 {
        println!("Tell me another one...");
    }
    let hand_shaking = read_yes_no("Hold your arm out. Tense your fist. Hand shaking? [yn] ")?;
    let ambidextrous = read_yes_no("Ambidextrous, are you? [yn] ")?;
    let can_cut_deck_one_hand = read_yes_no("Can you cut a deck of cards with one hand? [yn] ")?;
    let can_tie_shoes_blindfolded = read_yes_no("Can you tie your shoes blindfolded? [yn] ")?;

    let gets_colds = read_yes_no("Do you ever get colds? [yn] ")?;
    let colds_frequent = if gets_colds { read_yes_no("Frequently? [yn] ")? } else { false };
    let recent_serious_accident_or_illness =
        read_yes_no("Had any serious accident or illness this year? [yn] ")?;
    let chronic_disease = read_yes_no("Have a chronic disease? [yn] ")?;
    let overweight_or_underweight_20pct =
        read_yes_no("Overweight or underweight by more than 20 percent? [yn] ")?;
    let high_blood_pressure = read_yes_no("High blood pressure? [yn] ")?;
    let smokes = read_yes_no("Do you smoke? [yn] ")?;
    if smokes {
        println!("*cough*");
    }
    let aerobics_classes = read_yes_no("Take aerobics classes? [yn] ")?;
    let miles_can_run = read_i32("How many miles can you run? ")?;
    if miles_can_run >= 26 {
        println!("Right. Sure. Give me a break.");
    }

    let animals_react_oddly = read_yes_no("Do animals react oddly to your presence? [yn] ")?;
    if animals_react_oddly {
        println!("How curious that must be.");
    }
    let can_see_auras = read_yes_no("Can you see auras? [yn] ")?;
    if can_see_auras {
        println!("How strange.");
    }
    let out_of_body_experience = read_yes_no("Ever have an out-of-body experience? [yn] ")?;
    if out_of_body_experience {
        println!("Wow, man! Fly the friendly skies...");
    }
    let cast_spell = read_yes_no("Did you ever cast a spell? [yn] ")?;
    let spell_worked = if cast_spell { read_yes_no("Did it work? [yn] ")? } else { false };
    if cast_spell && spell_worked {
        println!("Sure it did...");
    }
    let has_esp = read_yes_no("Do you have ESP? [yn] ")?;
    if has_esp {
        println!("Somehow, I knew you were going to say that.");
    }
    let has_pk = read_yes_no("Do you have PK? [yn] ")?;
    if has_pk {
        println!("I can't tell you how much that moves me.");
    }
    let believes_in_ghosts = read_yes_no("Do you believe in ghosts? [yn] ")?;
    if believes_in_ghosts {
        println!("I do! I do! I do believe in ghosts!");
    }
    let is_irish = read_yes_no("Are you Irish? [yn] ")?;
    if is_irish {
        println!("Is that blarney or what?");
    }
    let sexual_preference =
        read_preference("Are you sexually interested in males or females? [mfyn] ")?;

    let answers = LegacyQuestionnaireAnswers {
        bench_press_lbs,
        took_iq_test,
        iq_score,
        took_undergraduate_exam,
        undergraduate_percentile,
        took_graduate_exam,
        graduate_percentile,
        pretty_dumb,
        can_dance,
        dance_well,
        has_martial_training,
        has_dan_rank,
        plays_field_sport,
        good_field_sport,
        does_caving_or_mountaineering,
        skates_or_skis,
        good_at_skating_or_skiing,
        physically_handicapped,
        accident_prone,
        can_ride_bicycle,
        plays_video_games,
        gets_high_scores,
        archer_fencer_marksman,
        good_marksman,
        picked_lock,
        typing_speed_wpm,
        hand_shaking,
        ambidextrous,
        can_cut_deck_one_hand,
        can_tie_shoes_blindfolded,
        gets_colds,
        colds_frequent,
        recent_serious_accident_or_illness,
        chronic_disease,
        overweight_or_underweight_20pct,
        high_blood_pressure,
        smokes,
        aerobics_classes,
        miles_can_run,
        animals_react_oddly,
        can_see_auras,
        out_of_body_experience,
        cast_spell,
        spell_worked,
        has_esp,
        has_pk,
        believes_in_ghosts,
        is_irish,
        sexual_preference,
    };
    let derived = derive_legacy_questionnaire_creation(name, &answers);
    println!(
        "Questionnaire result: archetype={}, alignment={:?}, STR {} IQ {} AGI {} DEX {} CON {} POW {}",
        derived.creation.archetype_id,
        derived.creation.alignment,
        derived.profile.strength,
        derived.profile.iq,
        derived.profile.agility,
        derived.profile.dexterity,
        derived.profile.constitution,
        derived.profile.power
    );
    Ok(CreationSelection { creation: derived.creation, legacy_profile: Some(derived.profile) })
}

fn prompt_character_creation() -> Result<CreationSelection> {
    println!();
    println!("=== Character Creation ===");
    let name = read_line("Name (blank for Adventurer): ")?;
    let mode = read_line(CREATION_MODE_PROMPT)?;
    if mode.trim() == "2" {
        return questionnaire_character_creation(name);
    }

    let archetypes = default_character_archetypes();
    for (idx, archetype) in archetypes.iter().enumerate() {
        println!(
            "{}. {} (hp {}, atk {}-{}, def {}, gold {}, mana {})",
            idx + 1,
            archetype.label,
            archetype.stats.max_hp,
            archetype.stats.attack_min,
            archetype.stats.attack_max,
            archetype.stats.defense,
            archetype.starting_gold,
            archetype.starting_mana
        );
    }
    let archetype_choice = read_line("Choose archetype (number or id): ")?;
    let archetype_id = if let Ok(idx) = archetype_choice.parse::<usize>() {
        archetypes
            .get(idx.saturating_sub(1))
            .map(|a| a.id.clone())
            .unwrap_or_else(|| archetypes[0].id.clone())
    } else if !archetype_choice.trim().is_empty() {
        archetype_choice
    } else {
        archetypes[0].id.clone()
    };

    println!("Alignment options: 1) Lawful 2) Neutral 3) Chaotic");
    let alignment_choice = read_line("Choose alignment: ")?;
    let alignment = match alignment_choice.as_str() {
        "1" | "lawful" | "Lawful" => Alignment::Lawful,
        "3" | "chaotic" | "Chaotic" => Alignment::Chaotic,
        _ => Alignment::Neutral,
    };

    Ok(CreationSelection {
        creation: CharacterCreation { name, archetype_id, alignment },
        legacy_profile: None,
    })
}

fn load_bootstrap_state(mode: GameMode) -> Result<GameState> {
    let (mut state, diagnostics) = bootstrap_game_state_with_mode(mode)
        .context("content bootstrap failed (fallback runtime disabled)")?;
    state.options.interactive_sites = true;
    println!(
        "Content bootstrap loaded: source={}, spawn={}, monsters={}, items={}",
        diagnostics.map_source,
        diagnostics.player_spawn_source,
        diagnostics.monster_spawns,
        diagnostics.item_spawns
    );
    Ok(state)
}

fn load_slot(path: &PathBuf, mode: GameMode) -> Result<GameState> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("read save slot {}", path.display()))?;
    let mut state = decode_state_json_for_mode(&raw, mode).context("decode save slot payload")?;
    state.options.interactive_sites = true;
    Ok(state)
}

fn map_char_to_key(ch: char) -> Option<BevyKey> {
    match ch {
        '\r' | '\n' => None,
        ' ' => Some(BevyKey::Char(ch)),
        _ if ch.is_ascii_graphic() => Some(BevyKey::Char(ch)),
        _ => None,
    }
}

fn parse_input_line_to_keys(input: &str) -> Vec<BevyKey> {
    let mut keys = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut idx = 0usize;
    while idx < chars.len() {
        let ch = chars[idx];
        if ch == '^'
            && let Some(next) = chars.get(idx + 1).copied()
            && next.is_ascii_alphabetic()
        {
            keys.push(BevyKey::Ctrl(next.to_ascii_lowercase()));
            idx += 2;
            continue;
        }
        if let Some(key) = map_char_to_key(ch) {
            keys.push(key);
        }
        idx += 1;
    }
    keys
}

fn run_session(
    seed: u64,
    mode: GameMode,
    mut initial_state: GameState,
    bootstrap_state: GameState,
    slot: PathBuf,
) -> Result<()> {
    let mut app = build_runtime_app_with_options_and_mode(seed, mode, bootstrap_state, slot);
    app.update();
    enqueue_input(&mut app, BevyKey::Enter);
    app.update();

    {
        let mut runtime = app.world_mut().resource_mut::<FrontendRuntime>();
        if let Some(session) = runtime.0.session.as_mut() {
            initial_state.options.interactive_sites = true;
            session.state = initial_state;
            session.last_outcome = None;
        }
    }

    loop {
        let status = runtime_status(&app);
        if status.should_quit || status.app_state == AppState::Menu {
            break;
        }

        if let Some(frame) = runtime_frame(&app) {
            println!();
            println!(
                "[Bevy runtime] state={:?} turn={} hp_line={} tiles={}",
                status.app_state,
                frame.hud_lines.first().cloned().unwrap_or_default(),
                frame.hud_lines.get(2).cloned().unwrap_or_default(),
                frame.tiles.len()
            );
            for line in frame.event_lines.iter().rev().take(4).rev() {
                println!("  {line}");
            }
        }

        let input = read_line(
            "Command (:quit to exit, :wizard for wizard mode, keys map directly to legacy commands): ",
        )?;
        if input.eq_ignore_ascii_case(":quit") {
            break;
        }
        if input.eq_ignore_ascii_case(":wizard") || input.eq_ignore_ascii_case(":wiz") {
            enqueue_input(&mut app, BevyKey::F12);
            app.update();
            continue;
        }

        for key in parse_input_line_to_keys(&input) {
            enqueue_input(&mut app, key);
        }
        app.update();
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let start_in_arena = args.iter().any(|arg| arg == "--arena");

    let mode = if start_in_arena {
        GameMode::Modern
    } else {
        loop {
            println!("Select game mode:");
            println!("1. Classic (frozen parity)");
            println!("2. Modern (isolated evolution)");
            let value = read_line("Mode: ")?;
            match value.as_str() {
                "1" => break GameMode::Classic,
                "2" => break GameMode::Modern,
                _ => println!("Unknown mode option."),
            }
        }
    };

    let save_slot = PathBuf::from(format!("target/omega-{}-slot-1.json", mode.as_str()));
    if let Some(parent) = save_slot.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("create save slot directory {}", parent.display()))?;
    }

    let mut seed = 0xBEE5_0001u64;
    if start_in_arena {
        let (mut bootstrap, _) = omega_content::bootstrap_wizard_arena().expect("arena bootstrap");
        bootstrap.mode = GameMode::Modern;
        bootstrap.options.interactive_sites = true;
        run_session(seed, mode, bootstrap.clone(), bootstrap, save_slot.clone())?;
        return Ok(());
    }

    loop {
        println!();
        println!("=== Omega Bevy Launcher ===");
        println!("1. New game (content bootstrap)");
        println!("2. Load game from {}", save_slot.display());
        println!("3. Quit");

        let choice = read_line("Select option: ")?;
        match choice.as_str() {
            "1" => {
                let mut bootstrap = load_bootstrap_state(mode)?;
                let selection = prompt_character_creation()?;
                apply_character_creation(&mut bootstrap, &selection.creation);
                if let Some(profile) = selection.legacy_profile {
                    apply_legacy_questionnaire_profile(&mut bootstrap, profile);
                }
                run_session(seed, mode, bootstrap.clone(), bootstrap, save_slot.clone())?;
                seed = seed.wrapping_add(1);
            }
            "2" => {
                let bootstrap = load_bootstrap_state(mode)?;
                match load_slot(&save_slot, mode) {
                    Ok(loaded) => {
                        run_session(seed, mode, loaded, bootstrap, save_slot.clone())?;
                        seed = seed.wrapping_add(1);
                    }
                    Err(err) => eprintln!("Load failed: {err}"),
                }
            }
            "3" => break,
            _ => println!("Unknown option."),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn questionnaire_derives_expected_alignment_and_archetype() {
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
        let derived = derive_legacy_questionnaire_creation("Ari".to_string(), &answers);
        assert_eq!(derived.creation.archetype_id, "mage");
        assert_eq!(derived.creation.alignment, Alignment::Neutral);
        assert!(derived.profile.power >= 14);
    }

    #[test]
    fn launcher_prompt_mentions_questionnaire_mode() {
        assert!(CREATION_MODE_PROMPT.contains("[2] questionnaire"));
    }

    #[test]
    fn questionnaire_creation_is_applicable_to_runtime_state() {
        let mut state = GameState::default();
        let answers = LegacyQuestionnaireAnswers {
            bench_press_lbs: 120,
            pretty_dumb: true,
            can_ride_bicycle: true,
            can_tie_shoes_blindfolded: true,
            sexual_preference: 'm',
            ..LegacyQuestionnaireAnswers::default()
        };
        let derived = derive_legacy_questionnaire_creation("Mori".to_string(), &answers);
        apply_character_creation(&mut state, &derived.creation);
        apply_legacy_questionnaire_profile(&mut state, derived.profile);
        assert_eq!(state.progression.alignment, Alignment::Neutral);
        assert!(state.player.stats.attack_max > state.player.stats.attack_min);
    }

    #[test]
    fn parser_maps_caret_control_tokens_to_ctrl_keys() {
        let keys = parse_input_line_to_keys("^xab^g");
        assert_eq!(
            keys,
            vec![BevyKey::Ctrl('x'), BevyKey::Char('a'), BevyKey::Char('b'), BevyKey::Ctrl('g')]
        );
    }
}
