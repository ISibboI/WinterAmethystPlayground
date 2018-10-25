use amethyst::ecs::{Component, DenseVecStorage};

pub struct Animated {
    pub time: f32
}

impl Default for Animated {
    fn default() -> Self {
        Self {time: 0.0}
    }
}

impl Component for Animated {
    type Storage = DenseVecStorage<Self>;
}