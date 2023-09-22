use crate::model::color::Color;
use crate::model::width::Width;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The footwear of a [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Footwear {
    pub color: Color,
    pub style: FootwearStyle,
    pub sole: Sole,
}

/// What style of [`footwear`](Footwear)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FootwearStyle {
    Boot,
    KneeHighBoot,
    Sandals,
    #[default]
    Shoe,
    Slippers,
}

/// The sole of [`footwear`](Footwear).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sole {
    pub color: Color,
    pub width: Width,
}
