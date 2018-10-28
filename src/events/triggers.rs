#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventTrigger {
    Timed(f32),
}
