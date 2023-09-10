use crate::ui::parser::{get_enum, UiParser};
use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What kind of goatee?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoateeStyle {
    GoatPatch,
    #[default]
    Goatee,
    SoulPatch,
    VanDyke,
}
