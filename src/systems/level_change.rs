use levels::{LevelChange, LevelStore};
use amethyst::ecs::{Read, Write, System, World, ReadStorage, WriteStorage, Join, Entities, LazyUpdate};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::core::Transform;
use amethyst::renderer::SpriteRender;
use entities::Player;
use components::WorldBackground;
use states::game::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT};
use resources::GameSpriteSheets;

#[derive(Default)]
pub struct LevelChangeSystem {
    reader: Option<ReaderId<LevelChange>>,
}

impl<'s> System<'s> for LevelChangeSystem {
    type SystemData = (
        Write<'s, EventChannel<LevelChange>>,
        Write<'s, LevelStore>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        WriteStorage<'s, WorldBackground>,
        Read<'s, LazyUpdate>,
        Read<'s, GameSpriteSheets>,
    );

    fn run(&mut self, (mut level_changes, mut level_store, players, mut transforms, entities, mut world_backgrounds, lazy_updater, game_sprite_sheets): <Self as System<'s>>::SystemData) {
        if self.reader.is_none() {
            self.reader = Some(
                level_changes.register_reader(),
            );
        }

        let reader = self.reader.as_mut().unwrap();
        let reader = level_changes.read(reader);
        let mut last_level_change = None;
        for level_change in reader {
            last_level_change = Some(level_change.clone());
        }
        if let Some(level_change) = last_level_change {
            debug!("Found level change: {:?}", level_change);
            level_store.change_level(&level_change);
            let starting_location = level_store.get_current_level().entry_point(&level_change.entry_point);
            for (_, transform) in (&players, &mut transforms).join() {
                transform.translation_mut().x = starting_location.x;
                transform.translation_mut().y = starting_location.y;
            }
            for (entity, _) in (&entities, &world_backgrounds).join() {
                entities.delete(entity).unwrap();
            }
            update_background(&game_sprite_sheets, &entities, &lazy_updater);
        }
    }
}

fn update_background<'s>(game_sprite_sheets: &Read<'s, GameSpriteSheets>, entities: &Entities<'s>, lazy_updater: &Read<'s, LazyUpdate>) {
    let mut transform = Transform::default();
    transform.translation_mut().x = VIEWPORT_WIDTH / 2.0 + 100.0;
    transform.translation_mut().y = VIEWPORT_HEIGHT / 2.0;
    transform.translation_mut().z = -1.0;

    let sprite_render = SpriteRender {
        sprite_sheet: game_sprite_sheets.background_outside(),
        sprite_number: 0,
    };

    let background = entities.create();
    lazy_updater.insert(background, transform);
    lazy_updater.insert(background, sprite_render);
    lazy_updater.insert(background, WorldBackground);
}