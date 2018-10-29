use amethyst::{
    core::Time,
    ecs::{Component, Entities, Join, Read, ReadStorage, Resources, System, Write, WriteStorage},
    input::InputHandler,
    shrev::EventChannel,
};
use events::{actions::EventAction, triggers::EventTrigger, Event};
use resources::dialogue::Dialogue;

#[derive(Default)]
pub struct EventSystem {
    actions: Vec<EventAction>,
    first_run: bool,
}

impl<'s> System<'s> for EventSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Event>,
        Read<'s, Time>,
        Write<'s, EventChannel<Dialogue>>,
    );

    fn run(
        &mut self,
        (entities, events, time, mut dialogue_actions): <Self as System<'s>>::SystemData,
    ) {
        let current_time = time.frame_number() as f32 * time.fixed_seconds();

        for (entity, event) in (&entities, &events).join() {
            match &event.trigger {
                EventTrigger::Timed(time) => {
                    if current_time >= *time {
                        entities.delete(entity).unwrap();
                        self.actions.push(event.action.clone());
                    }
                }
            }
        }

        if self.first_run {
            self.first_run = false;
        } else {
            for action in &self.actions {
                match action.clone() {
                    EventAction::Dialogue(dialogue) => dialogue_actions.single_write(dialogue),
                }
            }
            self.actions.clear();
        }
    }
}
