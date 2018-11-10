use amethyst::{
    core::{Time, Transform},
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System},
    renderer::{SpriteRender, Transparent},
};
use components::{GravityAffected, Velocity, WindAffected};
use entities::Snowflake;
use rand::distributions::{Distribution, Uniform};
use resources::{level::Level, GameSpriteSheets};
use states::game::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

const MAX_SNOWFLAKE_COUNT: usize = 200;
const SNOWFLAKE_RATE: f32 = 0.1;

pub struct SnowflakeSystem {
    snowflake_count: usize,
    partial_snowflake: f32,
}

impl SnowflakeSystem {
    pub fn new() -> Self {
        Self {
            snowflake_count: 0,
            partial_snowflake: 0.0,
        }
    }
}

impl<'s> System<'s> for SnowflakeSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, GameSpriteSheets>,
        ReadStorage<'s, Snowflake>,
        ReadStorage<'s, Transform>,
        Read<'s, Level>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, updater, sprite_sheets, snowflakes, transforms, level, time): <Self as System<
            's,
        >>::SystemData,
    ) {
        let rng = &mut rand::thread_rng();
        let deletion_distribution = Uniform::new(0, 15);

        for (entity, _, transform) in (&entities, &snowflakes, &transforms).join() {
            if transform.translation.y < -3.0
                || (transform.translation.y < 8.0 && deletion_distribution.sample(rng) == 0)
            {
                entities.delete(entity).expect("Could not delete snowflake");
                self.snowflake_count -= 1;
            }
        }

        self.partial_snowflake +=
            time.fixed_seconds() * SNOWFLAKE_RATE * level.bounding_box.size.width;
        while self.partial_snowflake >= 1.0 {
            if self.snowflake_count as f32
                >= MAX_SNOWFLAKE_COUNT as f32 / 10_000.0
                    * level.bounding_box.size.width
                    * level.bounding_box.size.height
            {
                self.partial_snowflake = 0.0;
                break;
            }

            self.spawn_snowflake(&entities, &updater, &sprite_sheets, &level);
            self.partial_snowflake -= 1.0;
        }
    }
}

impl<'s> SnowflakeSystem {
    fn spawn_snowflake(
        &mut self,
        entities: &Entities<'s>,
        updater: &Read<'s, LazyUpdate>,
        sprite_sheets: &Read<'s, GameSpriteSheets>,
        level: &Read<'s, Level>,
    ) {
        let snowflake = entities.create();
        updater.insert(snowflake, Snowflake::new());

        let mut transform = Transform::default();
        let rng = &mut rand::thread_rng();
        let sprite_number_distribution = Uniform::new(1, 7);
        let sprite_number = sprite_number_distribution.sample(rng);
        let translation_distribution = Uniform::new_inclusive(
            -5.0 + level.bounding_box.min_x(),
            level.bounding_box.max_x() + 5.0,
        );
        let z_distribution = Uniform::new_inclusive(-0.2, 0.5);
        transform.translation.x = translation_distribution.sample(rng);
        transform.translation.y = level.bounding_box.max_y() + 10.0;
        transform.translation.z = 0.25 - sprite_number as f32 * 0.1;
        transform.scale *= 0.5;
        updater.insert(snowflake, transform);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheets.snowflake(),
            sprite_number,
            flip_horizontal: false,
            flip_vertical: false,
        };
        let terminal_velocity_distribution = Uniform::new_inclusive(8.0, 12.0);
        let terminal_velocity = terminal_velocity_distribution.sample(rng);
        updater.insert(snowflake, sprite_render);
        updater.insert(snowflake, GravityAffected::new(terminal_velocity));
        updater.insert(snowflake, Velocity::new(0.0, 1.0 - terminal_velocity));
        updater.insert(snowflake, Transparent);

        let air_resistance_distribution = Uniform::new_inclusive(0.5, 2.0);
        let time_offset_distribution = Uniform::new_inclusive(-5.0, 5.0);
        updater.insert(
            snowflake,
            WindAffected::new(
                air_resistance_distribution.sample(rng),
                time_offset_distribution.sample(rng),
            ),
        );
        self.snowflake_count += 1;
    }
}
