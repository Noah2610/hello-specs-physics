use super::state_prelude::*;

use amethyst::assets::ProgressCounter;
use amethyst::utils::app_root_dir::application_dir;
use deathframe::menu::{Menu, UiData};

const UI_RON_PATH: &str = "resources/ui/ingame.ron";

#[derive(Default)]
pub struct Ingame {
    ui_data:     UiData,
    ui_progress: Option<ProgressCounter>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        self.ui_progress = Some(self.create_ui(&mut data));
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<CustomGameData<'a, 'b, ()>>,
    ) -> Trans<CustomGameData<'a, 'b, ()>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            return trans;
        }

        // Print ui progress errors
        if let Some(progress) = self.ui_progress.as_ref() {
            for error in progress.errors() {
                eprintln!("{:#?}", error);
            }
        }

        Trans::None
    }
}

impl<'a, 'b> Menu<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn event_triggered(
        &mut self,
        data: &mut StateData<CustomGameData<'a, 'b, ()>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, ()>, StateEvent>> {
        dbg!(&event_name);

        match event_name.as_ref() {
            "btn_primary" => None,
            "btn_secondary" => Some(Trans::Pop),
            _ => None,
        }
    }

    fn ui_ron_path(&self) -> String {
        // TODO stupid
        application_dir(UI_RON_PATH)
            .expect("Should have root dir")
            .into_os_string()
            .to_str()
            .expect("Should have root dir")
            .to_string()
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
