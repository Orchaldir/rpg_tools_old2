use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_core::visitor::UI;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What is the style of the eyebrow?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyebrowStyle {
    /// The eyebrows have a constant thickness.
    #[default]
    Even,
    /// The eyebrows are thicker on the side of the face than in the center.
    Managerial,
    /// The eyebrows are thicker in the center of the face than on the side.
    Winged,
}
