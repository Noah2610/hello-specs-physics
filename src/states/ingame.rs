use super::state_prelude::*;
use crate::resource;

use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::renderer::Camera as AmethystCamera;

const CAMERA_SIZE: (f32, f32) = (100.0, 100.0);
const PLAYER_DECR_VEL: (f32, f32) = (1000.0, 1000.0);

#[derive(Default)]
pub struct Ingame {}

impl<'a, 'b> State<CustomGameData<'a, 'b, ()>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData>) {
        data.world.delete_all();

        initialize_player(&mut data.world);
        // Initialize player before camera
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

    if let Some((_player_id, _player_pos)) = player_data {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            CAMERA_SIZE.0 * 0.5,
            CAMERA_SIZE.1 * 0.5,
            10.0,
        );

        world
            .create_entity()
            .with(transform)
            .with(AmethystCamera::standard_2d(CAMERA_SIZE.0, CAMERA_SIZE.1))
            // .with(AmethystCamera::from(Projection::orthographic(
            //     -CAMERA_SIZE.0 * 0.5, // Left
            //     CAMERA_SIZE.0 * 0.5,  // Right
            //     -CAMERA_SIZE.1 * 0.5, // Bottom (!)
            //     CAMERA_SIZE.1 * 0.5,  // Top    (!)
            //     0.0,
            //     10.0,
            // )))
            // .with(Camera::new().follow(player_id).build())
            .build();
    }
}

fn initialize_player(world: &mut World) {
    use amethyst::renderer::SpriteRender;
    use deathframe::handles::SpriteSheetHandles;

    let mut transform = Transform::default();
    transform.set_translation_xyz(10.0, 50.0, 0.0);

    let sprite_sheet_handle = world
        .write_resource::<SpriteSheetHandles>()
        .get_or_load(resource("spritesheets/player.png"), world);
    let sprite_render = SpriteRender {
        sprite_sheet:  sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(Player::default())
        .with(transform)
        .with(Velocity::default())
        .with(DecreaseVelocity::new(PLAYER_DECR_VEL.0, PLAYER_DECR_VEL.1))
        .with(Size::new(32.0, 64.0))
        .with(ScaleOnce)
        .with(sprite_render)
        .build();
}
