use crate::model::color::Color;
use crate::ui::parser::{get_enum, UiParser};
use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The skin of a [`Character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
pub enum Skin {
    Scales(Color),
    Skin(SkinColor),
    ExoticSkin(Color),
}

impl Default for Skin {
    fn default() -> Self {
        Self::ExoticSkin(Color::Aqua)
    }
}

/// The skin color of a [`Character`](crate::model::character::Character).
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkinColor {
    Fair,
    Light,
    #[default]
    Medium,
    Tan,
    Dark,
    VeryDark,
}
