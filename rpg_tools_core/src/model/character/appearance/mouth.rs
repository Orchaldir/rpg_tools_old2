use crate::model::character::appearance::Size;
use crate::model::color::Color;

/// How does the mouth look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mouth {
    None,
    /// Like a lamprey's mouth.
    Circle {
        size: Size,
        teeth: TeethType,
        teeth_color: TeethColor,
    },
    /// Like a human's mouth.
    Normal {
        width: Size,
        height: Size,
        /// Are the lips painted?
        color: Option<Color>,
        teeth: HumanoidTeeth,
    },
}

/// How does the teeth of a humanoid look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HumanoidTeeth {
    pub teeth: TeethType,
    pub special: SpecialTeeth,
    pub color: TeethColor,
}

/// How do the teeth look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeethType {
    Needle,
    Rectangle,
    Round,
    Triangle,
}

/// Does the character have special teeth?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialTeeth {
    None,
    /// All 4 canine teeth are longer. e.g. wolves
    Fangs(Size),
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs(Size),
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs(Size),
}

/// The color of the the teeth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeethColor {
    White,
    Yellow,
    Brown,
    Black,
}
