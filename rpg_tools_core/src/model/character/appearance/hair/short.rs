use crate::model::appearance::side::Side;
use crate::model::appearance::size::Size;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// Which short hair style?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShortHair {
    /// All hair is equally short.
    BuzzCut,
    /// The hair on the top of the head is cut and styled upright to form a flat profile.
    FlatTop {
        size: Size,
    },
    // Short hair that parts in the middle.
    #[default]
    MiddlePart,
    // Short hair that parts on one side.
    SidePart {
        side: Side,
    },
}

impl ShortHair {
    pub fn flat_top(size: Size) -> Self {
        Self::FlatTop { size }
    }

    pub fn side_part(side: Side) -> Self {
        Self::SidePart { side }
    }
}
