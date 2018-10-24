use amethyst::core::specs::world::Builder;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::Component;
use amethyst::ecs::prelude::DenseVecStorage;
use amethyst::prelude::World;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

pub struct Snowflake;

impl Snowflake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Snowflake {
    type Storage = DenseVecStorage<Self>;
}
