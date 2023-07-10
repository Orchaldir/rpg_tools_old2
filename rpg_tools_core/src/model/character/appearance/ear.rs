use crate::model::character::appearance::Size;
use crate::model::width::Width;

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ears {
    None,
    /// Like a human's ears.
    Normal {
        size: Size,
        width: Width,
    },
}
