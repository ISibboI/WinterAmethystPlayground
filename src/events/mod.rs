use amethyst::{
    assets::{Handle, Prefab, PrefabData, PrefabError, ProgressCounter},
    core::specs::error::Error,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
};
use events::{actions::EventAction, triggers::EventTrigger};

pub mod actions;
pub mod triggers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub trigger: EventTrigger,
    pub actions: Vec<EventAction>,
}

impl Component for Event {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GameEventPrefab {
    event: Option<Event>,
}

#[derive(Default)]
pub struct GameEvents {
    pub handle: Option<Handle<Prefab<GameEventPrefab>>>,
}

impl<'a> PrefabData<'a> for GameEventPrefab {
    type SystemData = (WriteStorage<'a, Event>);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        events: &mut <Self as PrefabData<'a>>::SystemData,
        entities: &[Entity],
    ) -> Result<<Self as PrefabData<'a>>::Result, Error> {
        events
            .insert(entity, self.event.clone().unwrap())
            .expect("Could not insert event");
        info!("Loaded event prefab");
        Ok(())
    }
}
