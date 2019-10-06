use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::math::base::Vector2,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use std::collections::HashMap;

use geometry::Rectangle;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Level {
    name: String,
    bounding_box: Rectangle,
    entry_points: HashMap<String, Vector2<f32>>,
}

impl Component for Level {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct LevelConfig {
    levels: Vec<Level>,
}

impl Component for LevelConfig {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Debug)]
pub struct LevelStore {
    pub name_level_map: HashMap<String, usize>,
    pub current_level: usize,
    pub levels: Vec<Level>,
}

impl Level {
    pub fn new(name: String, bounding_box: Rectangle, default_entry_point: Vector2<f32>) -> Self {
        let mut entry_points = HashMap::new();
        entry_points.insert("default".to_owned(), default_entry_point);
        Self {
            name,
            bounding_box,
            entry_points,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bounding_box(&self) -> &Rectangle {
        &self.bounding_box
    }

    pub fn entry_point(&self, name: &str) -> &Vector2<f32> {
        self.entry_points.get(name).unwrap_or_else(|| {
            panic!(
                "Entry point '{}' into level '{}' not found",
                name, self.name
            )
        })
    }

    /*pub fn bounding_box_mut(&mut self) -> &mut Rectangle {
        &mut self.bounding_box
    }*/
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelChange {
    pub name: String,
    pub entry_point: String,
}

impl From<LevelConfig> for LevelStore {
    fn from(level_config: LevelConfig) -> Self {
        let mut level_store = LevelStore {
            name_level_map: HashMap::default(),
            current_level: 0,
            levels: Vec::default(),
        };
        for level in level_config.levels {
            level_store.name_level_map.insert(level.name.clone(), level_store.levels.len());
            level_store.levels.push(level);
        }
        level_store
    }
}

impl LevelStore {
    pub fn get_current_level(&self) -> &Level {
        &self.levels[self.current_level]
    }

    pub fn change_level(&mut self, level_change: &LevelChange) {
        self.current_level = self.name_level_map.get(&level_change.name).unwrap_or_else(|| panic!("Unknown level name: {}", level_change.name)).clone();
    }
}