use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent},
    GameData, SimpleState, StateData,
};

use components::*;
use entities::{Player, Snowflake};
use resources::GameSpriteSheets;

#[allow(dead_code)]
pub struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
    }
}
