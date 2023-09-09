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

impl PonytailPosition {
    /// Mirrors along the center axis.
    pub fn mirror(&self) -> Self {
        match self {
            PonytailPosition::Left => PonytailPosition::Right,
            PonytailPosition::Right => PonytailPosition::Left,
            _ => *self,
        }
    }
}
