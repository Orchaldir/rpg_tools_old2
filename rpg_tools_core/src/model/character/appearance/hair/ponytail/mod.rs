use crate::model::character::appearance::hair::hairline::Hairline;
use crate::model::character::appearance::hair::ponytail::position::PonytailPosition;
use crate::model::character::appearance::hair::ponytail::style::PonytailStyle;
use crate::model::color::Color;
use crate::model::length::Length;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod position;
pub mod style;

/// How does the ponytail look like?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ponytail {
    pub position: PonytailPosition,
    pub style: PonytailStyle,
    pub hairline: Hairline,
    pub length: Length,
    pub color: Color,
}

impl Ponytail {
    /// Mirrors along the center axis.
    pub fn mirror(&self) -> Self {
        Self {
            position: self.position.mirror(),
            ..*self
        }
    }
}
