use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// Where is the starting position of the ponytail(s)?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PonytailPosition {
    BothSides,
    High,
    Left,
    #[default]
    Low,
    Right,
}

/// What style of ponytail?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PonytailStyle {
    Bubble,
    #[default]
    Straight,
    Wide,
}
