use crate::model::character::appearance::beard::goatee::GoateeStyle;
use crate::model::character::appearance::beard::moustache::MoustacheStyle;
use crate::model::color::Color;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod goatee;
pub mod moustache;

/// How does the beard look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Beard {
    None,
    Stubble {
        color: Color,
    },
    Moustache {
        moustache: MoustacheStyle,
        color: Color,
    },
    Goatee {
        goatee: GoateeStyle,
        color: Color,
    },
    GoateeAndMoustache {
        moustache: MoustacheStyle,
        goatee: GoateeStyle,
        color: Color,
    },
}
