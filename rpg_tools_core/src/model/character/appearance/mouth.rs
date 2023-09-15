use crate::model::character::appearance::beard::Beard;
use crate::model::color::Color;
use crate::model::size::Size;
use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How does the mouth look like?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mouth {
    #[default]
    None,
    /// Like a lamprey's mouth.
    Circle { size: Size, teeth_color: TeethColor },
    Simple {
        beard: Beard,
        width: Size,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
    Female {
        width: Size,
        color: Color,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
}

/// Does the character have special teeth?
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SpecialTeeth {
    #[default]
    None,
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs { size: Size },
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs { size: Size },
}

/// The color of the teeth.
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeethColor {
    #[default]
    White,
    Yellow,
    Brown,
}
