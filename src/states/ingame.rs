use super::state_prelude::*;
use crate::level_loader::LevelLoader;

const LEVEL_NAME: &str = "level.json";

#[derive(Default)]
pub struct Ingame {
    level_loader: LevelLoader,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        data.world.delete_all();

        self.level_loader.load(LEVEL_NAME);
        self.level_loader.build(data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        Trans::None
    }
}
