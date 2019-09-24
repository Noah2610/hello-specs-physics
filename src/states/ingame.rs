use super::state_prelude::*;

#[derive(Default)]
pub struct Ingame {}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
}
