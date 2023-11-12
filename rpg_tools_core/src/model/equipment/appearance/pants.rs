use crate::model::appearance::color::Color;
use crate::model::equipment::appearance::belt::Belt;
use macro_convert::Convert;
use macro_core::visitor::visit_option;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The pants of the [`character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pants {
    pub style: PantsStyle,
    pub color: Color,
    pub belt: Option<Belt>,
}

/// What style of [`pants`](Pants)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PantsStyle {
    Balloon,
    Bermuda,
    HotPants,
    #[default]
    Regular,
    Shorts,
}
