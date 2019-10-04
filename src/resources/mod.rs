use amethyst::{ecs::Entity, renderer::sprite::SpriteSheetHandle};

pub mod dialogue;
pub mod level;

#[derive(Default, Clone)]
pub struct GameSpriteSheets {
    santa: Option<SpriteSheetHandle>,
    snowflake: Option<SpriteSheetHandle>,
    ground: Option<SpriteSheetHandle>,
    background: Option<SpriteSheetHandle>,
}

impl GameSpriteSheets {
    pub fn santa(&self) -> SpriteSheetHandle {
        self.santa.clone().unwrap()
    }

    pub fn snowflake(&self) -> SpriteSheetHandle {
        self.snowflake.clone().unwrap()
    }

    pub fn ground(&self) -> SpriteSheetHandle {
        self.ground.clone().unwrap()
    }

    pub fn background(&self) -> SpriteSheetHandle {
        self.background.clone().unwrap()
    }

    pub fn set_santa(&mut self, santa: SpriteSheetHandle) {
        self.santa = Some(santa);
    }

    pub fn set_snowflake(&mut self, snowflake: SpriteSheetHandle) {
        self.snowflake = Some(snowflake);
    }

    pub fn set_ground(&mut self, ground: SpriteSheetHandle) {
        self.ground = Some(ground);
    }

    pub fn set_background(&mut self, background: SpriteSheetHandle) {
        self.background = Some(background);
    }
}

#[derive(Default)]
pub struct Ui {
    pub dialogue_text: Option<Entity>,
}
