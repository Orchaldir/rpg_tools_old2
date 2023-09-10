use crate::model::size::Size;
use crate::ui::parser::UiParser;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What type of hairline? It is not visible by some hair styles.
///
/// The [`size`](Size) defines the y position of the hairline.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
pub enum Hairline {
    Round {
        size: Size,
    },
    Straight {
        size: Size,
    },
    Triangle {
        size: Size,
    },
    /// ```svgbob
    ///   +----*  *----+
    ///  /      \/      \
    ///  |              |
    /// ```
    WidowsPeak {
        size: Size,
    },
}

impl Hairline {
    pub fn get_y_position(&self) -> Size {
        match self {
            Hairline::Round { size } => *size,
            Hairline::Straight { size } => *size,
            Hairline::Triangle { size } => *size,
            Hairline::WidowsPeak { size } => *size,
        }
    }
}

impl Default for Hairline {
    fn default() -> Self {
        Self::Round { size: Size::Medium }
    }
}
