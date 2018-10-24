use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct GravityAffected {
    pub terminal_velocity: f32,
}

impl GravityAffected {
    pub fn new(terminal_velocity: f32) -> Self {
        Self { terminal_velocity }
    }
}

impl Component for GravityAffected {
    type Storage = DenseVecStorage<Self>;
}
