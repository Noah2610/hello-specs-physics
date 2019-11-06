pub mod prelude {
    pub use super::decrease_velocities::DecreaseVelocitiesSystem;
    pub use super::input_manager::InputManagerSystem;
    pub use super::move_player::MovePlayer;
    pub use super::scale_sprites::ScaleSpritesSystem;
}

pub mod system_prelude {
    pub use amethyst::assets::{AssetStorage, Handle};
    pub use amethyst::core::timing::Time;
    pub use amethyst::ecs::world::Index;
    pub use amethyst::ecs::{
        Entities,
        Entity,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        Storage,
        System,
        Write,
        WriteExpect,
        WriteStorage,
    };
    pub use amethyst::ecs::{World, WorldExt};
    pub use amethyst::input::InputHandler;
    pub use amethyst::renderer::sprite::{SpriteSheet, SpriteSheetHandle};
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        SpriteRender,
        Texture,
    };

    pub use crate::components::prelude::*;
    pub use crate::input_manager::InputManager;

    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
}

mod decrease_velocities;
mod input_manager;
mod move_player;
mod scale_sprites;
