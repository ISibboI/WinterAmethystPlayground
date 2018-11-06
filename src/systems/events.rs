use amethyst::{
    core::Time,
    core::Transform,
    ecs::{Component, Entities, Join, Read, ReadStorage, Resources, System, Write, WriteStorage},
    input::InputHandler,
    shrev::EventChannel,
};
use entities::Player;
use events::{actions::EventAction, Event, triggers::EventTrigger};
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
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Write<'s, EventChannel<Dialogue>>,
    );

    fn run(
        &mut self,
        (entities, events, time, players, transforms, input_handler, mut dialogue_actions): <Self as System<'s>>::SystemData,
    ) {
        let current_time = time.frame_number() as f32 * time.fixed_seconds();
        let (player, _) = (&entities, &players).join().next().unwrap();

        for (entity, event) in (&entities, &events).join() {
            let mut is_triggered = true;
            for trigger in &event.triggers {
                match trigger {
                    EventTrigger::Timed(time) => {
                        if current_time < *time {
                            is_triggered = false;
                        }
                    }
                    EventTrigger::Area(area) => {
                        if !area.contains(&transforms.get(player).unwrap()) {
                            is_triggered = false;
                        }
                    }
                    EventTrigger::ActionKey => {
                        if !input_handler.action_is_down("action").unwrap() {
                            is_triggered = false;
                        }
                    }
                }
            }

            if is_triggered {
                entities.delete(entity).unwrap();
                self.actions.extend(event.actions.iter().cloned());
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
