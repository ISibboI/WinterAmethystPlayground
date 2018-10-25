use amethyst::core::cgmath::Vector2;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Velocity {
    pub velocity: Vector2<f32>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            velocity: Vector2 { x, y },
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}