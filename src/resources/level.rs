use std::collections::HashMap;
use amethyst::assets::{PrefabData, ProgressCounter};
use amethyst::ecs::{Entity, Component, DenseVecStorage, WriteStorage};
use amethyst::Error;

use geometry::Rectangle;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Level {
    name: String,
    bounding_box: Rectangle,
}

impl Component for Level {
    type Storage = DenseVecStorage<Self>;
}

pub type LevelStore = HashMap<String, Level>;

impl Level {
    pub fn new(name: String, bounding_box: Rectangle) -> Self {
        Self {
            name,
            bounding_box
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bounding_box(&self) -> &Rectangle {
        &self.bounding_box
    }

    /*pub fn bounding_box_mut(&mut self) -> &mut Rectangle {
        &mut self.bounding_box
    }*/
}