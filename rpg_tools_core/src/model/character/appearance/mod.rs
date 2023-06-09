use crate::model::character::appearance::skin::Skin;
use crate::model::length::Length;

pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    /// The [`character`](crate::model::character::Character) consists only of a head. E.g. a floating skull
    HeadOnly { height: Length, skin: Skin },
    /// The [`character`](crate::model::character::Character) is a humanoid. E.g. a human
    Humanoid { height: Length, skin: Skin },
}

impl Appearance {
    pub fn head(height: Length, skin: Skin) -> Self {
        Self::HeadOnly { height, skin }
    }

    pub fn humanoid(height: Length, skin: Skin) -> Self {
        Self::Humanoid { height, skin }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Self::humanoid(Length::from_metre(1.0), Skin::default())
    }
}
