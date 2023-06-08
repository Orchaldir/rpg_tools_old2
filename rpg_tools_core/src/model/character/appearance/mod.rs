use crate::model::character::appearance::skin::Skin;

pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
/// Currently this is limited to humanoids.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Appearance {
    skin: Skin,
}

impl Appearance {
    pub fn new(skin: Skin) -> Self {
        Self { skin }
    }

    pub fn skin(&self) -> &Skin {
        &self.skin
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(Skin::default())
    }
}
