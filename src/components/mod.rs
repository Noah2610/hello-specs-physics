pub mod prelude {
    pub use amethyst::core::transform::Transform;
    pub use amethyst::core::Hidden;
    pub use amethyst::renderer::Transparent;

    pub use super::decrease_velocity::DecreaseVelocity;
    pub use super::player::Player;
    pub use super::scale_once::ScaleOnce;
    pub use super::size::Size;
    pub use super::velocity::Velocity;
}

mod component_prelude {
    pub use amethyst::ecs::{
        Component,
        DenseVecStorage,
        HashMapStorage,
        NullStorage,
        Storage,
        VecStorage,
    };

    pub use crate::helpers::*;
}

mod decrease_velocity;
mod player;
mod scale_once;
mod size;
mod velocity;
