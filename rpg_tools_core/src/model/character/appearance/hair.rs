use crate::model::color::Color;
use crate::model::side::Side;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use ui_macro::ui;

/// How does the hair look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
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
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
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
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Hairline {
    Round(Size),
    Straight(Size),
    Triangle(Size),
    /// ```svgbob
    ///   +----*  *----+
    ///  /      \/      \
    ///  |              |
    /// ```
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
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
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
