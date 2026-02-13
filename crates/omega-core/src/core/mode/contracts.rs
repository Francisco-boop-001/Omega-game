use crate::{Command, Event, GameMode, GameState};

pub trait CombatPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait MagicPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait ItemPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait ServicePolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait QuestPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait TraversalPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait VictoryPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

pub trait EconomyPolicy: Sync {
    fn before_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {
    }
    fn after_command(&self, _state: &mut GameState, _command: &Command, _events: &mut Vec<Event>) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeConfig {
    pub mode: GameMode,
    pub content_pack_id: &'static str,
    pub save_slot_namespace: &'static str,
    pub classic_freeze_contract: bool,
}

pub struct ModePolicySet {
    pub combat: &'static dyn CombatPolicy,
    pub magic: &'static dyn MagicPolicy,
    pub item: &'static dyn ItemPolicy,
    pub service: &'static dyn ServicePolicy,
    pub quest: &'static dyn QuestPolicy,
    pub traversal: &'static dyn TraversalPolicy,
    pub victory: &'static dyn VictoryPolicy,
    pub economy: &'static dyn EconomyPolicy,
}
