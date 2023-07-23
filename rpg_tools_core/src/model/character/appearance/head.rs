use crate::model::character::appearance::ear::Ears;
use crate::model::character::appearance::eye::Eyes;
use crate::model::character::appearance::hair::Hair;
use crate::model::character::appearance::mouth::Mouth;
use crate::model::character::appearance::skin::Skin;
use serde::Serialize;
use std::fmt;

/// How does the head look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Head {
    pub ears: Ears,
    pub eyes: Eyes,
    pub hair: Hair,
    pub mouth: Mouth,
    pub shape: HeadShape,
    pub skin: Skin,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            ears: Ears::None,
            eyes: Eyes::None,
            hair: Hair::None,
            mouth: Mouth::None,
            shape: HeadShape::default(),
            skin: Skin::default(),
        }
    }
}

/// What is the shape of the head?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum HeadShape {
    Oval,
    Rectangle,
    Round,
    Square,
    TriangleDown,
    TriangleUp,
}

impl Default for HeadShape {
    fn default() -> Self {
        Self::Round
    }
}

impl HeadShape {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Oval,
            Self::Round,
            Self::Rectangle,
            Self::Square,
            Self::TriangleDown,
            Self::TriangleUp,
        ]
    }
}

impl fmt::Display for HeadShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for HeadShape {
    fn from(shape: &str) -> Self {
        match shape {
            "Oval" => Self::Oval,
            "Rectangle" => Self::Rectangle,
            "Square" => Self::Square,
            "TriangleDown" => Self::TriangleDown,
            "TriangleUp" => Self::TriangleUp,
            _ => Self::Round,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for shape in HeadShape::get_all() {
            let string = shape.to_string();
            assert_eq!(shape, HeadShape::from(&*string));
        }
    }
}
