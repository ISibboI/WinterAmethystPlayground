use amethyst::{
    core::Transform,
    ecs::{Join, Read, System, WriteStorage},
};

use components::{Velocity, WorldCollisionAffected};
use levels::Level;

const GROUND_HEIGHT: f32 = 8.0;

pub struct WorldCollisionSystem;

impl<'s> System<'s> for WorldCollisionSystem {
    type SystemData = (
        WriteStorage<'s, WorldCollisionAffected>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        Read<'s, Level>,
    );

    fn run(
        &mut self,
        (mut world_collision_affecteds, mut transforms, mut velocities, level): <Self as System<
            's,
        >>::SystemData,
    ) {
        for (mut world_collision_affected, transform, velocity) in (
            &mut world_collision_affecteds,
            &mut transforms,
            &mut velocities,
        )
            .join()
        {
            if transform.translation().y
                < level.bounding_box().min_y() + world_collision_affected.height / 2.0 + GROUND_HEIGHT
            {
                world_collision_affected.on_ground = true;
                transform.translation_mut().y = level.bounding_box().min_y()
                    + world_collision_affected.height / 2.0
                    + GROUND_HEIGHT;
                velocity.velocity.y = 0.0;
            } else {
                world_collision_affected.on_ground = false;
            }

            if transform.translation().x
                < level.bounding_box().min_x() + world_collision_affected.width / 2.0
            {
                world_collision_affected.collides_left = true;
                transform.translation_mut().x =
                    level.bounding_box().min_x() + world_collision_affected.width / 2.0;
                velocity.velocity.x = velocity.velocity.x.max(0.0);
            } else {
                world_collision_affected.collides_left = false;
            }

            if transform.translation().x
                > level.bounding_box().max_x() - world_collision_affected.width / 2.0
            {
                world_collision_affected.collides_right = true;
                transform.translation_mut().x =
                    level.bounding_box().max_x() - world_collision_affected.width / 2.0;
                velocity.velocity.x = velocity.velocity.x.min(0.0);
            } else {
                world_collision_affected.collides_right = false;
            }
        }
    }
}
