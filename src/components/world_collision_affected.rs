use amethyst::ecs::{Component, VecStorage};

pub struct WorldCollisionAffected {
    pub on_ground: bool,
    pub collides_left: bool,
    pub collides_right: bool,
    pub width: f32,
    pub height: f32,
}

impl WorldCollisionAffected {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            on_ground: false,
            collides_left: false,
            collides_right: false,
            width,
            height,
        }
    }
}

impl Default for WorldCollisionAffected {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Component for WorldCollisionAffected {
    type Storage = VecStorage<Self>;
}
