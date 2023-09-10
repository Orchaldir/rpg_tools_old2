use crate::ui::parser::{get_enum, UiParser};
use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// What kind of moustache?
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoustacheStyle {
    FuManchu,
    #[default]
    Handlebar,
    Pencil,
    Pyramid,
    Toothbrush,
    Walrus,
}
