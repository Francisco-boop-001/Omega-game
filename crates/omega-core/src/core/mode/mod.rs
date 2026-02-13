pub mod classic;
pub mod contracts;
pub mod modern;

use crate::{Command, Event, GameMode, GameState};
use contracts::{ModeConfig, ModePolicySet};

pub fn mode_config(mode: GameMode) -> ModeConfig {
    match mode {
        GameMode::Classic => ModeConfig {
            mode,
            content_pack_id: "classic",
            save_slot_namespace: "classic",
            classic_freeze_contract: true,
        },
        GameMode::Modern => ModeConfig {
            mode,
            content_pack_id: "modern",
            save_slot_namespace: "modern",
            classic_freeze_contract: false,
        },
    }
}

pub fn policy_set_for(mode: GameMode) -> &'static ModePolicySet {
    match mode {
        GameMode::Classic => classic::policy_set(),
        GameMode::Modern => modern::policy_set(),
    }
}

pub fn apply_before_command(
    policies: &ModePolicySet,
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) {
    policies.traversal.before_command(state, command, events);
    policies.combat.before_command(state, command, events);
    policies.magic.before_command(state, command, events);
    policies.item.before_command(state, command, events);
    policies.economy.before_command(state, command, events);
    policies.service.before_command(state, command, events);
    policies.quest.before_command(state, command, events);
    policies.victory.before_command(state, command, events);
}

pub fn apply_after_command(
    policies: &ModePolicySet,
    state: &mut GameState,
    command: &Command,
    events: &mut Vec<Event>,
) {
    policies.victory.after_command(state, command, events);
    policies.quest.after_command(state, command, events);
    policies.service.after_command(state, command, events);
    policies.economy.after_command(state, command, events);
    policies.item.after_command(state, command, events);
    policies.magic.after_command(state, command, events);
    policies.combat.after_command(state, command, events);
    policies.traversal.after_command(state, command, events);
}
