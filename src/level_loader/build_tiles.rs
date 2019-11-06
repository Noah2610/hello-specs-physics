use super::*;

const TILE_Z: f32 = 0.0;

impl LevelLoader {
    pub(super) fn build_tiles(&self, world: &mut World) {
        for EntityData {
            pos,
            size,
            properties,
            sprite,
        } in &self.tiles_data
        {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(TILE_Z),
            );

            let sprite_render_opt = if let Some(sprite_data) = sprite {
                let spritesheet_handle = world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(&sprite_data.spritesheet_path, &world);
                Some(SpriteRender {
                    sprite_sheet:  spritesheet_handle.clone(),
                    sprite_number: sprite_data.sprite_id,
                })
            } else {
                None
            };

            let mut entity = world
                .create_entity()
                .with(transform)
                .with(Size::new(size.0, size.1))
                .with(ScaleOnce::default())
                .with(Transparent);

            if let Some(sprite_render) = sprite_render_opt {
                entity = entity.with(sprite_render);
            }

            if is_solid(&properties) {
                // TODO
            }

            entity.build();
        }
    }
}
