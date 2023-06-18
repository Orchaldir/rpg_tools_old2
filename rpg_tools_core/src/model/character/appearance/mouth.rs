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
    Fangs,
    Needle,
    Normal,
    Triangular,
}

/// Does the character have special teeth?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialTeeth {
    None,
    VampireFangs,
    Tusks,
}

/// The color of the the teeth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeethColor {
    White,
    Yellow,
    Brown,
    Black,
}
