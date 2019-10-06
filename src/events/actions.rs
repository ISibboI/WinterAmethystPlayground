use amethyst::core::math::base::Vector2;
use levels::LevelChange;
use resources::dialogue::Dialogue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventAction {
    Dialogue(Dialogue),
    ChangeLevel(LevelChange),
}
