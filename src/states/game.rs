use std::{collections::HashMap, fs, path::Path};

use amethyst::{
    assets::{AssetStorage, Completion, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    audio::{OggFormat, SourceHandle},
    core::{math::base::Vector2, transform::Transform},
    ecs::Write,
    prelude::*,
    renderer::{
        sprite::SpriteSheetHandle, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture, Transparent,
    },
    ui::{UiCreator, UiFinder},
    GameData, SimpleState, StateData,
    shrev::EventChannel,
};
use euclid::{TypedPoint2D, TypedRect, TypedSize2D};

use components::*;
use entities::{Player, Snowflake};
use events::{actions::EventAction, triggers::EventTrigger, Event};
use geometry::Rectangle;
use levels::{Level, LevelConfig, LevelStore, LevelChange};
use resources::{dialogue::Dialogue, GameSpriteSheets, Ui};
use std::fs::File;

pub const VIEWPORT_WIDTH: f32 = 200.0;
pub const VIEWPORT_HEIGHT: f32 = 200.0;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        load_sprite_sheets(world);
        load_audio(world);

        world.register::<Snowflake>();
        world.register::<GravityAffected>();
        world.register::<Velocity>();
        world.register::<WindAffected>();
        world.register::<Player>();
        world.register::<WorldCollisionAffected>();
        world.register::<Animated>();
        world.register::<WindGenerator>();
        world.register::<Event>();

        world.exec(|mut creator: UiCreator| creator.create("ui/dialogue.ron", ()));
        let event_prefab_handle =
            world.exec(|loader: PrefabLoader<Event>| loader.load("events.ron", RonFormat, ()));
        world
            .create_entity()
            .with(event_prefab_handle.clone())
            .build();

        let levels_file = File::open("assets/levels.ron").unwrap();
        let level_config: LevelConfig = ron::de::from_reader(levels_file).unwrap();
        world.insert::<LevelStore>(level_config.into());

        initialize_background(world);
        initialize_player(world);
        initialize_camera(world);
    }
}

fn initialize_background(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation_mut().x = VIEWPORT_WIDTH / 2.0 + 100.0;
    transform.translation_mut().y = VIEWPORT_HEIGHT / 2.0;
    transform.translation_mut().z = -1.0;

    let sprite_render = SpriteRender {
        sprite_sheet: world.read_resource::<GameSpriteSheets>().background_outside(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .with(WorldBackground)
        .build();
}

fn initialize_player(world: &mut World) {
    let level_store = world.fetch::<LevelStore>();
    let current_level = level_store.get_current_level();
    let entry_point = current_level.entry_point("default");
    let mut transform = Transform::default();
    transform.translation_mut().x = entry_point.x;
    transform.translation_mut().y = entry_point.y;
    drop(level_store);

    let sprite_sheet = world.read_resource::<GameSpriteSheets>().santa();
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Player::new(30.0, 100.0))
        .with(GravityAffected::new(110.0))
        .with(Velocity::default())
        .with(sprite_render)
        .with(WorldCollisionAffected::new(36.0, 51.0))
        .with(transform)
        .with(Transparent)
        .with(Animated::default())
        //.with(WindGenerator::new(18.0, 25.5, 0.3))
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation_mut().z = 1.0;
    world
        .create_entity()
        .with(Camera::standard_2d(VIEWPORT_WIDTH, VIEWPORT_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheets(world: &mut World) {
    let mut sprite_sheets = GameSpriteSheets::default();
    sprite_sheets.set_santa(load_texture(world, "santa"));
    sprite_sheets.set_snowflake(load_texture(world, "snowflake"));
    sprite_sheets.set_ground(load_texture(world, "ground"));
    sprite_sheets.set_background_outside(load_texture(world, "background_outside"));
    world.insert(sprite_sheets);
}

fn load_texture(world: &mut World, name: &str) -> SpriteSheetHandle {
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}_spritesheet.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}_spritesheet.ron", name), // Here we load the associated ron file
        SpriteSheetFormat(texture_handle), // We pass it the ID of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn load_audio(world: &mut World) {
    debug!("Loading audio");
    let mut audio_map = HashMap::new();
    let dir = fs::read_dir("assets/speech/").expect("Could not read speech dir");
    for file in dir {
        debug!("Found DirEntry {:?}", &file);
        let file = file.expect("Could not read file");
        let path = Path::new("speech").join(file.file_name());
        let path_str = path.to_str().expect("Could not convert path to string");
        if path.is_file() && path_str.ends_with(".ogg") {
            debug!("Loading sound {:?}", &path);
            let source_handle = load_audio_track(world, path_str);
            let file_stem = path
                .file_stem()
                .expect("File does not have stem")
                .to_str()
                .expect("Cannot convert from OsString to String")
                .to_owned();
            info!("Loaded sound {}", &file_stem);
            audio_map.insert(file_stem, source_handle);
        }
    }
    world.insert(audio_map);
    world.insert(Music);
}

/// Loads an ogg audio track.
fn load_audio_track<N: Into<String>>(world: &World, name: N) -> SourceHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(name, OggFormat, (), &world.read_resource())
}

pub struct Music;
