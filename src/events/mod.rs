use amethyst::{assets::{Handle, Prefab, PrefabData, ProgressCounter}, ecs::{Component, DenseVecStorage, Entity, WriteStorage}, Error};

use events::{actions::EventAction, triggers::EventTrigger};

pub mod actions;
pub mod triggers;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Event {
    pub triggers: Vec<EventTrigger>,
    pub actions: Vec<EventAction>,
}

impl Component for Event {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> PrefabData<'a> for Event {
    type SystemData = (WriteStorage<'a, Event>);
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        events: &mut <Self as PrefabData<'a>>::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<<Self as PrefabData<'a>>::Result, Error> {
        events
            .insert(entity, self.clone())
            .expect("Could not insert event");
        info!("Loaded event prefab");
        Ok(())
    }
}
