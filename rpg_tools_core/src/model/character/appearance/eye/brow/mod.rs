use crate::model::character::appearance::eye::brow::shape::EyebrowShape;
use crate::model::character::appearance::eye::brow::style::EyebrowStyle;
use crate::model::color::Color;
use crate::model::width::Width;
use macro_core::parser::UiParser;
use macro_core::visitor::{UiVisitor, UI};
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
        width: Width,
    },
    Unibrow {
        color: Color,
        shape: EyebrowShape,
        style: EyebrowStyle,
        width: Width,
    },
}
