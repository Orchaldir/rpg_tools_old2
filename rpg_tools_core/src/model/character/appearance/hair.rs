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
    /// The hair on the top of the head is cut and styled upright to form a flat profile.
    FlatTop(Size),
    // Short hair that parts in the middle.
    MiddlePart,
    // Short hair that parts on one side.
    SidePart(Side),
}

/// What type of hairline? It is not visible by some hair styles.
///
/// The [`size`](Size) defines the y position of the hairline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hairline {
    Round(Size),
    Straight(Size),
    Triangle(Size),
    WidowsPeak(Size),
}

impl Hairline {
    pub fn get_y_position(&self) -> Size {
        match self {
            Hairline::Round(y) => *y,
            Hairline::Straight(y) => *y,
            Hairline::Triangle(y) => *y,
            Hairline::WidowsPeak(y) => *y,
        }
    }
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
