use omega_core::{
    Command, DeterministicRng, Event, GameState, Item, MapBounds, Position, StatusEffect, step,
};

fn setup_spell_test() -> (GameState, DeterministicRng) {
    let mut state = GameState::new(MapBounds { width: 20, height: 20 });
    state.player.position = Position { x: 10, y: 10 };
    // Grant all spells
    for spell in &mut state.spellbook.spells {
        spell.known = true;
    }
    state.spellbook.max_mana = 1000; // Give plenty of mana
    state.spellbook.mana = 1000;
    state.options.confirm = false;
    let rng = DeterministicRng::seeded(12345);
    (state, rng)
}

fn cast_spell(state: &mut GameState, rng: &mut DeterministicRng, spell_name: &str) {
    // Initiate spell casting
    step(state, Command::Legacy { token: "m".to_string() }, rng);
    // Type the spell name
    step(state, Command::Legacy { token: spell_name.to_string() }, rng);
    // Commit the spell
    step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
}

fn cast_spell_with_note(
    state: &mut GameState,
    rng: &mut DeterministicRng,
    spell_name: &str,
) -> String {
    step(state, Command::Legacy { token: "m".to_string() }, rng);
    step(state, Command::Legacy { token: spell_name.to_string() }, rng);
    let outcome = step(state, Command::Legacy { token: "<enter>".to_string() }, rng);

    outcome
        .events
        .iter()
        .filter_map(|e| match e {
            Event::LegacyHandled { token, note, .. } if token == "m" => Some(note.clone()),
            _ => None,
        })
        .next_back()
        .unwrap_or_default()
}

fn cast_projectile_spell(
    state: &mut GameState,
    rng: &mut DeterministicRng,
    spell_name: &str,
    direction: &str,
) -> Vec<Event> {
    step(state, Command::Legacy { token: "m".to_string() }, rng);
    step(state, Command::Legacy { token: spell_name.to_string() }, rng);
    step(state, Command::Legacy { token: "<enter>".to_string() }, rng);
    step(state, Command::Legacy { token: direction.to_string() }, rng);
    let outcome = step(state, Command::Legacy { token: ".".to_string() }, rng);
    outcome.events
}

#[test]
fn test_healing_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.player.stats.max_hp = 50;
    state.player.stats.hp = 10;

    cast_spell(&mut state, &mut rng, "healing");

    // Healing restores 14 HP (min max_hp)
    assert_eq!(state.player.stats.hp, 24);
}

#[test]
fn test_restoration_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.player.stats.max_hp = 50;
    state.player.stats.hp = 10;
    state.status_effects.push(StatusEffect {
        id: "poison".to_string(),
        remaining_turns: 10,
        magnitude: 1,
    });

    cast_spell(&mut state, &mut rng, "restoration");

    assert_eq!(state.player.stats.hp, 50);
    assert!(!state.status_effects.iter().any(|e| e.id == "poison"));
}

#[test]
fn test_curing_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.status_effects.push(StatusEffect {
        id: "poison".to_string(),
        remaining_turns: 10,
        magnitude: 1,
    });

    cast_spell(&mut state, &mut rng, "curing");

    assert!(!state.status_effects.iter().any(|e| e.id == "poison"));
}

#[test]
fn test_teleport_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_pos = state.player.position;

    cast_spell(&mut state, &mut rng, "teleport");

    assert_ne!(state.player.position, start_pos);
}

#[test]
fn test_invisibility_spell() {
    let (mut state, mut rng) = setup_spell_test();

    cast_spell(&mut state, &mut rng, "invisibility");

    assert!(state.status_effects.iter().any(|e| e.id == "invisible"));
}

#[test]
fn test_heroism_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_attack_max = state.player.stats.attack_max;

    cast_spell(&mut state, &mut rng, "heroism");

    assert!(state.status_effects.iter().any(|e| e.id == "heroism"));
    assert_eq!(state.player.stats.attack_max, start_attack_max + 2);
}

#[test]
fn test_levitate_spell() {
    let (mut state, mut rng) = setup_spell_test();

    cast_spell(&mut state, &mut rng, "levitate");

    assert!(state.status_effects.iter().any(|e| e.id == "levitate"));
}

#[test]
fn test_nutrition_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_food = state.food;

    cast_spell(&mut state, &mut rng, "nutrition");

    assert_eq!(state.food, start_food + 12);
}

#[test]
fn test_accuracy_spell() {
    let (mut state, mut rng) = setup_spell_test();
    cast_spell(&mut state, &mut rng, "accuracy");
    assert!(state.status_effects.iter().any(|e| e.id == "accuracy"));
}

