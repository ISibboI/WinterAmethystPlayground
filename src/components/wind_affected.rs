use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct WindAffected {
    pub air_resistance: f32,
    pub time_offset: f32,
}

impl WindAffected {
    pub fn new(air_resistance: f32, time_offset: f32) -> Self {
        Self {
            air_resistance,
            time_offset,
        }
    }
}

impl Component for WindAffected {
    type Storage = DenseVecStorage<Self>;
}
