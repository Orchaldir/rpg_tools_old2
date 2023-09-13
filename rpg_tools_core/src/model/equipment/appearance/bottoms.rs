use crate::model::color::Color;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// Clothing for the lower body.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Bottoms {
    Bermuda { color: Color },
    HotPants { color: Color },
    Pants { color: Color },
    Shorts { color: Color },
}
