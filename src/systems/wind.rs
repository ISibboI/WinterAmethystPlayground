use amethyst::core::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use components::wind_affected::WindAffected;
use noise::{BasicMulti, MultiFractal, NoiseFn, Point3, Seedable};

pub struct WindSystem<T: NoiseFn<Point3<f64>>> {
    noise: T,
    wind_change_rate: f32,
    wind_speed: f32,
}

impl<T: NoiseFn<Point3<f64>> + Seedable> WindSystem<T> {
    pub fn new(noise: T, wind_change_rate: f32, wind_speed: f32) -> Self {
        Self {
            noise,
            wind_change_rate,
            wind_speed,
        }
    }
}

impl Default for WindSystem<BasicMulti> {
    fn default() -> Self {
        let noise = BasicMulti::new()
            .set_octaves(6)
            .set_frequency(0.02)
            .set_lacunarity(1.5)
            .set_persistence(0.7);
        Self::new(noise, 0.5, 0.1)
    }
}

impl<'s, T: NoiseFn<Point3<f64>>> System<'s> for WindSystem<T> {
    type SystemData = (
        ReadStorage<'s, WindAffected>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (wind_affecteds, mut transforms, time): <Self as System<'s>>::SystemData) {
        let z = time.absolute_time_seconds() * self.wind_change_rate as f64;

        for (wind_affected, mut transform) in (&wind_affecteds, &mut transforms).join() {
            let x = self.noise.get([
                transform.translation.x as f64,
                transform.translation.y as f64,
                z + wind_affected.time_offset as f64,
            ]) as f32
                * self.wind_speed
                * wind_affected.air_resistance;
            let y = self.noise.get([
                transform.translation.x as f64,
                transform.translation.y as f64,
                z + wind_affected.time_offset as f64 + 10000.0,
            ]) as f32
                * self.wind_speed
                * wind_affected.air_resistance;
            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}
