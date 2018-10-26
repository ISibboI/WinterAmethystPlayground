use amethyst::ecs::{System, Join, Read, ReadStorage, WriteStorage};
use components::{Animated, WorldCollisionAffected};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;
use amethyst::core::Time;
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