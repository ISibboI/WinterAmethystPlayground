use std::collections::{HashMap, VecDeque};

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source, SourceHandle},
    core::Time,
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, World, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
    ui::{FontHandle, UiFinder, UiText},
};

use components::{Animated, WorldCollisionAffected};
use resources::{dialogue::Dialogue, Ui};
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
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, EventChannel<Dialogue>>,
        Read<'s, HashMap<String, SourceHandle>>,
        Read<'s, AssetStorage<Source>>,
        Option<Read<'s, Output>>,
        WriteStorage<'s, UiText>,
        Write<'s, InDialogue>,
        UiFinder<'s>,
    );

    fn run(
        &mut self,
        (
            input_handler,
            dialogues,
            sound_effects,
            audio_sources,
            audio_output,
            mut ui_texts,
            mut in_dialogue,
            ui_finder,
        ): <Self as System<'s>>::SystemData,
    ) {
        let reader = self.reader.as_mut().unwrap();
        let reader = dialogues.read(reader);
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
                    // ui_finder.find("dialogue_box").unwrap(); // TODO unhide box
                    debug!("Showing dialogue: {}", dialogue.text);
                    if let (Some(sound_name), Some(output)) =
                        (dialogue.sound.as_ref(), audio_output)
                    {
                        if let Some(sound) = sound_effects.get(sound_name) {
                            if let Some(sound) = audio_sources.get(sound) {
                                debug!("Playing sound: {}", sound_name);
                                output.play_once(sound, 1.0);
                            } else {
                                warn!("Sound not loaded: {}", sound_name);
                            }
                        } else {
                            warn!("Sound not registered: {}", sound_name);
                        }
                    }
                } else {
                    // TODO hide box
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

    fn setup(&mut self, world: &mut World) {
        self.reader = Some(
            world
                .get_mut::<EventChannel<Dialogue>>()
                .unwrap()
                .register_reader(),
        );
        //self.reader = Some(Write::<EventChannel<Dialogue>>::fetch(world).register_reader());
        world.insert::<InDialogue>(InDialogue { in_dialogue: false });
    }
}

#[derive(Default)]
pub struct InDialogue {
    pub in_dialogue: bool,
}
