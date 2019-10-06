use amethyst::{ecs::Entity, renderer::sprite::SpriteSheetHandle};

pub mod dialogue;

#[derive(Default, Clone)]
pub struct GameSpriteSheets {
    santa: Option<SpriteSheetHandle>,
    snowflake: Option<SpriteSheetHandle>,
    ground: Option<SpriteSheetHandle>,
    background_outside: Option<SpriteSheetHandle>,
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

    pub fn background_outside(&self) -> SpriteSheetHandle {
        self.background_outside.clone().unwrap()
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

    pub fn set_background_outside(&mut self, background_outside: SpriteSheetHandle) {
        self.background_outside = Some(background_outside);
    }
}

#[derive(Default)]
pub struct Ui {
    pub dialogue_text: Option<Entity>,
}
