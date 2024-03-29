use super::component_prelude::*;

/// For entities which have `Transform`, `Size`, `SpriteRender`, and `ScaleOnce`,
/// their sprites will be scaled to the entity's size once.
/// After scaling, this component is removed from the entity.
#[derive(Default, Serialize, Deserialize)]
pub struct ScaleOnce;

impl Component for ScaleOnce {
    type Storage = NullStorage<Self>;
}
