use crate::model::appearance::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The footwear of a [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Footwear {
    pub color: Color,
    pub style: FootwearStyle,
    pub sole: Color,
}

/// What style of [`footwear`](Footwear)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FootwearStyle {
    Boots,
    KneeHighBoots,
    Sandals,
    #[default]
    Shoes,
    Slippers,
}

impl FootwearStyle {
    pub fn is_over_pants(&self) -> bool {
        *self == Self::KneeHighBoots
    }
}
