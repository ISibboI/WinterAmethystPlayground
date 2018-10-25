/*use amethyst::core::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, WriteStorage};
use components::wind_affected::WindAffected;
use noise::{Point3, OpenSimplex, Seedable, NoiseFn};

pub struct WindSystem<T: NoiseFn<Point3<f64>>> {
    noise: T,
}

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
}*/
