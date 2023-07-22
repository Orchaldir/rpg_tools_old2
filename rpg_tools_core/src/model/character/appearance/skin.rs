use crate::model::color::Color;
use serde::Serialize;

/// The skin of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum SkinColor {
    Fair,
    Light,
    Medium,
    Tan,
    Dark,
    VeryDark,
    Exotic(Color),
}
