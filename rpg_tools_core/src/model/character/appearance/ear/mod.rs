use crate::model::appearance::size::Size;
use crate::model::character::appearance::ear::shape::EarShape;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod shape;

/// How many ears does the character have?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
