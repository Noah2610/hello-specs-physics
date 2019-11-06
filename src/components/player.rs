use super::component_prelude::*;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
