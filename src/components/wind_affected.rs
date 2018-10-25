use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct WindAffected {
    pub air_resistance: f32,
}

impl WindAffected {
    pub fn new(air_resistance: f32) -> Self {
        Self { air_resistance }
    }
}

impl Component for WindAffected {
    type Storage = DenseVecStorage<Self>;
}
