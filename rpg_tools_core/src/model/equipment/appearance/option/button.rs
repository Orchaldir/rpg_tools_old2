use crate::model::color::Color;
use crate::model::size::Size;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// A button used in clothing.
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Button {
    pub size: Size,
    pub color: Color,
}

/// A column of (buttons)[Button].
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonColumn {
    pub button: Button,
    pub count: u32,
}
