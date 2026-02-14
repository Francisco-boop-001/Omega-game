use crate::{
    default_combat_sequence, equipment_effect_profile, monster_index_at, push_or_refresh_status,
    remove_monster_with_drops, CombatLine, CombatManeuver, CombatStep, Direction, Event, Faction,
    GameState, RandomSource,
};

fn next_combat_step(state: &mut GameState) -> CombatStep {
    if state.combat_sequence.is_empty() {
        state.combat_sequence = default_combat_sequence();
    }
    let idx = state.combat_sequence_cursor % state.combat_sequence.len();
    let step = state.combat_sequence[idx].clone();
    state.combat_sequence_cursor = (state.combat_sequence_cursor + 1) % state.combat_sequence.len();
    step
}

pub fn resolve_attack_command<R: RandomSource>(
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
