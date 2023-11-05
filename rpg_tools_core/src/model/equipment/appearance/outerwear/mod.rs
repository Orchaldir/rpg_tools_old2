use crate::model::equipment::appearance::outerwear::cloak::Cloak;
use crate::model::equipment::appearance::outerwear::coat::Coat;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod cloak;
pub mod coat;

/// The outerwear of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Outerwear {
    #[default]
    None,
    Cloak(Cloak),
    Coat(Coat),
}

/// How long is the [`outerwear`](Outerwear)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum OuterwearLength {
    #[default]
    Hip,
    Knee,
    Ankle,
}
