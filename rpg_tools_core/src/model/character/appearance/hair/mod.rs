use crate::model::character::appearance::hair::bun::BunStyle;
use crate::model::character::appearance::hair::hairline::Hairline;
use crate::model::character::appearance::hair::long::LongHairStyle;
use crate::model::character::appearance::hair::ponytail::Ponytail;
use crate::model::character::appearance::hair::short::ShortHair;
use crate::model::color::Color;
use crate::model::length::Length;
use crate::model::size::Size;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod bun;
pub mod hairline;
pub mod long;
pub mod ponytail;
pub mod short;

/// How does the hair look like?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
    #[default]
    None,
    Ponytail(Ponytail),
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
            Hair::Ponytail(ponytail) => Hair::Ponytail(ponytail.mirror()),
            _ => *self,
        }
    }
}
