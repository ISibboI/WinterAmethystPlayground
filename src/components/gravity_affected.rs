use amethyst::{
    assets::{Prefab, PrefabData, ProgressCounter},
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
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
