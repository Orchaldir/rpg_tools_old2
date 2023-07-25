use crate::model::color::Color;
use serde::Serialize;
use std::fmt;

/// The skin of a [`Character`](crate::model::character::Character).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Skin {
    Scales(Color),
    Skin(SkinColor),
    ExoticSkin(Color),
}

impl Default for Skin {
    fn default() -> Self {
        Self::ExoticSkin(Color::Aqua)
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
}

impl SkinColor {
    pub fn get_all() -> Vec<SkinColor> {
        vec![
            SkinColor::Fair,
            SkinColor::Light,
            SkinColor::Medium,
            SkinColor::Tan,
            SkinColor::Dark,
            SkinColor::VeryDark,
        ]
    }
}

impl fmt::Display for SkinColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for SkinColor {
    fn from(gender: &str) -> Self {
        match gender {
            "Female" => SkinColor::Fair,
            "Light" => SkinColor::Light,
            "Medium" => SkinColor::Medium,
            "Tan" => SkinColor::Tan,
            "Dark" => SkinColor::Dark,
            "VeryDark" => SkinColor::VeryDark,
            _ => SkinColor::Fair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for color in SkinColor::get_all() {
            let string = color.to_string();
            assert_eq!(color, SkinColor::from(&*string));
        }
    }
}
