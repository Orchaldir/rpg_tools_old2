use serde::Serialize;
use std::fmt;
use Color::*;

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
    SaddleBrown,
    Silver,
    Teal,
    White,
    Yellow,
}

impl Color {
    pub fn get_all() -> Vec<Color> {
        vec![
            Aqua,
            Black,
            Blue,
            SaddleBrown,
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
            "Aqua" => Aqua,
            "Black" => Black,
            "Blue" => Blue,
            "Fuchsia" => Fuchsia,
            "Gray" => Gray,
            "Green" => Green,
            "Lime" => Lime,
            "Maroon" => Maroon,
            "Navy" => Navy,
            "Olive" => Olive,
            "Orange" => Orange,
            "Red" => Red,
            "SaddleBrown" => SaddleBrown,
            "Silver" => Silver,
            "Teal" => Teal,
            "White" => White,
            "Yellow" => Yellow,
            _ => Purple,
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
