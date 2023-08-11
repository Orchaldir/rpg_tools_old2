use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What is the style of the eyebrow?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyebrowStyle {
    #[default]
    Even,
    Managerial,
    Winged,
}
