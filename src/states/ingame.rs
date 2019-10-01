use super::state_prelude::*;

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
    // add_resource_display_config(data);
    // add_resource_screen_dimensions(data); // NOTE: Has to be called _after_ DisplayConfig resource is added.
}
