use amethyst::{
    core::Time,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};
use components::{Animated, WorldCollisionAffected};
use systems::dialogue::InDialogue;

const ANIMATION_DELAY: f32 = 0.2;

#[derive(Default)]
pub struct AnimationSystem {
    last_dialogue: bool,
    last_on_ground: bool,
}

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        WriteStorage<'s, Animated>,
        ReadStorage<'s, WorldCollisionAffected>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InDialogue>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (
            mut animateds,
            world_collision_affecteds,
            mut sprite_renders,
            in_dialogue,
            time,
            input_handler,
        ): <Self as System<'s>>::SystemData,
    ) {
        let in_dialogue = in_dialogue.in_dialogue;
        for (mut animated, world_collision_affected, mut sprite_render) in (
            &mut animateds,
            &world_collision_affecteds,
            &mut sprite_renders,
        )
            .join()
        {
            let move_axis = input_handler.axis_value("move").unwrap();
            if in_dialogue {
                if !self.last_dialogue {
                    animated.time = 0.0;
                    sprite_render.sprite_number = 2;
                } else {
                    if animated.time > ANIMATION_DELAY {
                        animated.time -= ANIMATION_DELAY;
                        sprite_render.sprite_number = 2 - sprite_render.sprite_number;
                    }
                    animated.time += time.fixed_seconds();
                }
            } else {
                if self.last_dialogue {
                    animated.time = 0.0;
                    sprite_render.sprite_number = if move_axis == 0.0 { 0 } else { 1 };
                }
                if !world_collision_affected.on_ground {
                    sprite_render.sprite_number = 1;
                    animated.time = 0.0;
                } else {
                    if !self.last_on_ground {
                        sprite_render.sprite_number = 0;
                        animated.time = 0.0;
                    }

                    if move_axis == 0.0 {
                        sprite_render.sprite_number = 0;
                        animated.time = 0.0;
                    } else {
                        if animated.time > ANIMATION_DELAY {
                            animated.time -= ANIMATION_DELAY;
                            sprite_render.sprite_number = 1 - sprite_render.sprite_number;
                        }
                        animated.time += time.fixed_seconds();
                    }
                }

                if move_axis != 0.0 {
                    sprite_render.flip_horizontal = move_axis < 0.0;
                }
            }
            self.last_dialogue = in_dialogue;
            self.last_on_ground = world_collision_affected.on_ground;
        }
    }
}
