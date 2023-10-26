use crate::model::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The belt of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Belt {
    pub buckle: Buckle,
    pub color: Color,
}

/// The pants of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buckle {
    pub style: BuckleStyle,
    pub color: Color,
}

/// The style of [`buckle`](Buckle).
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuckleStyle {
    Box,
    #[default]
    Frame,
    Plate,
}
