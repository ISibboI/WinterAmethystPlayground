use amethyst::{
    core::{Time, Transform},
    ecs::{Component, Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};
use entities::Player;
use events::{actions::EventAction, triggers::EventTrigger, Event};
use resources::dialogue::Dialogue;
use levels::LevelChange;

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
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<Dialogue>>,
        Write<'s, EventChannel<LevelChange>>,
    );

    fn run(
        &mut self,
        (entities, events, time, players, transforms, input_handler, mut dialogue_actions, mut level_change_actions): <Self as System<'s>>::SystemData,
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
                debug!("Event triggered: {:?}", event);
                entities.delete(entity).unwrap();
                self.actions.extend(event.actions.iter().cloned());
            }
        }

        if self.first_run {
            self.first_run = false;
        } else {
            for action in &self.actions {
                match action.clone() {
                    EventAction::Dialogue(dialogue) => {
                        debug!("Queueing dialogue action: {:?}", dialogue);
                        dialogue_actions.single_write(dialogue);
                    },
                    EventAction::ChangeLevel(level_change) => {
                        debug!("Queueing level change action: {:?}", level_change);
                        level_change_actions.single_write(level_change);
                    },
                }
            }
            self.actions.clear();
        }
    }
}
