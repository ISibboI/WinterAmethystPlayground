use amethyst::{
    core::{specs::SystemData, Time},
    ecs::{Entities, Entity, Join, Read, ReadStorage, Resources, System, Write, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
    ui::{FontHandle, UiFinder, UiText},
};
use components::{Animated, WorldCollisionAffected};
use resources::{dialogue::Dialogue, Ui};
use std::collections::VecDeque;

#[derive(Default)]
pub struct DialogueSystem {
    reader: Option<ReaderId<Dialogue>>,
    dialogue_text: Option<Entity>,
}

impl<'s> System<'s> for DialogueSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, EventChannel<Dialogue>>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
    );

    fn run(
        &mut self,
        (entities, input_handler, dialogues, mut ui_texts, ui_finder): <Self as System<'s>>::SystemData,
    ) {
        if let Some(dialogue_text) = self.dialogue_text {
            let dialogue_text = &mut ui_texts.get_mut(dialogue_text).unwrap();
            if dialogue_text.text == "" {
                let reader = self.reader.as_mut().unwrap();
                let mut reader = dialogues.read(reader);
                if let Some(dialogue) = reader.next() {
                    dialogue_text.text = dialogue.text.clone();
                }
            }
        } else {
            self.dialogue_text = ui_finder.find("dialogue");
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        self.reader = Some(Write::<EventChannel<Dialogue>>::fetch(res).register_reader());
    }
}
