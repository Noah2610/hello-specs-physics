#[derive(Clone, PartialEq)]
pub enum SolidTag {
    All,
}

impl Default for SolidTag {
    fn default() -> Self {
        SolidTag::All
    }
}
