use super::state_prelude::*;
use crate::resource;
use crate::CAMERA_SIZE;

use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::renderer::Camera as AmethystCamera;
use deathframe::handles::TextureHandles;

const PLAYER_Z: f32 = 1.0;

#[derive(Default)]
pub struct Ingame {}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        data.world.delete_all();

        initialize_player(&mut data.world);
        // Initialize before player
        initialize_camera(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, "ingame").unwrap();

        Trans::None
    }
}

fn initialize_camera(world: &mut World) {
    use amethyst::ecs::world::Index;
    use amethyst::ecs::{Entities, Join, ReadStorage};
    use deathframe::geo::Vector;

    let player_data: Option<(Index, Vector)> = world.exec(
        |(entities, players, transforms): (
            Entities,
            ReadStorage<Player>,
            ReadStorage<Transform>,
        )| {
            (&entities, &players, &transforms)
                .join()
                .next()
                .map(|(entity, _, transform)| (entity.id(), transform.into()))
        },
    );

    if let Some((player_id, player_pos)) = player_data {
        let mut transform = Transform::default();
        transform.set_translation_xyz(player_pos.0, player_pos.1, PLAYER_Z);

        world
            .create_entity()
            .with(transform)
            .with(AmethystCamera::standard_2d(CAMERA_SIZE.0, CAMERA_SIZE.1))
            .with(Camera::new().follow(player_id).build())
            .build();
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
        .with(Size::new(32.0, 64.0))
        .with(ScaleOnce)
        .with(texture_handle)
        .build();
}
