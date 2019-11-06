pub mod prelude {
    pub use super::Ingame;
    pub use super::MainMenu;
}

pub mod state_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::ui::{UiEvent, UiEventType};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::custom_game_data::CustomGameData;
    pub use crate::input_manager::InputManager;
    pub use crate::menu::prelude::*;
}

mod ingame;
mod main_menu;

pub use ingame::Ingame;
pub use main_menu::MainMenu;
