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
    pub triggers: Vec<EventTrigger>,
    pub actions: Vec<EventAction>,
}

impl Component for Event {
    type Storage = DenseVecStorage<Self>;
}

pub type EventHandle = Option<Handle<Prefab<Event>>>;

impl<'a> PrefabData<'a> for Event {
    type SystemData = (WriteStorage<'a, Event>);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        events: &mut <Self as PrefabData<'a>>::SystemData,
        _entities: &[Entity],
    ) -> Result<<Self as PrefabData<'a>>::Result, Error> {
        events
            .insert(entity, self.clone())
            .expect("Could not insert event");
        info!("Loaded event prefab");
        Ok(())
    }
}
