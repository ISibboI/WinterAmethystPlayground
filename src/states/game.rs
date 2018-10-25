use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
use amethyst::GameData;
use amethyst::SimpleState;
use amethyst::StateData;
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;
use components::wind_affected::WindAffected;
use entities::snowflake::Snowflake;
use states::paddle::Paddle;
use states::paddle::Side;
use states::paddle::PADDLE_WIDTH;
use resources::GameSpriteSheets;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub struct GameState;

impl<'a, 'b> SimpleState<'a, 'b> for GameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheets = load_sprite_sheets(world);
        world.add_resource(sprite_sheets.clone());

        world.register::<Paddle>();
        world.register::<Snowflake>();
        world.register::<GravityAffected>();
        world.register::<Velocity>();
        world.register::<WindAffected>();

        initialise_paddles(world, sprite_sheets.pong());
        initialize_camera(world);
    }
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

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the paddles
    let sprite_render_left = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        flip_horizontal: false,
        flip_vertical: false,
    };

    let sprite_render_right = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
        flip_horizontal: true,
        flip_vertical: false,
    };

    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = ARENA_HEIGHT / 2.0;
    left_transform.translation = Vector3::new(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.translation = Vector3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with(sprite_render_left)
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(sprite_render_right)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

fn load_sprite_sheets(world: &mut World) -> GameSpriteSheets {
    let mut sprite_sheets = GameSpriteSheets::default();
    sprite_sheets.pong = Some(load_texture(world, "pong", 0));
    sprite_sheets.snowflake = Some(load_texture(world, "snowflake", 1));
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