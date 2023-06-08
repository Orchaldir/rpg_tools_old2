use crate::model::color::Color;

/// The skin of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Skin {
    Scales(Color),
    Skin(SkinColor),
}

impl Default for Skin {
    fn default() -> Self {
        Self::Skin(SkinColor::Exotic(Color::Aqua))
    }
}

/// The skin color of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SkinColor {
    Light,
    Medium,
    Warm,
    Tan,
    Dark,
    VeryDark,
    Exotic(Color),
}
