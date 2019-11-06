use deathframe::components::solid::SolidTag as STag;

#[derive(Clone)]
pub enum SolidTag {
    Player,
    Tile,
}

impl STag for SolidTag {
    fn collides_with(&self, other: &Self) -> bool {
        match (self, other) {
            (SolidTag::Player, SolidTag::Tile)
            | (SolidTag::Tile, SolidTag::Player) => true,
            _ => false,
        }
    }
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::Tile
    }
}
