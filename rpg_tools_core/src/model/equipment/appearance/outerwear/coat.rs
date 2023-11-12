use crate::model::appearance::color::Color;
use crate::model::equipment::appearance::belt::Belt;
use crate::model::equipment::appearance::option::button::ButtonColumn;
use crate::model::equipment::appearance::option::neckline::Neckline;
use crate::model::equipment::appearance::option::sleeve::SleeveStyle;
use crate::model::equipment::appearance::outerwear::OuterwearLength;
use macro_core::visitor::visit_option;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// A coat is an [`outerwear`](Outerwear).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coat {
    pub sleeve: SleeveStyle,
    pub length: OuterwearLength,
    pub neckline: Neckline,
    pub closing: ClosingOption,
    pub color: Color,
    pub belt: Option<Belt>,
}

/// How is the [`outerwear`](Outerwear) closed?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClosingOption {
    #[default]
    None,
    SingleBreasted {
        buttons: ButtonColumn,
    },
    DoubleBreasted {
        buttons: ButtonColumn,
    },
    Zipper {
        color: Color,
    },
}
