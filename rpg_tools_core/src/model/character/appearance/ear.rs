use crate::model::character::appearance::Size;

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ears {
    None,
    /// Like a human's ears.
    Normal {
        shape: EarShape,
    },
}

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EarShape {
    /// Like an elf's ears.
    Pointed(Size),
    Round,
    Square,
}
