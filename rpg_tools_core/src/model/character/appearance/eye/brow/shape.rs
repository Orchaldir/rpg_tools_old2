use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What is the shape of the eyebrow?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyebrowShape {
    Angled,
    #[default]
    Curved,
    Straight,
}
