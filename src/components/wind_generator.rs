use amethyst::ecs::{Component, DenseVecStorage};

pub struct WindGenerator {
    width: f32,
    height: f32,
    speed: f32,
}

impl WindGenerator {
    pub fn new(width: f32, height: f32, speed: f32) -> Self {
        Self {
            width,
            height,
            speed,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

impl Component for WindGenerator {
    type Storage = DenseVecStorage<Self>;
}