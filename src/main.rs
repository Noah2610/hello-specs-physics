extern crate amethyst;
extern crate deathframe;
extern crate specs_physics;
#[macro_use]
extern crate serde;

mod input;
mod states;

use amethyst::core::transform::TransformBundle;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst::{ApplicationBuilder, LogLevelFilter, LoggerConfig};
use deathframe::custom_game_data::prelude::*;

fn main() -> Result<(), String> {
    init_game().map_err(|e| e.to_string())
}

fn init_game() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<()>> =
        ApplicationBuilder::new("./", states::Ingame::default())?
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
    let display_config_file = application_root_dir()
        .expect("Couldn't get game's root directory")
        .join("resources/config/display.ron");

    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_file)
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());
    // .with_plugin(DrawUi::new());
    let transform_bundle = TransformBundle::new();
    let input_bundle = input::input_bundle();
    let ui_bundle = UiBundle::<input::Bindings>::new();

    let custom_game_data = CustomGameDataBuilder::<'a, 'b, ()>::default()
        .dispatcher("ingame")?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(input_bundle)?
        .with_core_bundle(ui_bundle)?;

    Ok(custom_game_data)
}
