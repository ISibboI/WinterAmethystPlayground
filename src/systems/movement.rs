use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use components::Velocity;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, time): <Self as System<'s>>::SystemData) {
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.translation_mut().x += velocity.velocity.x * time.fixed_seconds();
            transform.translation_mut().y += velocity.velocity.y * time.fixed_seconds();
        }
    }
}
