use crate::model::character::appearance::hair::hairline::Hairline;
use crate::model::color::Color;
use crate::model::side::Side;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use ui_macro::ui;

pub mod hairline;

/// How does the hair look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Hair {
    None,
    /// Short normal hair.
    Short {
        style: ShortHair,
        hairline: Hairline,
        color: Color,
    },
}

/// Which short hair style?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "c")]
pub enum ShortHair {
    /// All hair is equally short.
    BuzzCut,
    /// The hair on the top of the head is cut and styled upright to form a flat profile.
    FlatTop(Size),
    // Short hair that parts in the middle.
    MiddlePart,
    // Short hair that parts on one side.
    SidePart(Side),
}
