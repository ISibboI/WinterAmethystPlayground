use amethyst::renderer::SpriteSheetHandle;

#[derive(Default, Clone)]
pub struct GameSpriteSheets {
    pub pong: Option<SpriteSheetHandle>,
    pub snowflake: Option<SpriteSheetHandle>,
}

impl GameSpriteSheets {
    pub fn pong(&self) -> SpriteSheetHandle {
        self.pong.clone().unwrap()
    }

    pub fn snowflake(&self) -> SpriteSheetHandle {
        self.snowflake.clone().unwrap()
    }
}
