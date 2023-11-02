use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What style of sleeves?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleeveStyle {
    #[default]
    Long,
    None,
    Short,
}
