use crate::model::color::Color;
use crate::model::size::Size;

/// How does the mouth look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        /// Are the lips painted?
        color: Option<Color>,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
}

/// Does the character have special teeth?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialTeeth {
    None,
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs(Size),
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs(Size),
}

/// The color of the teeth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeethColor {
    White,
    Yellow,
    Brown,
}
