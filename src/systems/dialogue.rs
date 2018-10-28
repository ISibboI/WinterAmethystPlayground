use amethyst::core::Time;
use amethyst::ecs::{Entities, Entity, Join, Read, ReadStorage, Resources, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{FontHandle, UiText};
use components::{Animated, WorldCollisionAffected};
use resources::dialogue::Dialogue;
use std::collections::VecDeque;

#[derive(Default)]
pub struct DialogueSystem {
    reader: Option<ReaderId<Dialogue>>,
    current_dialogue: Option<UiText>,
}

impl<'s> System<'s> for DialogueSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, EventChannel<Dialogue>>,
        WriteStorage<'s, UiText>,
    );

    // TODO Make this system handle the positioning of the dialogue box. The dialogue should be created by some kind of event system.
    fn run(
        &mut self,
        (entities, input_handler, dialogues, mut ui_texts): <Self as System<'s>>::SystemData,
    ) {
        let reader = self.reader.as_mut().unwrap();

        if self.current_dialogue.as_ref().unwrap().text == "" {}
    }

    fn setup(&mut self, res: &mut Resources) {
        self.reader = Some(res.fetch_mut::<EventChannel<Dialogue>>().register_reader());

        for ui_text in &res.fetch_mut::<UiText>() {}
    }
}
