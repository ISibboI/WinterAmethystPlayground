use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    GameData,
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, Transparent,
    }, SimpleState, StateData,
};

use components::*;
use entities::{Player, Snowflake};
use resources::GameSpriteSheets;

pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
    }
}
