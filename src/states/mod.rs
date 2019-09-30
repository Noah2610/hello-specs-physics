pub mod prelude {
    pub use super::Ingame;
}

pub mod state_prelude {
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::CustomGameData;
}

mod ingame;

pub use ingame::Ingame;