#[test]
fn test_alertness_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.status_effects.push(StatusEffect {
        id: "poison".to_string(),
        remaining_turns: 10,
        magnitude: 1,
    });
    state.status_effects.push(StatusEffect {
        id: "immobile".to_string(),
        remaining_turns: 10,
        magnitude: 1,
    });

    cast_spell(&mut state, &mut rng, "alertness");

    assert!(!state.status_effects.iter().any(|e| e.id == "poison"));
    assert!(!state.status_effects.iter().any(|e| e.id == "immobile"));
}

#[test]
fn test_breathing_spell() {
    let (mut state, mut rng) = setup_spell_test();
    cast_spell(&mut state, &mut rng, "breathing");
    assert!(state.status_effects.iter().any(|e| e.id == "breathing"));
}

#[test]
fn test_haste_spell() {
    let (mut state, mut rng) = setup_spell_test();
    cast_spell(&mut state, &mut rng, "haste");
    assert!(state.status_effects.iter().any(|e| e.id == "haste"));
}

#[test]
fn test_regeneration_spell() {
    let (mut state, mut rng) = setup_spell_test();
    cast_spell(&mut state, &mut rng, "regeneration");
    assert!(state.status_effects.iter().any(|e| e.id == "regen"));
}

#[test]
fn test_sanctuary_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.legal_heat = 10;
    cast_spell(&mut state, &mut rng, "sanctuary");
    assert!(state.status_effects.iter().any(|e| e.id == "sanctuary"));
    assert!(state.legal_heat < 10);
}

#[test]
fn test_shadow_form_spell() {
    let (mut state, mut rng) = setup_spell_test();
    cast_spell(&mut state, &mut rng, "shadow form");
    assert!(state.status_effects.iter().any(|e| e.id == "shadow_form"));
}

#[test]
fn test_wishing_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let initial_count = state.player.inventory.len() + state.ground_items.len();

    cast_spell(&mut state, &mut rng, "wishing");

    let final_count = state.player.inventory.len() + state.ground_items.len();
    assert!(final_count > initial_count);
}

#[test]
fn test_summoning_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let initial_monsters = state.monsters.len();

    cast_spell(&mut state, &mut rng, "summoning");

    assert_eq!(state.monsters.len(), initial_monsters + 1);
}

#[test]
fn test_return_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let target = Position { x: 5, y: 5 };
    state.topology.last_city_position = Some(target);
    state.player.position = Position { x: 10, y: 10 };

    cast_spell(&mut state, &mut rng, "return");

    assert_eq!(state.player.position, target);
}

#[test]
fn test_ritual_magic_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_favor = state.progression.deity_favor;
    let start_steps = state.progression.quest_steps_completed;

    cast_spell(&mut state, &mut rng, "ritual magic");

    assert_eq!(state.progression.deity_favor, start_favor + 2);
    assert_eq!(state.progression.quest_steps_completed, start_steps + 1);
}

#[test]
fn test_sanctification_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_favor = state.progression.deity_favor;
    let start_law = state.progression.law_chaos_score;

    cast_spell(&mut state, &mut rng, "sanctification");

    assert_eq!(state.progression.deity_favor, start_favor + 2);
    assert_eq!(state.progression.law_chaos_score, start_law + 2);
}

#[test]
fn test_desecration_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_favor = state.progression.deity_favor;
    let start_law = state.progression.law_chaos_score;

    cast_spell(&mut state, &mut rng, "desecration");

    assert_eq!(state.progression.deity_favor, start_favor - 1);
    assert_eq!(state.progression.law_chaos_score, start_law - 2);
}

#[test]
fn test_magic_missile_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    // Cast magic missile East
    let events = cast_projectile_spell(&mut state, &mut rng, "magic missile", "l"); // 'l' is East in vi keys

    assert!(events.iter().any(|e| matches!(e, Event::MonsterAttacked { .. })));
}

#[test]
fn test_firebolt_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 20,
            max_hp: 20,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    let events = cast_projectile_spell(&mut state, &mut rng, "firebolt", "l");

    assert!(events.iter().any(|e| matches!(e, Event::MonsterAttacked { .. })));
}

#[test]
fn test_disrupt_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 20,
            max_hp: 20,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    let events = cast_projectile_spell(&mut state, &mut rng, "disrupt", "l");

    assert!(events.iter().any(|e| matches!(e, Event::MonsterAttacked { .. })));
}

#[test]
fn test_ball_lightning_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 20,
            max_hp: 20,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    cast_spell(&mut state, &mut rng, "ball lightning");

    // We can't easily check events from cast_spell helper, need to check monster HP
    assert!(state.monsters[0].stats.hp < 20);
}

#[test]
fn test_hellfire_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 50,
            max_hp: 50,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    cast_spell(&mut state, &mut rng, "hellfire");

    assert!(state.monsters[0].stats.hp < 50);
}

#[test]
fn test_sleep_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );
    state.monsters[0].behavior = omega_core::MonsterBehavior::Brute;

    cast_spell(&mut state, &mut rng, "sleep");

    assert_eq!(state.monsters[0].behavior, omega_core::MonsterBehavior::Skirmisher);
}

