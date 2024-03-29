use macro_convert::Convert;
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
