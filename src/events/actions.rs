use resources::dialogue::Dialogue;
use amethyst::core::math::base::Vector2;
use levels::LevelChange;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventAction {
    Dialogue(Dialogue),
    ChangeLevel(LevelChange),
}
