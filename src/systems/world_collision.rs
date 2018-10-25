use amethyst::core::Transform;
use amethyst::ecs::{System, Join, WriteStorage};
use components::{WorldCollisionAffected, Velocity};
use states::game::ARENA_WIDTH;

pub struct WorldCollisionSystem;

impl<'s> System<'s> for WorldCollisionSystem {
    type SystemData = (WriteStorage<'s, WorldCollisionAffected>, WriteStorage<'s, Transform>, WriteStorage<'s, Velocity>);

    fn run(&mut self, (mut world_collision_affecteds, mut transforms, mut velocities): <Self as System<'s>>::SystemData) {
        for (mut world_collision_affected, mut transform, mut velocity) in (&mut world_collision_affecteds, &mut transforms, &mut velocities).join() {
            if transform.translation.y < world_collision_affected.height / 2.0 {
                world_collision_affected.on_ground = true;
                transform.translation.y = world_collision_affected.height / 2.0;
                velocity.velocity.y = 0.0;
            } else {
                world_collision_affected.on_ground = false;
            }

            if transform.translation.x < world_collision_affected.width / 2.0 {
                world_collision_affected.collides_left = true;
                transform.translation.x = world_collision_affected.width / 2.0;
                velocity.velocity.x = velocity.velocity.x.max(0.0);
            } else {
                world_collision_affected.collides_left = false;
            }

            if transform.translation.x > ARENA_WIDTH - world_collision_affected.width / 2.0 {
                world_collision_affected.collides_right = true;
                transform.translation.x = ARENA_WIDTH - world_collision_affected.width / 2.0;
                velocity.velocity.x = velocity.velocity.x.min(0.0);
            } else {
                world_collision_affected.collides_right = false;
            }
        }
    }
}