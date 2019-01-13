use amethyst::assets::PrefabData;
use amethyst::core::transform::Transform;
use amethyst::ecs::{Component, Entity, error::Error, VecStorage, WriteStorage};
use amethyst::renderer::SpriteRender;

use components::GravityAffected;
use components::WorldCollisionAffected;

#[derive(Clone, Serialize, Deserialize)]
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

pub struct PlayerPrefab {
    player: Player,
    transform: Transform,
    sprite_render: SpriteRender,
    gravity_affected: GravityAffected,
    world_collision_affected: WorldCollisionAffected,
}

impl<'a> PrefabData<'a> for PlayerPrefab {
    type SystemData = (WriteStorage<'a, Player>, WriteStorage<'a, Transform>, WriteStorage<'a, SpriteRender>, WriteStorage<'a, GravityAffected>, WriteStorage<'a, WorldCollisionAffected>);
    type Result = ();

    fn add_to_entity(&self, entity: Entity, (players, transforms, sprite_renders, gravity_affecteds, world_collision_affecteds): &mut <Self as PrefabData<'a>>::SystemData, entities: &[Entity]) -> Result<<Self as PrefabData<'a>>::Result, Error> {
        players.insert(entity, self.player.clone())?;
        transforms.insert(entity, self.transform.clone())?;
        sprite_renders.insert(entity, self.sprite_render.clone())?;
        gravity_affecteds.insert(entity, self.gravity_affected.clone())?;
        world_collision_affecteds.insert(entity, self.world_collision_affected.clone())?;

        unimplemented!()
    }
}