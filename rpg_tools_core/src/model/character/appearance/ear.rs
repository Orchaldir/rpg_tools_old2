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
    },
}

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum EarShape {
    /// Like an elf's ears.
    Pointed(Size),
    Round,
    Square,
}
