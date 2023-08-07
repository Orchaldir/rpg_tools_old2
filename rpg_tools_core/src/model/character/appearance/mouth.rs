use crate::model::character::appearance::beard::Beard;
use crate::model::size::Size;
use crate::ui::{UiVisitor, UI};
use serde::{Deserialize, Serialize};
use std::fmt;
use ui_macro::ui;
use TeethColor::*;

/// How does the mouth look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Mouth {
    None,
    /// Like a lamprey's mouth.
    Circle {
        size: Size,
        teeth_color: TeethColor,
    },
    /// Like a human's mouth.
    Normal {
        beard: Beard,
        width: Size,
        teeth: SpecialTeeth,
        teeth_color: TeethColor,
    },
}

/// Does the character have special teeth?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
pub enum SpecialTeeth {
    None,
    /// The 2 lower canine teeth are longer. e.g. orcs
    LowerFangs(Size),
    /// The 2 upper canine teeth are longer. e.g. snakes, vampires
    UpperFangs(Size),
}

/// The color of the teeth.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeethColor {
    White,
    Yellow,
    Brown,
}

impl TeethColor {
    pub fn get_all() -> Vec<TeethColor> {
        vec![White, Yellow, Brown]
    }
}

impl fmt::Display for TeethColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for TeethColor {
    fn from(shape: &str) -> Self {
        match shape {
            "Brown" => Brown,
            "Yellow" => Yellow,
            _ => White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for color in TeethColor::get_all() {
            let string = color.to_string();
            assert_eq!(color, TeethColor::from(&*string));
        }
    }
}
