use amethyst::core::timing::Time;
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

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, time): <Self as System<'s>>::SystemData) {
        for (velocity, mut transform) in (&velocities, &mut transforms).join() {
            transform.translation.x += velocity.velocity.x * time.delta_seconds();
            transform.translation.y += velocity.velocity.y * time.delta_seconds();
        }
    }
}
