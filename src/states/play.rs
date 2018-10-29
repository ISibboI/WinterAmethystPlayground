use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    },
    GameData, SimpleState, StateData,
};
use components::*;
use entities::{Player, Snowflake};
use resources::GameSpriteSheets;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub struct PlayState;

impl<'a, 'b> SimpleState<'a, 'b> for PlayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
    }
}
