use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle, Transparent};
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;
use entities::snowflake::Snowflake;
use pong::ARENA_HEIGHT;
use pong::ARENA_WIDTH;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::distributions::Uniform;
use resources::SpriteSheets;
use components::wind_affected::WindAffected;
use amethyst::core::Time;
use noise::Point3;
use noise::OpenSimplex;
use noise::Seedable;

pub struct WindSystem<T: NoiseFn<Point3<f64>>> {
    noise: T,
};

impl<T: NoiseFn<Point3<f64>> + Seedable> WindSystem<T> {
    pub fn new(noise: T) -> Self {
        Self {
            noise,
        }
    }
}

impl<T: NoiseFn<Point3<f64>> + Default> Default for WindSystem<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<'s, T: NoiseFn<Point3<f64>>> System<'s> for WindSystem<T> {
    type SystemData = (ReadStorage<'s, WindAffected>, WriteStorage<'s, Transform>, Read<'s, Time>);

    fn run(&mut self, (wind_affecteds, mut transforms, time): <Self as System<'s>>::SystemData) {

    }
}