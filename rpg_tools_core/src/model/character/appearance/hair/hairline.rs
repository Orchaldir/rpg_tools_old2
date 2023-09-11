use crate::model::size::Size;
use crate::ui::parser::get_enum;
use crate::ui::parser::UiParser;
use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// /// What style of hairline? It is not visible by some hair styles.
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairlineStyle {
    #[default]
    Round,
    Straight,
    Triangle,
    /// ```svgbob
    ///   +----*  *----+
    ///  /      \/      \
    ///  |              |
    /// ```
    WidowsPeak,
}

/// How does the hairline look like?

#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hairline {
    pub style: HairlineStyle,
    /// Defines the y position of the hairline.
    pub size: Size,
}

impl Hairline {
    pub fn new(style: HairlineStyle, size: Size) -> Self {
        Self { style, size }
    }
}

impl Default for Hairline {
    fn default() -> Self {
        Self {
            style: HairlineStyle::default(),
            size: Size::Medium,
        }
    }
}
