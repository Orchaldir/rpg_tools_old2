use crate::model::character::appearance::body::Body;
use crate::model::character::appearance::skin::Skin;
use crate::model::length::Length;

pub mod body;
pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Appearance {
    /// The [`character`](crate::model::character::Character) consists only of a head. E.g. a floating skull
    HeadOnly { height: Length, skin: Skin },
    /// The [`character`](crate::model::character::Character) is a humanoid. E.g. a human
    Humanoid { body: Body, height: Length },
}

impl Appearance {
    pub fn head(height: Length, skin: Skin) -> Self {
        Self::HeadOnly { height, skin }
    }

    pub fn humanoid(body: Body, height: Length) -> Self {
        Self::Humanoid { body, height }
    }
}
