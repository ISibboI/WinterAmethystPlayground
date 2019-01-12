use std::collections::HashMap;

use geometry::Rectangle;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Level {
    bounding_box: Rectangle,
}

pub type LevelStore = HashMap<String, Level>;

impl Level {
    pub fn new(bounding_box: Rectangle) -> Self {
        Self {
            bounding_box
        }
    }

    pub fn bounding_box(&self) -> &Rectangle {
        &self.bounding_box
    }

    /*pub fn bounding_box_mut(&mut self) -> &mut Rectangle {
        &mut self.bounding_box
    }*/
}