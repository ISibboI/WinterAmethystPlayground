use amethyst::renderer::SpriteSheetHandle;

#[derive(Default, Clone)]
pub struct GameSpriteSheets {
    santa: Option<SpriteSheetHandle>,
    snowflake: Option<SpriteSheetHandle>,
}

impl GameSpriteSheets {
    pub fn santa(&self) -> SpriteSheetHandle {
        self.santa.clone().unwrap()
    }

    pub fn snowflake(&self) -> SpriteSheetHandle {
        self.snowflake.clone().unwrap()
    }

    pub fn set_santa(&mut self, santa: SpriteSheetHandle) {
        self.santa = Some(santa);
    }

    pub fn set_snowflake(&mut self, snowflake: SpriteSheetHandle) {
        self.snowflake = Some(snowflake);
    }
}
