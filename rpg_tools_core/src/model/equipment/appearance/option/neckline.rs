use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What style of neckline?
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Neckline {
    Boat,
    Crew,
    DeepV,
    #[default]
    None,
    Scoop,
    V,
    VeryDeppV,
}
