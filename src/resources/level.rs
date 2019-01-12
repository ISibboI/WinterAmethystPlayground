use std::collections::HashMap;

use euclid::{TypedPoint2D, TypedRect, TypedSize2D};

#[derive(Clone, Debug)]
pub struct Level {
    pub bounding_box: TypedRect<f32>,
}

pub type LevelStore = HashMap<String, Level>;

impl Default for Level {
    fn default() -> Self {
        Self {
            bounding_box: TypedRect::new(
                TypedPoint2D::new(0.0, 0.0),
                TypedSize2D::new(100.0, 100.0),
            ),
        }
    }
}

