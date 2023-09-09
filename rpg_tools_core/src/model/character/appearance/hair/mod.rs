use crate::model::character::appearance::hair::bun::BunStyle;
use crate::model::character::appearance::hair::hairline::Hairline;
use crate::model::character::appearance::hair::long::LongHairStyle;
use crate::model::character::appearance::hair::ponytail::position::PonytailPosition;
use crate::model::character::appearance::hair::ponytail::style::PonytailStyle;
use crate::model::character::appearance::hair::short::ShortHair;
use crate::model::color::Color;
use crate::model::length::Length;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod bun;
pub mod hairline;
pub mod long;
pub mod ponytail;
pub mod short;

/// How does the hair look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Hair {
    Bun {
        style: BunStyle,
        size: Size,
        hairline: Hairline,
        color: Color,
    },
    Long {
        style: LongHairStyle,
        hairline: Hairline,
        length: Length,
        color: Color,
    },
    None,
    Ponytail {
        position: PonytailPosition,
        style: PonytailStyle,
        hairline: Hairline,
        length: Length,
        color: Color,
    },
    Short {
        style: ShortHair,
        hairline: Hairline,
        color: Color,
    },
}

impl Hair {
    /// Mirrors along the center axis.
    pub fn mirror(&self) -> Self {
        match self {
            Hair::Ponytail {
                position,
                style,
                hairline,
                length,
                color,
            } => Hair::Ponytail {
                position: position.mirror(),
                style: *style,
                hairline: *hairline,
                length: *length,
                color: *color,
            },
            _ => *self,
        }
    }
}
