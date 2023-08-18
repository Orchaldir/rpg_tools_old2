use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How long is the hair?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairLength {
    Shoulder,
    Waist,
    Hip,
    Knee,
    Ankle,
}
