use serde::Serialize;
use std::fmt;
use EarShape::*;

/// How many ears does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EarShape {
    /// Like an elf's ears.
    Pointed,
    Round,
    Square,
}

impl EarShape {
    pub fn get_all() -> Vec<EarShape> {
        vec![Pointed, Round, Square]
    }
}

impl fmt::Display for EarShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for EarShape {
    fn from(shape: &str) -> Self {
        match shape {
            "Pointed" => Pointed,
            "Round" => Round,
            _ => Square,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for shape in EarShape::get_all() {
            let string = shape.to_string();
            assert_eq!(shape, EarShape::from(&*string));
        }
    }
}
