use crate::model::character::appearance::Size;
use crate::model::color::Color;

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
        height: Size,
        /// Are the lips painted?
        color: Option<Color>,
        teeth: Teeth,
        teeth_color: TeethColor,
    },
}

/// How do the teeth look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teeth {
    Fangs,
    Normal,
    Needle,
    VampireFangs,
    Triangular,
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
