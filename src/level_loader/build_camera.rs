use super::*;

const CAMERA_SIZE: (f32, f32) = (400.0, 400.0);
const CAMERA_Z: f32 = 10.0;

impl LevelLoader {
    pub(super) fn build_camera(&self, world: &mut World) {
        let player_data: Option<(Index, Vector)> = world.exec(
            |(entities, players, transforms): (
                Entities,
                ReadStorage<Player>,
                ReadStorage<Transform>,
            )| {
                (&entities, &players, &transforms).join().next().map(
                    |(entity, _, transform)| (entity.id(), transform.into()),
                )
            },
        );

        if let Some((_player_id, _player_pos)) = player_data {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                CAMERA_SIZE.0 * 0.5,
                CAMERA_SIZE.1 * 0.5,
                CAMERA_Z,
            );

            world
                .create_entity()
                .with(transform)
                .with(Camera::standard_2d(CAMERA_SIZE.0, CAMERA_SIZE.1))
                // .with(Camera::from(Projection::orthographic(
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
}
