use crate::model::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The shirt of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shirt {
    pub sleeve_style: SleeveStyle,
    pub neckline: Neckline,
    pub color: Color,
}

/// What style of neckline?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Neckline {
    Boat,
    #[default]
    Crew,
    DeepV,
    Scoop,
    V,
}

/// What style of sleeves?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleeveStyle {
    #[default]
    Long,
    None,
    Short,
}
