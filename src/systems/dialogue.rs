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
use systems::animation::AnimationSystem;

#[derive(Default)]
pub struct DialogueSystem {
    reader: Option<ReaderId<Dialogue>>,
    dialogue_queue: Vec<Dialogue>,
    dialogue_text: Option<Entity>,
    action_key_down: bool,
}

impl<'s> System<'s> for DialogueSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, EventChannel<Dialogue>>,
        WriteStorage<'s, UiText>,
        Write<'s, InDialogue>,
        UiFinder<'s>,
    );

    fn run(
        &mut self,
        (entities, input_handler, dialogues, mut ui_texts, mut in_dialogue, ui_finder): <Self as System<'s>>::SystemData,
    ) {
        let reader = self.reader.as_mut().unwrap();
        let mut reader = dialogues.read(reader);
        self.dialogue_queue.append(&mut reader.cloned().collect());

        let action_key_down = input_handler.action_is_down("action").unwrap();
        let replace_dialogue = !action_key_down && self.action_key_down;
        self.action_key_down = action_key_down;

        if let Some(dialogue_text) = self.dialogue_text {
            let dialogue_text = &mut ui_texts.get_mut(dialogue_text).unwrap();
            if dialogue_text.text == "" || replace_dialogue {
                if let Some(dialogue) = self.dialogue_queue.first() {
                    dialogue_text.text = format!("{}\nPress <F>", dialogue.text);
                    in_dialogue.in_dialogue = true;
                } else {
                    dialogue_text.text = "".to_owned();
                    in_dialogue.in_dialogue = false;
                }
                if self.dialogue_queue.len() > 0 {
                    self.dialogue_queue.remove(0);
                }
            }
        } else {
            self.dialogue_text = ui_finder.find("dialogue");
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        self.reader = Some(Write::<EventChannel<Dialogue>>::fetch(res).register_reader());
        res.insert::<InDialogue>(InDialogue {in_dialogue: false});
    }
}

#[derive(Default)]
pub struct InDialogue {
    pub in_dialogue: bool,
}