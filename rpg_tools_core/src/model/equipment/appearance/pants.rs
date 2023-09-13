use crate::model::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// Clothing for the lower body.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pants {
    style: PantsStyle,
    rise: Rise,
    color: Color,
}

/// What style of pants?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PantsStyle {
    Bermuda,
    HotPants,
    #[default]
    Regular,
    Shorts,
}

/// The distance from the crotch seam to the top of the waistband.
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rise {
    Low,
    #[default]
    Regular,
    High,
}
