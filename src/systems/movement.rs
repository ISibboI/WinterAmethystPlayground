use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use components::velocity::Velocity;

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
