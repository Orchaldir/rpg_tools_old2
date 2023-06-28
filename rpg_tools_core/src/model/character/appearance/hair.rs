use crate::model::character::appearance::{Side, Size};
use crate::model::color::Color;

/// How does the hair look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hair {
    None,
    /// Short normal hair.
    Short {
        style: ShortHair,
        hairline: Hairline,
        color: HairColor,
    },
}

/// Which short hair style?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortHair {
    /// All hair is equally short.
    BuzzCut,
    CrewCut,
    MiddlePart,
    SidePart(Side),
}

/// What type of hairline?
///
/// The [`size`](Size) defines the y position of the hairline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hairline {
    Round(Size),
    Straight(Size),
    Triangle(Size),
    WidowsPeak(Size),
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
