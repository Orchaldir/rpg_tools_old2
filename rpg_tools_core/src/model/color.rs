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

impl Color {
    pub fn get_all() -> Vec<Color> {
        vec![
            Color::Aqua,
            Color::Black,
            Color::Blue,
            Color::Fuchsia,
            Color::Gray,
            Color::Green,
            Color::Lime,
            Color::Maroon,
            Color::Navy,
            Color::Olive,
            Color::Orange,
            Color::Purple,
            Color::Red,
            Color::Silver,
            Color::Teal,
            Color::White,
            Color::Yellow,
        ]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Color {
    fn from(shape: &str) -> Self {
        match shape {
            "Aqua" => Color::Aqua,
            "Black" => Color::Black,
            "Blue" => Color::Blue,
            "Fuchsia" => Color::Fuchsia,
            "Gray" => Color::Gray,
            "Green" => Color::Green,
            "Lime" => Color::Lime,
            "Maroon" => Color::Maroon,
            "Navy" => Color::Navy,
            "Olive" => Color::Olive,
            "Orange" => Color::Orange,
            "Red" => Color::Red,
            "Silver" => Color::Silver,
            "Teal" => Color::Teal,
            "White" => Color::White,
            "Yellow" => Color::Yellow,
            _ => Self::Purple,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for color in Color::get_all() {
            let string = color.to_string();
            assert_eq!(color, Color::from(&*string));
        }
    }
}
