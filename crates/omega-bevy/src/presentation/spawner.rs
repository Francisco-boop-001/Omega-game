use bevy::prelude::*;
use crate::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum SpawnerCategory {
    #[default]
    Monster,
    Item,
    Hazard,
}

#[derive(Debug, Clone, Reflect)]
pub struct SpawnerEntry {
    pub id: String,
    pub name: String,
    pub category: SpawnerCategory,
}

#[derive(Debug, Clone, Resource, Reflect)]
pub struct SpawnerState {
    pub is_visible: bool,
    pub selected_category: SpawnerCategory,
    pub selected_id: String,
    pub catalog: Vec<SpawnerEntry>,
}

impl Default for SpawnerState {
    fn default() -> Self {
        let catalog = vec![
            SpawnerEntry { id: "rat".to_string(), name: "Rat".to_string(), category: SpawnerCategory::Monster },
            SpawnerEntry { id: "goblin".to_string(), name: "Goblin".to_string(), category: SpawnerCategory::Monster },
            SpawnerEntry { id: "practice_blade".to_string(), name: "Practice Blade".to_string(), category: SpawnerCategory::Item },
            SpawnerEntry { id: "trap".to_string(), name: "Trap".to_string(), category: SpawnerCategory::Hazard },
        ];
        
        Self {
            is_visible: true,
            selected_category: SpawnerCategory::Monster,
            selected_id: "rat".to_string(),
            catalog,
        }
    }
}
