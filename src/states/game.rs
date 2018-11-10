use amethyst::{
    assets::{AssetStorage, Completion, Loader, PrefabLoader, ProgressCounter, RonFormat},
    audio::AudioFormat,
    audio::SourceHandle,
    core::transform::Transform,
    ecs::Write,
    GameData,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
    shrev::EventChannel,
    SimpleState, StateData, ui::{UiCreator, UiFinder},
};
use components::*;
use entities::{Player, Snowflake};
use euclid::TypedPoint2D;
use euclid::TypedRect;
use euclid::TypedSize2D;
use events::{actions::EventAction, Event, GameEventPrefab, GameEvents, triggers::EventTrigger};
use resources::{dialogue::Dialogue, GameSpriteSheets, Ui};
use resources::level::Level;
use std::collections::HashMap;
use std::fs;

pub const VIEWPORT_WIDTH: f32 = 100.0;
pub const VIEWPORT_HEIGHT: f32 = 100.0;

#[derive(Default)]
pub struct GameState {
    progress: Option<ProgressCounter>,
    initialized: bool,
}

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.progress = Some(ProgressCounter::default());

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

        world.exec(|mut creator: UiCreator| creator.create("resources/ui/dialogue.ron", ()));
        world.exec(
            |(loader, mut store): (PrefabLoader<GameEventPrefab>, Write<GameEvents>)| {
                store.handle = Some(loader.load(
                    "resources/events.ron",
                    RonFormat,
                    (),
                    self.progress.as_mut().unwrap(),
                ));
            },
        );

        world.add_resource(Level {
            bounding_box: TypedRect::new(TypedPoint2D::new(0.0, 0.0), TypedSize2D::new(200.0, 100.0)),
        });

        initialize_background(world);
        initialize_player(world);
        initialize_camera(world);
    }

    fn update(
        &mut self,
        data: &mut StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if !self.initialized {
            self.initialized = match self.progress.as_ref().map(|p| p.complete()) {
                None | Some(Completion::Loading) => false,
                _ => {
                    let event_handle = data
                        .world
                        .read_resource::<GameEvents>()
                        .handle
                        .as_ref()
                        .unwrap()
                        .clone();
                    data.world.create_entity().with(event_handle).build();
                    info!("GameState initialized");
                    true
                }
            }
        }

        Trans::None
    }
}

fn initialize_background(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.x = VIEWPORT_WIDTH / 2.0 + 50.0;
    transform.translation.y = VIEWPORT_HEIGHT / 2.0;
    ;
    transform.translation.z = -1.0;
    transform.scale *= 0.5;

    let sprite_render = SpriteRender {
        sprite_sheet: world.read_resource::<GameSpriteSheets>().background(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build();
}

fn initialize_player(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.x = VIEWPORT_WIDTH / 2.0;
    transform.translation.y = VIEWPORT_HEIGHT / 2.0;
    transform.scale *= 0.5;

    let sprite_sheet = world.read_resource::<GameSpriteSheets>().santa();
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    world
        .create_entity()
        .with(Player::new(30.0, 100.0))
        .with(GravityAffected::new(110.0))
        .with(Velocity::default())
        .with(sprite_render)
        .with(WorldCollisionAffected::new(
            36.0 * transform.scale.x,
            51.0 * transform.scale.y,
        ))
        .with(transform)
        .with(Transparent)
        .with(Animated::default())
        //.with(WindGenerator::new(18.0, 25.5, 0.3))
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.z = 1.0;
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            VIEWPORT_WIDTH,
            VIEWPORT_HEIGHT,
            0.0,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheets(world: &mut World) {
    let mut sprite_sheets = GameSpriteSheets::default();
    sprite_sheets.set_santa(load_texture(world, "santa", 0));
    sprite_sheets.set_snowflake(load_texture(world, "snowflake", 1));
    sprite_sheets.set_ground(load_texture(world, "ground", 2));
    sprite_sheets.set_background(load_texture(world, "background", 3));
    world.add_resource(sprite_sheets);
}

fn load_texture(world: &mut World, name: &str, texture_id: u64) -> SpriteSheetHandle {
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("resources/texture/{}_spritesheet.png", name),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    // `texture_id` is a application defined ID given to the texture to store in
    // the `World`. This is needed to link the texture to the sprite_sheet.
    let mut material_texture_set = world.write_resource::<MaterialTextureSet>();
    material_texture_set.insert(texture_id, texture_handle);

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("resources/texture/{}_spritesheet.ron", name), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_id, // We pass it the ID of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn load_audio(world: &mut World) {
    debug!("Loading audio");
    let mut audio_map = HashMap::new();
    let dir = fs::read_dir("resources/speech/").expect("Could not read speech dir");
    for file in dir {
        debug!("Found DirEntry {:?}", &file);
        let file = file.expect("Could not read file");
        let path = file.path();
        let path_str = path.to_str().expect("Could not convert path to string");
        if path.is_file() && path_str.ends_with(".ogg") {
            debug!("Loading sound {:?}", &path);
            let source_handle = load_audio_track(world, path_str);
            let file_stem = path.file_stem().expect("File does not have stem").to_str().expect("Cannot convert from OsString to String").to_owned();
            info!("Loaded sound {}", &file_stem);
            audio_map.insert(file_stem, source_handle);
        }
    }
    world.add_resource(audio_map);
    world.add_resource(Music);
}

/// Loads an ogg audio track.
fn load_audio_track<N: Into<String>>(world: &World, name: N) -> SourceHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(name, AudioFormat::Ogg, (), (), &world.read_resource())
}

pub struct Music;