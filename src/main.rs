extern crate amethyst;
extern crate rand;
//extern crate noise;

mod components;
mod entities;
mod states;
mod resources;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage,
        ALPHA,
    },
};
use states::game::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.01, 0.01, 0.01, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    let binding_path = "./resources/bindings_config.ron";
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?
        .with_bundle(input_bundle)?
        //.with(systems::WindSystem, "wind_system", &["transform_system"])
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::SnowflakeSystem::new(), "snowflake_system", &[])
        .with(systems::GravitySystem, "gravity_system", &[])
        .with(
            systems::MovementSystem,
            "movement_system",
            &["gravity_system"],
        );
    let mut game = Application::new("./", Pong, game_data)?;
    game.run();

    Ok(())
}
