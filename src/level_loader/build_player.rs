use super::*;

const PLAYER_DECR_VEL: (f32, f32) = (1000.0, 1000.0);
const PLAYER_Z: f32 = 1.0;

impl LevelLoader {
    pub(super) fn build_player(&self, world: &mut World) {
        if let Some(EntityData {
            pos,
            size,
            properties,
            sprite: _,
        }) = self.player_data.as_ref()
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(PLAYER_Z),
            );

            let size = Size::new(size.0, size.1);

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
                .with(DecreaseVelocity::new(
                    PLAYER_DECR_VEL.0,
                    PLAYER_DECR_VEL.1,
                ))
                .with(size)
                .with(ScaleOnce)
                .with(sprite_render)
                .build();
        }
    }
}
