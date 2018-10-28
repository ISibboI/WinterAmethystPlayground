use amethyst::ecs::{Component, DenseVecStorage};
use events::actions::EventAction;
use events::triggers::EventTrigger;

pub mod actions;
pub mod triggers;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub trigger: EventTrigger,
    pub action: EventAction,
}

impl Component for Event {
    type Storage = DenseVecStorage<Self>;
}
