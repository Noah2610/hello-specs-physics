use super::state_prelude::*;
use crate::resource;

use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::handles::TextureHandles;

#[derive(Default)]
pub struct Ingame {}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        initialize_player(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        Trans::None
    }
}

fn initialize_player(world: &mut World) {
    let texture_handle = world
        .write_resource::<TextureHandles>()
        .get_or_load(resource("textures/player.png"), world);

    world
        .create_entity()
        .with(Player::default())
        .with(Transform::default())
        .with(Velocity::default())
        .with(texture_handle)
        .build();
}
