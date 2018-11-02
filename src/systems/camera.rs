use amethyst::core::cgmath::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::renderer::Camera;
use entities::Player;
use euclid::TypedSize2D;
use resources::level::Level;
use states::game::VIEWPORT_HEIGHT;
use states::game::VIEWPORT_WIDTH;

const DEAD_ZONE: f32 = 10.0;

#[derive(Default)]
pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (ReadStorage<'s, Player>, WriteStorage<'s, Transform>, ReadStorage<'s, Camera>, Read<'s, Level>);

    fn run(&mut self, (players, mut transforms, cameras, level): <Self as System<'s>>::SystemData) {
        let (_, transform) = (&players, &transforms).join().next().unwrap();
        let player_translation = transform.translation.clone();

        for (camera, mut transform) in (&cameras, &mut transforms).join() {
            let camera_translation = &mut transform.translation;
            let mut difference = player_translation - *camera_translation;
            difference.z = 0.0;
            difference -= Vector3::new(50.0, 50.0, 0.0);
            dead_zone(&mut difference.x);
            dead_zone(&mut difference.y);
            *camera_translation += 0.1 * difference;

            let mut camera_rect = level.bounding_box.clone();
            camera_rect.size = camera_rect.size - TypedSize2D::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);
            clamp(&mut camera_translation.x, camera_rect.min_x(), camera_rect.max_x());
            clamp(&mut camera_translation.y, camera_rect.min_y(), camera_rect.max_y());
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