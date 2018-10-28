use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
};
use amethyst::shrev::EventChannel;
use amethyst::ui::{UiCreator, UiFinder};
use amethyst::GameData;
use amethyst::SimpleState;
use amethyst::StateData;
use components::*;
use entities::Player;
use entities::Snowflake;
use events::actions::EventAction;
use events::triggers::EventTrigger;
use events::Event;
use resources::dialogue::Dialogue;
use resources::{GameSpriteSheets, Ui};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheets = load_sprite_sheets(world);
        world.add_resource(sprite_sheets.clone());

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

        initialize_background(world);
        initialize_player(world);
        initialize_camera(world);

        let event = Event {
            trigger: EventTrigger::Timed(1.0),
            action: EventAction::Dialogue(Dialogue {
                text: "Hello! I'm Santa.".to_owned(),
            }),
        };
        world.create_entity().with(event).build();
    }
}

fn initialize_background(world: &mut World) {
    let mut transform = Transform::default();
    transform.translation.x = ARENA_WIDTH / 2.0;
    transform.translation.y = ARENA_HEIGHT / 2.0;;
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
    transform.translation.x = ARENA_WIDTH / 2.0;
    transform.translation.y = ARENA_HEIGHT / 2.0;
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
        .with(Player::new(100.0, 150.0))
        .with(GravityAffected::new(100.0))
        .with(Velocity::default())
        .with(transform.clone())
        .with(sprite_render)
        .with(WorldCollisionAffected::new(
            36.0 * transform.scale.x,
            51.0 * transform.scale.y,
        ))
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
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheets(world: &mut World) -> GameSpriteSheets {
    let mut sprite_sheets = GameSpriteSheets::default();
    sprite_sheets.set_santa(load_texture(world, "santa", 0));
    sprite_sheets.set_snowflake(load_texture(world, "snowflake", 1));
    sprite_sheets.set_ground(load_texture(world, "ground", 2));
    sprite_sheets.set_background(load_texture(world, "background", 3));
    sprite_sheets
}

fn load_texture(world: &mut World, name: &str, texture_id: u64) -> SpriteSheetHandle {
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}_spritesheet.png", name),
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
        format!("texture/{}_spritesheet.ron", name), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_id, // We pass it the ID of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}
