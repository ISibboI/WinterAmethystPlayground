use amethyst::core::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use components::{WindAffected, WindGenerator, Velocity};
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
        Self::new(noise, 0.5, 3.0)
    }
}

impl<'s, T: NoiseFn<Point3<f64>>> System<'s> for WindSystem<T> {
    type SystemData = (
        ReadStorage<'s, WindAffected>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, WindGenerator>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (wind_affecteds, mut transforms, wind_generators, velocities, time): <Self as System<'s>>::SystemData) {
        let mut generators = Vec::new();
        for (wind_generator, velocity, transform) in (&wind_generators, &velocities, &transforms).join() {
            generators.push((wind_generator.clone(), velocity.clone(), transform.clone()));
        }

        let z = time.frame_number() as f64 * time.fixed_seconds() as f64 * self.wind_change_rate as f64;

        for (wind_affected, mut transform) in (&wind_affecteds, &mut transforms).join() {
            let mut x = self.noise.get([
                transform.translation.x as f64,
                transform.translation.y as f64,
                z + wind_affected.time_offset as f64,
            ]) as f32
                * self.wind_speed
                * wind_affected.air_resistance
                * time.fixed_seconds();
            let mut y = self.noise.get([
                transform.translation.x as f64,
                transform.translation.y as f64,
                z + wind_affected.time_offset as f64 + 10000.0,
            ]) as f32
                * self.wind_speed
                * wind_affected.air_resistance
                * time.fixed_seconds();

            for (wind_generator, velocity, wind_transform) in &generators {
                let mut gen_x = transform.translation.x - wind_transform.translation.x;
                let mut gen_y = transform.translation.y - wind_transform.translation.y;
                gen_x /= wind_generator.width();
                gen_y /= wind_generator.height();
                let power = (1.0 - gen_x.abs().max(gen_y.abs())).max(0.0) * wind_generator.speed();
                x += velocity.velocity.x * power * time.fixed_seconds();
                y += velocity.velocity.y * power * time.fixed_seconds();
            }

            transform.translation.x += x;
            transform.translation.y += y;
        }
    }
}
