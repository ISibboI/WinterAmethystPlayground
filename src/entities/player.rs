use amethyst::ecs::{Component, VecStorage};

pub struct Player {
    speed: f32,
    jump_power: f32,
}

impl Player {
    pub fn new(speed: f32, jump_power: f32) -> Self {
        Self {
            speed,
            jump_power,
        }
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn jump_power(&self) -> f32 {
        self.jump_power
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
