use amethyst::{
    core::{math::base::Vector3, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::Camera,
};
use euclid::TypedSize2D;

use entities::Player;
use levels::{Level, LevelStore};
use states::game::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

const DEAD_ZONE: f32 = 10.0;

#[derive(Default)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, LevelStore>,
    );

    fn run(&mut self, (players, mut transforms, cameras, level): <Self as System<'s>>::SystemData) {
        let level = level.get_current_level();
        let (_, transform) = (&players, &transforms).join().next().unwrap();
        let player_translation = transform.translation().clone();

        for (_camera, transform) in (&cameras, &mut transforms).join() {
            let camera_translation = transform.translation_mut();
            let mut difference = player_translation - *camera_translation;
            difference.z = 0.0;
            difference -= Vector3::new(0.0, 0.0, 0.0);
            dead_zone(&mut difference.x);
            dead_zone(&mut difference.y);
            *camera_translation += 0.1 * difference;

            let mut camera_rect = level.bounding_box().clone();
            camera_rect.decrease_size_clamped(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
            clamp(
                &mut camera_translation.x,
                camera_rect.min_x() + VIEWPORT_WIDTH / 2.0,
                camera_rect.max_x() + VIEWPORT_WIDTH / 2.0,
            );
            clamp(
                &mut camera_translation.y,
                camera_rect.min_y() + VIEWPORT_HEIGHT / 2.0,
                camera_rect.max_y() + VIEWPORT_HEIGHT / 2.0,
            );
        }
    }
}

fn dead_zone(x: &mut f32) {
    let signum = x.signum();
    *x -= DEAD_ZONE * signum;
    if x.signum() != signum {
        *x = 0.0;
    }
}

fn clamp(x: &mut f32, min: f32, max: f32) {
    *x = x.min(max).max(min);
}
