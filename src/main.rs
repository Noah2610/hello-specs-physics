extern crate amethyst;
extern crate regex;
extern crate specs_physics;
#[macro_use]
extern crate serde;

mod components;
mod custom_game_data;
mod helpers;
mod input;
mod input_manager;
mod level_loader;
mod menu;
mod music;
mod sprite_sheet_handles;
mod states;
mod systems;

use amethyst::audio::{AudioBundle, DjSystemDesc};
use amethyst::config::Config;
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst::{ApplicationBuilder, LogLevelFilter, LoggerConfig};

use custom_game_data::prelude::*;
use helpers::*;

fn main() -> Result<(), String> {
    init_game().map_err(|e| e.to_string())
}

fn init_game() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<()>> =
        ApplicationBuilder::new("./", states::MainMenu::default())?
            .with_frame_limit_config(FrameRateLimitConfig::load(resource(
                "config/frame_limiter.ron",
            )))
            .build(game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>(
) -> amethyst::Result<CustomGameDataBuilder<'a, 'b, ()>> {
    use music::Music;
    use systems::prelude::*;

    let display_config_file = application_root_dir()
        .expect("Couldn't get game's root directory")
        .join("resources/config/display.ron");

    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_file)
                .with_clear([0.0, 0.0, 0.0, 0.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default());
    let transform_bundle = TransformBundle::new();
    let input_bundle = input::input_bundle()
        .with_bindings_from_file(resource("config/bindings.ron"))
        .unwrap();
    let ui_bundle = UiBundle::<input::Bindings>::new();
    let audio_bundle = AudioBundle::default();

    let custom_game_data = CustomGameDataBuilder::<'a, 'b, ()>::default()
        .dispatcher("main_menu")?
        .dispatcher("ingame")?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(input_bundle)?
        .with_core_bundle(ui_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )?
        .with_core(ScaleSpritesSystem, "scale_sprites", &[])?
        .with_core(
            InputManagerSystem::<input::Bindings>::default(),
            "input_manager",
            &[],
        )?
        .with(
            "ingame",
            DecreaseVelocitiesSystem::default(),
            "decrease_velocities",
            &[],
            // &["move_entities"], // TODO
        )?
        // .with("ingame", CameraSystem::default(), "camera", &[])?
        .with("ingame", MovePlayer::default(), "move_player", &[])?;

    Ok(custom_game_data)
}
