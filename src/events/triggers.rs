use amethyst::core::Transform;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventTrigger {
    Timed(f32),
    Area(Area),
    ActionKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Area {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl Area {
    pub fn contains(&self, t: &Transform) -> bool {
        let x = t.translation.x;
        let y = t.translation.y;
        self.x1 <= x && x <= self.x2 && self.y1 <= y && y <= self.y2
    }
}
