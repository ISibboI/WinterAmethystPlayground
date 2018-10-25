use amethyst::core::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System};
use amethyst::renderer::{SpriteRender, Transparent};
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;
use components::wind_affected::WindAffected;
use entities::snowflake::Snowflake;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use resources::GameSpriteSheets;
use states::game::ARENA_HEIGHT;
use states::game::ARENA_WIDTH;

const MAX_SNOWFLAKE_COUNT: usize = 200;
const SNOWFLAKE_RATE: f32 = 10.0;

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
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (entities, updater, sprite_sheets, snowflakes, transforms, time): <Self as System<'s>>::SystemData,
    ) {
        for (entity, _, transform) in (&entities, &snowflakes, &transforms).join() {
            if transform.translation.y < -3.0 {
                entities.delete(entity).expect("Could not delete snowflake");
                self.snowflake_count -= 1;
            }
        }

        self.partial_snowflake += time.delta_seconds() * SNOWFLAKE_RATE;
        while self.partial_snowflake >= 1.0 {
            if self.snowflake_count >= MAX_SNOWFLAKE_COUNT {
                self.partial_snowflake = 0.0;
                break;
            }

            self.spawn_snowflake(&entities, &updater, &sprite_sheets);
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
    ) {
        let snowflake = entities.create();
        updater.insert(snowflake, Snowflake::new());

        let mut transform = Transform::default();
        let rng = &mut rand::thread_rng();
        let translation_distribution = Uniform::new_inclusive(-5.0, ARENA_WIDTH + 5.0);
        transform.translation.x = translation_distribution.sample(rng);
        transform.translation.y = ARENA_HEIGHT + 10.0;
        let scale_distribution = Uniform::new_inclusive(0.5, 1.2);
        let scale = scale_distribution.sample(rng);
        transform.scale.x = 0.25 * scale;
        transform.scale.y = 0.25 * scale;
        updater.insert(snowflake, transform);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheets.snowflake(),
            sprite_number: 0,
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
