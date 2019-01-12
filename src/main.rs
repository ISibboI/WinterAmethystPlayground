#![feature(nll)]
// Disable some warnings
#[allow(unused_imports)]

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
    assets::PrefabLoaderSystem,
    audio::{AudioBundle, SourceHandle},
    core::transform::TransformBundle,
    input::InputBundle,
    LoggerConfig,
    prelude::*,
    renderer::{
        ALPHA, ColorMask, DepthMode, DisplayConfig, DrawSprite, Pipeline, RenderBundle, Stage,
    },
    ui::{DrawUi, UiBundle},
};

use events::Event;
use states::game::{GameState, Music};

mod components;
mod entities;
mod events;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    let log_file = Path::new("debug.log");
    if log_file.is_file() {
        fs::remove_file(log_file)
            .unwrap_or_else(|error| warn!("Could not delete old log file: {}", error));
    }

    let mut logger_config = LoggerConfig::default();
    logger_config.log_file = Some(log_file.to_path_buf());
    amethyst::start_logger(logger_config);

    let path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.1, 0.01, 0.01, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()),
    );

    let binding_path = "./resources/bindings_config.ron";
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<Event>::default(),
            "event_loader",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with(systems::SnowflakeSystem::new(), "snowflake_system", &[])
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
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
        .with_bundle(AudioBundle::new(|_: &mut Music| None))?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&[
                    "world_collision_system",
                    "ui_transform",
                    "camera_system",
                ]),
        )?;
    let mut game = Application::new("./", GameState::default(), game_data)?;
    game.run();

    Ok(())
}
