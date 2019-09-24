extern crate amethyst;
extern crate deathframe;
extern crate specs_physics;
#[macro_use]
extern crate serde;

mod input;
mod states;

use amethyst::core::transform::TransformBundle;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::ApplicationBuilder;
use amethyst::{LogLevelFilter, LoggerConfig};
use deathframe::custom_game_data::{CustomGameData, CustomGameDataBuilder};

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
    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new();
    let transform_bundle = TransformBundle::new();
    let input_bundle = input::input_bundle();

    let custom_game_data = CustomGameDataBuilder::new()
        .dispatcher("ingame")?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(input_bundle)?;

    Ok(custom_game_data)
}
