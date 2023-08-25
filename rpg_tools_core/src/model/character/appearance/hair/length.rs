use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How long is the hair?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairLength {
    Shoulder,
    #[default]
    Waist,
    Hip,
    Knee,
    Ankle,
}
