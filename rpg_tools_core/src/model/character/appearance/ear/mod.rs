use crate::model::character::appearance::ear::shape::EarShape;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use ui_macro::ui;

pub mod shape;

/// How many ears does the character have?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Ears {
    None,
    /// Like a human's ears.
    Normal {
        shape: EarShape,
        size: Size,
    },
}

impl Default for Ears {
    fn default() -> Self {
        Self::Normal {
            shape: EarShape::default(),
            size: Size::Medium,
        }
    }
}
