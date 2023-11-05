use crate::model::color::Color;
use crate::model::equipment::appearance::option::neckline::Neckline;
use crate::model::equipment::appearance::option::sleeve::SleeveStyle;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The shirt of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shirt {
    pub sleeve_style: SleeveStyle,
    pub neckline: Neckline,
    pub color: Color,
}
