use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What style of long hair?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LongHairStyle {
    Rounded,
    #[default]
    Straight,
    Triangle,
}
