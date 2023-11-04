use crate::model::color::Color;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// A cloak is an [`outerwear`](Outerwear).
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cloak {
    pub status: CloakStatus,
    pub outer_color: Color,
    pub inner_color: Color,
}

impl Cloak {
    pub fn get_color(&self, is_outer: bool) -> Color {
        if is_outer {
            self.outer_color
        } else {
            self.inner_color
        }
    }
}

/// How long is the [`outerwear`](Outerwear)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloakStatus {
    #[default]
    Closed,
    Open,
    Backwards,
}
