use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System};
use amethyst::renderer::{SpriteRender, Transparent};
use components::gravity_affected::GravityAffected;
use components::velocity::Velocity;
use components::wind_affected::WindAffected;
use entities::snowflake::Snowflake;
use states::game::ARENA_HEIGHT;
use states::game::ARENA_WIDTH;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use resources::SpriteSheets;

pub struct SnowflakeSystem {
    snowflake_count: usize,
}

impl SnowflakeSystem {
    pub fn new() -> Self {
        Self { snowflake_count: 0 }
    }
}

impl<'s> System<'s> for SnowflakeSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, SpriteSheets>,
        ReadStorage<'s, Snowflake>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, updater, sprite_sheets, snowflakes, transforms): <Self as System<'s>>::SystemData,
    ) {
        for (entity, _, transform) in (&entities, &snowflakes, &transforms).join() {
            if transform.translation.y < -3.0 {
                entities.delete(entity).expect("Could not delete snowflake");
                self.snowflake_count -= 1;
            }
        }

        if self.snowflake_count < 10000 {
            let snowflake = entities.create();
            updater.insert(snowflake, Snowflake::new());

            let mut transform = Transform::default();
            let mut rng = rand::thread_rng();
            let translation_distribution = Uniform::new_inclusive(0.0, ARENA_WIDTH);
            transform.translation.x = translation_distribution.sample(&mut rng);
            transform.translation.y = ARENA_HEIGHT;
            transform.scale.x = 0.5;
            transform.scale.y = 0.5;
            updater.insert(snowflake, transform);

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheets.pong.clone().unwrap(),
                sprite_number: 2,
                flip_horizontal: false,
                flip_vertical: false,
            };
            updater.insert(snowflake, sprite_render);
            updater.insert(snowflake, GravityAffected::new(2.0));
            updater.insert(snowflake, Velocity::new(0.0, 0.0));
            updater.insert(snowflake, Transparent);
            updater.insert(snowflake, WindAffected::new(1.0));
            self.snowflake_count += 1;
        }
    }
}
