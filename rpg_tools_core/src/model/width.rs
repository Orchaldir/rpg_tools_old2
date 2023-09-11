use macro_convert::Convert;
use macro_core::parser::{get_enum, UiParser};
use macro_core::visitor::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How wide?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Width {
    Thin,
    #[default]
    Average,
    Wide,
}
