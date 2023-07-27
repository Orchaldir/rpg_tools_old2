use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use ui_macro::ui;

/// How does the mouth look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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
        width: Size,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
}

/// Does the character have special teeth?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum SpecialTeeth {
    None,
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs(Size),
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs(Size),
}

/// The color of the teeth.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TeethColor {
    White,
    Yellow,
    Brown,
}
