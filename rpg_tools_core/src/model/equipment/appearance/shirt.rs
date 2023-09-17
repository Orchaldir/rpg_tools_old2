use crate::model::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The shirt of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shirt {
    pub sleeves: Sleeves,
    pub color: Color,
}

/// What style of sleeves?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sleeves {
    #[default]
    Long,
    None,
    Short,
}
