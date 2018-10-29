use amethyst::ecs::{Component, DenseVecStorage};
//use amethyst::core::specs::storage::UnprotectedStorage;
use amethyst::assets::Asset;
use events::{actions::EventAction, triggers::EventTrigger};

pub mod actions;
pub mod triggers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub trigger: EventTrigger,
    pub action: EventAction,
}

impl Component for Event {
    type Storage = DenseVecStorage<Self>;
}

/*#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameEventList {
    pub events: Vec<Event>,
}

impl Asset for GameEventList {
    const NAME: &'static str = "game_event_list";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Self>;
}*/