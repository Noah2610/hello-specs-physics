use amethyst::assets::ProgressCounter;
use deathframe::input_manager::InputManager;
use deathframe::menu::{Menu, UiData};

use super::state_prelude::*;
use crate::helpers::*;

const UI_RON_PATH: &str = "ui/menu.ron";

#[derive(Default)]
pub struct MainMenu {
    ui_data:     UiData,
    ui_progress: Option<ProgressCounter>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for MainMenu {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        // InputManager
        data.world
            .insert(InputManager::<crate::input::Bindings>::default());

        // Music
        crate::music::initialize_music(data.world);

        // MainMenu UI
        self.ui_progress =
            Some(self.create_ui(&mut data, resource(UI_RON_PATH)));

        // SpriteSheetHandles and TextureHandles
        data.world
            .insert(deathframe::handles::SpriteSheetHandles::default());
        data.world
            .insert(deathframe::handles::TextureHandles::default());
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "main_menu").unwrap();

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

impl<'a, 'b> Menu<CustomGameData<'a, 'b, ()>, StateEvent> for MainMenu {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<CustomGameData<'a, 'b, ()>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<CustomGameData<'a, 'b, ()>, StateEvent>> {
        if let UiEventType::ClickStart = event.event_type {
            match event_name.as_ref() {
                "btn_primary" => Some(Trans::Push(Box::new(Ingame::default()))),
                "btn_secondary" => Some(Trans::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
