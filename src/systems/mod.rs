pub mod prelude {
    pub use super::move_player::MovePlayer;
    pub use deathframe::systems::prelude::*;
}

pub mod system_prelude {
    pub use amethyst::ecs::{World, WorldExt};
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
}

mod move_player;