#[test]
fn test_fear_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );
    state.monsters[0].behavior = omega_core::MonsterBehavior::Brute;

    cast_spell(&mut state, &mut rng, "fear");

    assert_eq!(state.monsters[0].behavior, omega_core::MonsterBehavior::Skirmisher);
}

#[test]
fn test_disintegrate_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    cast_spell(&mut state, &mut rng, "disintegrate");

    assert!(state.monsters.is_empty());
}

#[test]
fn test_apportation_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let item_pos = Position { x: 12, y: 10 };
    state.place_item("gold piece", item_pos);

    cast_spell(&mut state, &mut rng, "apportation");

    assert!(state.ground_items.is_empty());
    assert!(!state.player.inventory.is_empty());
}

#[test]
fn test_enchantment_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let mut item = Item::new(1, "sword");
    item.plus = 0;
    state.player.inventory.push(item);

    step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    step(&mut state, Command::Legacy { token: "enchantment".to_string() }, &mut rng);
    step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

    // Auto-targeting means it should be enchanted now.
    // enchant_item_with_risk adds (delta + 1) -> 2.
    assert_eq!(state.player.inventory[0].plus, 2);
}

#[test]
fn test_blessing_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let mut item = Item::new(1, "sword");
    item.blessing = 0;
    state.player.inventory.push(item);

    let start_favor = state.progression.deity_favor;
    let start_law = state.progression.law_chaos_score;

    step(&mut state, Command::Legacy { token: "m".to_string() }, &mut rng);
    step(&mut state, Command::Legacy { token: "blessing".to_string() }, &mut rng);
    step(&mut state, Command::Legacy { token: "<enter>".to_string() }, &mut rng);

    assert!(state.player.inventory[0].blessing > 0);
    assert_eq!(state.progression.deity_favor, start_favor + 1);
    assert_eq!(state.progression.law_chaos_score, start_law + 1);
}

#[test]
fn test_identification_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let mut item = Item::new(1, "strange potion");
    item.known = false;
    state.player.inventory.push(item);

    let note = cast_spell_with_note(&mut state, &mut rng, "identification");

    assert!(note.contains("identified"));
}

#[test]
fn test_monster_detection_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );

    let note = cast_spell_with_note(&mut state, &mut rng, "monster detection");

    assert!(note.contains("detected 1 nearby signatures"));
}

#[test]
fn test_object_detection_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.place_item("gold piece", Position { x: 11, y: 10 });

    let note = cast_spell_with_note(&mut state, &mut rng, "object detection");

    assert!(note.contains("detected 1 nearby objects"));
}

#[test]
fn test_true_sight_spell() {
    let (mut state, mut rng) = setup_spell_test();

    cast_spell(&mut state, &mut rng, "true sight");

    assert!(state.known_sites.contains(&Position { x: 0, y: 0 }));
}

#[test]
fn test_clairvoyance_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.player.position = Position { x: 10, y: 10 };

    cast_spell(&mut state, &mut rng, "clairvoyance");

    assert!(state.known_sites.contains(&Position { x: 9, y: 10 }));
    assert!(state.known_sites.contains(&Position { x: 11, y: 10 }));
}

#[test]
fn test_self_knowledge_spell() {
    let (mut state, mut rng) = setup_spell_test();

    let note = cast_spell_with_note(&mut state, &mut rng, "self knowledge");

    assert!(note.contains("hp="));
    assert!(note.contains("gold="));
}

#[test]
fn test_energy_drain_spell() {
    let (mut state, mut rng) = setup_spell_test();

    cast_spell(&mut state, &mut rng, "energy drain");
}

#[test]
fn test_polymorph_spell() {
    let (mut state, mut rng) = setup_spell_test();
    state.spawn_monster(
        "rat",
        Position { x: 11, y: 10 },
        omega_core::Stats {
            hp: 10,
            max_hp: 10,
            attack_min: 1,
            attack_max: 1,
            defense: 0,
            weight: 1,
        },
    );
    let original_name = state.monsters[0].name.clone();

    cast_spell(&mut state, &mut rng, "polymorph");

    assert_ne!(state.monsters[0].name, original_name);
}

#[test]
fn test_dispelling_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let mut item = Item::new(1, "cursed ring");
    item.blessing = -1;
    item.used = true;
    state.player.inventory.push(item);
    state.player.equipment.ring_1 = Some(1);

    let note = cast_spell_with_note(&mut state, &mut rng, "dispelling");

    assert!(!note.is_empty());
}

#[test]
fn test_the_warp_spell() {
    let (mut state, mut rng) = setup_spell_test();
    let start_pos = state.player.position;

    cast_spell(&mut state, &mut rng, "the warp");

    assert_ne!(state.player.position, start_pos);
}
