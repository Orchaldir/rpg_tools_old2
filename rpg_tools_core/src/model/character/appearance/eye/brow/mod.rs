use crate::model::character::appearance::eye::brow::shape::EyebrowShape;
use crate::model::character::appearance::eye::brow::style::EyebrowStyle;
use crate::model::color::Color;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod shape;
pub mod style;

/// Defines the character's eyebrows.
#[derive(Default, ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EyeBrows {
    #[default]
    None,
    Normal {
        color: Color,
        shape: EyebrowShape,
        style: EyebrowStyle,
    },
    Unibrow {
        color: Color,
        shape: EyebrowShape,
        style: EyebrowStyle,
    },
}
