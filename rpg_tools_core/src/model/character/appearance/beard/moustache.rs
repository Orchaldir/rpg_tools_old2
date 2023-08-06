use crate::ui::{UiVisitor, UI};
use serde::{Deserialize, Serialize};
use ui_macro::ui;

/// What kind of moustache?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MoustacheStyle {
    FuManchu,
    Handlebar,
    Pencil,
    Pyramid,
    Toothbrush,
    Walrus,
}
