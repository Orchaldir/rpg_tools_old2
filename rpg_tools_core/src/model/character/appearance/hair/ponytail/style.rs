use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What style of ponytail?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PonytailStyle {
    Braid,
    Bubble,
    #[default]
    Straight,
    Wide,
}
