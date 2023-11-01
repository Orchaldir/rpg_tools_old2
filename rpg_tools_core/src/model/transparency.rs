use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// The level of transparency.
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transparency {
    Opaque,
    Low,
    #[default]
    Medium,
    High,
}
