use crate::model::character::appearance::eye::brow::shape::EyebrowShape;
use crate::model::character::appearance::eye::brow::style::EyebrowStyle;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod shape;
pub mod style;

/// Defines the character's eyebrows.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EyeBrows {
    None,
    Normal {
        shape: EyebrowShape,
        style: EyebrowStyle,
    },
    Unibrow {
        shape: EyebrowShape,
        style: EyebrowStyle,
    },
}

impl Default for EyeBrows {
    fn default() -> Self {
        Self::Normal {
            shape: EyebrowShape::default(),
            style: EyebrowStyle::default(),
        }
    }
}
