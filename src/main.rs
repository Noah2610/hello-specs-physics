extern crate amethyst;
extern crate deathframe;
extern crate specs_physics;
#[macro_use]
extern crate serde;

mod components;
mod input;
mod music;
mod solid_tag;
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
use deathframe::custom_game_data::prelude::*;

pub fn resource<S>(path: S) -> String
where
    S: ToString,
{
    use amethyst::utils::app_root_dir::application_dir;
    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path.to_string());
    path.to_str().unwrap().to_string()
}

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
    use deathframe::systems::InputManagerSystem;
    use music::Music;
    use systems::prelude::*;

    let display_config_file = application_root_dir()
        .expect("Couldn't get game's root directory")
        .join("resources/config/display.ron");

    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_file)
                .with_clear([0.0, 0.0, 0.0, 1.0]),
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
        .with_core(
            InputManagerSystem::<input::Bindings>::default(),
            "input_manager",
            &[],
        )?
        .with(
            "ingame",
            MoveEntitiesSystem::<solid_tag::SolidTag>::default(),
            "move_entities",
            &[],
        )?
        .with("ingame", MovePlayer::default(), "move_player", &[])?;

    Ok(custom_game_data)
}
