use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::{Deserialize, Serialize};
use ui_macro::ui;

/// What type of hairline? It is not visible by some hair styles.
///
/// The [`size`](Size) defines the y position of the hairline.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
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
