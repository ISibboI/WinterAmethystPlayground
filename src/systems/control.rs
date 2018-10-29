use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use components::{Velocity, WorldCollisionAffected};
use entities::Player;

pub struct ControlSystem;

impl ControlSystem {
    fn change_velocity(velocity: &mut Velocity, target: f32, sharpness: f32) {
        let new_velocity = target * sharpness + velocity.velocity.x * (1.0 - sharpness);
        velocity.velocity.x = new_velocity;
    }
}

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, WorldCollisionAffected>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (players, world_collision_affecteds, mut velocities, input_handler): <Self as System<'s>>::SystemData,
    ) {
        let movement = input_handler.axis_value("move").unwrap() as f32;
        let jump = input_handler.action_is_down("jump").unwrap();

        for (player, world_collision_affected, mut velocity) in
            (&players, &world_collision_affecteds, &mut velocities).join()
        {
            if world_collision_affected.on_ground && jump {
                velocity.velocity.y = player.jump_power();
            }

            let mut target = movement * player.speed();
            let mut sharpness = 0.5;
            if !world_collision_affected.on_ground {
                sharpness *= 0.3;
            }
            Self::change_velocity(&mut velocity, target, sharpness);
        }
    }
}
