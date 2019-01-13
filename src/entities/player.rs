use amethyst::assets::{PrefabData, PrefabError, ProgressCounter};
use amethyst::core::transform::Transform;
use amethyst::ecs::{Component, Entity, VecStorage, WriteStorage};
use amethyst::renderer::{SpriteRender, Transparent};

use components::{Animated, Velocity, WorldCollisionAffected};
use components::GravityAffected;

#[derive(Default, Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    speed: f32,
    jump_power: f32,
}

impl Player {
    pub fn new(speed: f32, jump_power: f32) -> Self {
        Self { speed, jump_power }
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

#[derive(Serialize, Deserialize, PrefabData)]
pub struct PlayerPrefab {
    #[prefab(Component)]
    player: Player,
    #[prefab(Component)]
    transform: Transform,
    #[prefab(Component)]
    sprite_render: SpriteRender,
    #[prefab(Component)]
    gravity_affected: GravityAffected,
    #[prefab(Component)]
    world_collision_affected: WorldCollisionAffected,
    #[prefab(Component)]
    #[serde(default)]
    velocity: Velocity,
    #[prefab(Component)]
    #[serde(default)]
    animated: Animated,
    #[prefab(Component)]
    #[serde(default)]
    transparent: Transparent,
}