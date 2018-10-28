use resources::dialogue::Dialogue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventAction {
    Dialogue(Dialogue),
}
