use crate::model::character::appearance::Side;
use crate::model::color::Color;

/// How does the hair look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hair {
    None,
    /// Short normal hair.
    Short {
        style: ShortHair,
        color: HairColor,
    },
}

/// Which short hair style?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortHair {
    /// All hair is equally short.
    BuzzCut,
    CrewCut,
    SidePart(Side),
}

/// The color of the hair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HairColor {
    White,
    Grey,
    Blond,
    Orange,
    Red,
    Brown,
    Black,
    Exotic(Color),
}
