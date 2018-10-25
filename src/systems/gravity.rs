use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, GravityAffected>,
        WriteStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (gravity_affecteds, mut velocities, time): <Self as System<'s>>::SystemData) {
        for (gravity_affected, mut velocity) in (&gravity_affecteds, &mut velocities).join() {
            velocity.velocity.y = (velocity.velocity.y - 5.0 * time.delta_seconds())
                .min(gravity_affected.terminal_velocity)
                .max(-gravity_affected.terminal_velocity);
        }
    }
}
