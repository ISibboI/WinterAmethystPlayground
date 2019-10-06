use std::collections::HashMap;
use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::ecs::{Entity, Component, DenseVecStorage, WriteStorage};
use amethyst::Error;
use amethyst::core::math::base::Vector2;

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
pub struct LevelStore {
    levels: HashMap<String, Level>
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
        self.entry_points.get(name).unwrap_or_else(|| panic!("Entry point '{}' into level '{}' not found", name, self.name))
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