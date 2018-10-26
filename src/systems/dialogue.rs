use amethyst::core::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;
use components::{Animated, WorldCollisionAffected};
use std::collections::VecDeque;

pub struct DialogueSystem {
    dialogue_queue: VecDeque<String>,
}

impl<'s> System<'s> for DialogueSystem {
    type SystemData = (Read<'s, InputHandler<String, String>>);

    // TODO Make this system handle the positioning of the dialogue box. The dialogue should be created by some kind of event system.
    fn run(&mut self, (input_handler): <Self as System<'s>>::SystemData) {
        unimplemented!()
    }
}
