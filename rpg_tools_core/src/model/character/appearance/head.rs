use crate::model::character::appearance::ear::Ears;
use crate::model::character::appearance::eye::Eyes;
use crate::model::character::appearance::hair::Hair;
use crate::model::character::appearance::mouth::Mouth;
use crate::model::character::appearance::skin::Skin;
use serde::Serialize;

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
pub enum HeadShape {
    Geometric(GeometricHeadShape),
    Realistic(RealisticHeadShape),
}

impl Default for HeadShape {
    fn default() -> Self {
        Self::Geometric(GeometricHeadShape::Circle)
    }
}

/// What geometric shape does the head have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum GeometricHeadShape {
    Circle,
    Square,
}

/// What geometric shape does the head have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum RealisticHeadShape {
    Oval,
    Rectangle,
    Round,
    Square,
    TriangleDown,
    TriangleUp,
}

impl RealisticHeadShape {
    pub fn get_all() -> Vec<Self> {
        vec![
            Self::Oval,
            Self::Rectangle,
            Self::Round,
            Self::Square,
            Self::TriangleDown,
            Self::TriangleUp,
        ]
    }
}
