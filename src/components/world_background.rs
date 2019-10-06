use amethyst::ecs::{storage::NullStorage, Component};

#[derive(Default)]
pub struct WorldBackground;

impl Component for WorldBackground {
    type Storage = NullStorage<Self>;
}