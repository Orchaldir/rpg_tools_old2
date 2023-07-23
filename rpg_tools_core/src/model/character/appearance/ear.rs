use crate::model::size::Size;
use serde::Serialize;

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Ears {
    None,
    /// Like a human's ears.
    Normal {
        shape: EarShape,
        size: Size,
    },
}

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EarShape {
    /// Like an elf's ears.
    Pointed,
    Round,
    Square,
}
