use crate::model::character::appearance::beard::moustache::MoustacheStyle;
use crate::model::color::Color;
use crate::ui::{UiVisitor, UI};
use serde::{Deserialize, Serialize};
use ui_macro::ui;

pub mod moustache;

/// How does the beard look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Beard {
    None,
    Stubble {
        color: Color
    },
    Moustache {
        style: MoustacheStyle,
        color: Color
    },
}
