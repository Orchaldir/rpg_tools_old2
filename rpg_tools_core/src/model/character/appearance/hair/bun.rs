use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What style of hair bun?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BunStyle {
    Low,
    #[default]
    High,
    Twin,
}
