use super::state_prelude::*;

use deathframe::menu::{Menu, UiData};

const UI_RON_PATH: &str = "resources/ui/ingame.ron";

#[derive(Default)]
pub struct Ingame {
    ui_data: UiData,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.create_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        Trans::None
    }

    // TODO
    // fn fixed_update(
    //     &mut self,
    //     mut data: StateData<CustomGameData<()>>,
    // ) -> Trans<CustomGameData<'a, 'b, ()>, StateEvent> {
    //     if let Some(trans) = self.update_ui_events(&mut data) {
    //         return trans;
    //     }
    //     Trans::None
    // }
}

impl<'a, 'b> Menu<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn event_triggered(
        &mut self,
        data: &mut StateData<CustomGameData<'a, 'b, ()>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, ()>, StateEvent>> {
        match event_name.as_ref() {
            "btn_primary" => None,
            "btn_secondary" => Some(Trans::Pop),
            _ => None,
        }
    }

    fn ui_ron_path(&self) -> &str {
        UI_RON_PATH
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
