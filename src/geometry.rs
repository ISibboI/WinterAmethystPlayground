

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Rectangle {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Rectangle {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Decreases the size of the rectangle along the x and y axis by moving the lower right corner.
    pub fn decrease_size(&mut self, x: f32, y: f32) {
        self.x2 -= x;
        self.y2 -= y;
        debug_assert!(self.is_valid());
    }

    pub fn is_valid(&self) -> bool {
        self.x1 <= self.x2 && self.y1 <= self.y2
    }

    pub fn min_x(&self) -> f32 {
        self.x1
    }

    pub fn max_x(&self) -> f32 {
        self.x2
    }

    pub fn min_y(&self) -> f32 {
        self.y1
    }

    pub fn max_y(&self) -> f32 {
        self.y2
    }

    pub fn width(&self) -> f32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> f32 {
        self.y2 - self.y1
    }
}