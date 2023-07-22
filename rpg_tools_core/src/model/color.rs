use serde::Serialize;
use std::fmt;

/// A color defined by a name.
/// See https://en.wikipedia.org/wiki/Web_colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Color {
    Aqua,
    Black,
    Blue,
    Fuchsia,
    Gray,
    Green,
    Lime,
    Maroon,
    Navy,
    Olive,
    Orange,
    Purple,
    Red,
    Silver,
    Teal,
    White,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
