use crate::model::character::appearance::skin::Skin;
use crate::model::length::Length;

pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
/// Currently this is limited to humanoids.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Appearance {
    height: Length,
    skin: Skin,
}

impl Appearance {
    pub fn new(height: Length, skin: Skin) -> Self {
        Self { height, skin }
    }

    pub fn skin(&self) -> &Skin {
        &self.skin
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::new(Length::from_metre(1.0), Skin::default())
    }
}
