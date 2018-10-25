use amethyst::ecs::{System, Join, Read, ReadStorage, WriteStorage};
use components::{Animated, Velocity, WorldCollisionAffected};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;
use amethyst::core::Time;

const ANIMATION_DELAY: f32 = 0.2;

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (WriteStorage<'s, Animated>, ReadStorage<'s, Velocity>, ReadStorage<'s, WorldCollisionAffected>, WriteStorage<'s, SpriteRender>, Read<'s, Time>, Read<'s, InputHandler<String, String>>);

    fn run(&mut self, (mut animateds, velocities, world_collision_affecteds, mut sprite_renders, time, input_handler): <Self as System<'s>>::SystemData) {
        for (mut animated, velocity, world_collision_affected, mut sprite_render) in (&mut animateds, &velocities, &world_collision_affecteds, &mut sprite_renders).join() {
            let move_axis = input_handler.axis_value("move").unwrap();
            if !world_collision_affected.on_ground {
                sprite_render.sprite_number = 1;
                animated.time = 0.0;
            } else if move_axis == 0.0 {
                sprite_render.sprite_number = 0;
                animated.time = 0.0;
            } else {
                animated.time += time.fixed_seconds();
                if animated.time > ANIMATION_DELAY {
                    animated.time -= ANIMATION_DELAY;
                    sprite_render.sprite_number = 1 - sprite_render.sprite_number;
                }
            }

            if move_axis != 0.0 {
                sprite_render.flip_horizontal = move_axis < 0.0;
            }
        }
    }
}