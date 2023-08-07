use crate::model::character::appearance::beard::Beard;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How does the mouth look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mouth {
    None,
    /// Like a lamprey's mouth.
    Circle {
        size: Size,
        teeth_color: TeethColor,
    },
    /// Like a human's mouth.
    Normal {
        beard: Beard,
        width: Size,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
}

/// Does the character have special teeth?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
pub enum SpecialTeeth {
    None,
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs(Size),
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs(Size),
}

/// The color of the teeth.
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeethColor {
    #[default]
    White,
    Yellow,
    Brown,
}
