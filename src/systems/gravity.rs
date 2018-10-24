use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;
use entities::snowflake::Snowflake;
use pong::ARENA_HEIGHT;
use pong::ARENA_WIDTH;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::distributions::Uniform;
use resources::SpriteSheets;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (ReadStorage<'s, GravityAffected>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (gravity_affecteds, mut velocities): <Self as System<'s>>::SystemData) {
        for (gravity_affected, mut velocity) in (&gravity_affecteds, &mut velocities).join() {
            velocity.velocity.y = (velocity.velocity.y - 5.0 / 60.0).min(gravity_affected.terminal_velocity);
        }
    }
}
