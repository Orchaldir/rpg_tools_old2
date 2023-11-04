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

/// How long is the [`outerwear`](Outerwear)?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloakStatus {
    #[default]
    Closed,
    Open,
    Backwards,
}
