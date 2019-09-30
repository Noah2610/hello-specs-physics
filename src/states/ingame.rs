use super::state_prelude::*;
use amethyst::utils::application_root_dir;
use amethyst::window::{DisplayConfig, ScreenDimensions};
use std::fs::File;

const HIDPI: f64 = 1.0;

#[derive(Default)]
pub struct Ingame {}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        add_resources(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, "ingame").unwrap();

        Trans::None
    }
}

fn add_resources(data: &mut StateData<CustomGameData>) {
    add_resource_display_config(data);
    add_resource_screen_dimensions(data); // NOTE: Has to be called _after_ DisplayConfig resource is added.
}

fn add_resource_display_config(data: &mut StateData<CustomGameData>) {
    let display_config_file = {
        let mut path =
            application_root_dir().expect("Couldn't get game's root directory");
        path.push("resources/config/display.ron");
        File::open(path).expect("Couldn't open display.ron file for reading")
    };
    let display_config: DisplayConfig =
        ron::de::from_reader(display_config_file)
            .expect("Couldn't parse display.ron file");

    data.world.add_resource(display_config);
}

fn add_resource_screen_dimensions(data: &mut StateData<CustomGameData>) {
    let screen_dimensions = {
        let display_config = data.world.read_resource::<DisplayConfig>();
        if let Some(dim) = display_config.dimensions {
            ScreenDimensions::new(dim.0, dim.1, HIDPI)
        } else {
            ScreenDimensions::new(1280, 720, HIDPI)
        }
    };
    data.world.add_resource(screen_dimensions);
}
