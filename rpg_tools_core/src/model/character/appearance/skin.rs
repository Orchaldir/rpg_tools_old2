use crate::model::appearance::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The skin of a [`Character`](crate::model::character::Character).
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Skin {
    Scales { color: Color },
    NormalSkin { color: SkinColor },
    ExoticSkin { color: Color },
}

impl Default for Skin {
    fn default() -> Self {
        Skin::normal(SkinColor::default())
    }
}

impl Skin {
    pub fn exotic(color: Color) -> Skin {
        Self::ExoticSkin { color }
    }

    pub fn normal(color: SkinColor) -> Skin {
        Self::NormalSkin { color }
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
