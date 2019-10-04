// Disable some warnings
#![allow(unused_imports)]

#[macro_use]
extern crate amethyst;
extern crate euclid;
#[macro_use]
extern crate log;
extern crate noise;
extern crate rand;
#[macro_use]
extern crate serde_derive;

use std::{
    fs,
    path::{Path, PathBuf},
};

use amethyst::{
    assets::PrefabLoaderSystemDesc,
    audio::{AudioBundle, SourceHandle},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    LoggerConfig,
    prelude::*,
    renderer::{RenderingBundle, RenderToWindow, RenderFlat2D, types::DefaultBackend},
    ui::{UiBundle, RenderUi},
    utils::application_root_dir,
};

use events::Event;
use states::game::{GameState, Music};

mod components;
mod entities;
mod events;
mod resources;
mod states;
mod systems;
mod geometry;

fn main() -> amethyst::Result<()> {
    let log_file = Path::new("debug.log");
    if log_file.is_file() {
        fs::remove_file(log_file)
            .unwrap_or_else(|error| warn!("Could not delete old log file: {}", error));
    }

    let mut logger_config = LoggerConfig::default();
    logger_config.log_file = Some(log_file.to_path_buf());
    amethyst::start_logger(logger_config);

    let assets_dir = application_root_dir()?.join("assets");
    let display_config_path = assets_dir.join("display_config.ron");

    /*let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.01, 0.01, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),
    );*/

    let binding_dir = assets_dir.join("bindings_config.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(&binding_dir)?;

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<Event>::default(),
            "event_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with(systems::SnowflakeSystem::new(), "snowflake_system", &[])
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            systems::DialogueSystem::default(),
            "dialogue_system",
            &["ui_transform"],
        )
        .with(systems::EventSystem::default(), "event_system", &[])
        .with(
            systems::WindSystem::default(),
            "wind_system",
            &["snowflake_system"],
        )
        .with(
            systems::GravitySystem,
            "gravity_system",
            &["snowflake_system"],
        )
        .with(
            systems::ControlSystem,
            "control_system",
            &["input_system", "dialogue_system"],
        )
        .with(
            systems::MovementSystem,
            "movement_system",
            &["gravity_system", "control_system"],
        )
        .with(
            systems::WorldCollisionSystem,
            "world_collision_system",
            &["movement_system"],
        )
        .with(
            systems::AnimationSystem::default(),
            "animation_system",
            &["movement_system", "dialogue_system"],
        )
        .with(
            systems::CameraSystem::default(),
            "camera_system",
            &["control_system"],
        )
        .with_bundle(AudioBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.1, 0.01, 0.01, 0.1])).with_plugin(RenderFlat2D::default()).with_plugin(RenderUi::default()),
            /*RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[
                    "world_collision_system",
                    "ui_transform",
                    "camera_system",
                ]),*/
        )?;
    let mut game = Application::new(assets_dir, GameState::default(), game_data)?;
    game.run();

    Ok(())
}
