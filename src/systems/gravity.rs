use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};
use components::{GravityAffected, Velocity};

pub struct GravitySystem;

impl<'s> System<'s> for GravitySystem {
    type SystemData = (
        ReadStorage<'s, GravityAffected>,
        WriteStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (gravity_affecteds, mut velocities, time): <Self as System<'s>>::SystemData) {
        for (gravity_affected, velocity) in (&gravity_affecteds, &mut velocities).join() {
            velocity.velocity.y = (velocity.velocity.y - 350.0 * time.fixed_seconds())
                .max(-gravity_affected.terminal_velocity);
        }
    }
}
