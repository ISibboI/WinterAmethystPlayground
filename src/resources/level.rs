use std::collections::HashMap;

use euclid::{TypedPoint2D, TypedRect, TypedSize2D};

#[derive(Clone, Debug)]
pub struct Level {
    bounding_box: TypedRect<f32>,
}

pub type LevelStore = HashMap<String, Level>;

impl Level {
    pub fn new(bounding_box: TypedRect<f32>) -> Self {
        Self {
            bounding_box
        }
    }

    pub fn bounding_box(&self) -> &TypedRect<f32> {
        &self.bounding_box
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new(TypedRect::new(TypedPoint2D::new(Default::default(), Default::default()), TypedSize2D::new(Default::default(), Default::default())))
    }
}