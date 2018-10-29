use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Snowflake;

impl Snowflake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Snowflake {
    type Storage = DenseVecStorage<Self>;
}
