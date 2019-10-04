use amethyst::{
    ecs::{Component, DenseVecStorage},
    core::math::base::Vector2,
};

pub struct Velocity {
    pub velocity: Vector2<f32>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            velocity: Vector2::new(x, y),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
