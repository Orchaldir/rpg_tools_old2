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
#[serde(tag = "t", content = "c")]
pub enum HeadShape {
    Oval,
    Round,
    RoundedRectangle,
    RoundedSquare,
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
            Self::RoundedRectangle,
            Self::RoundedSquare,
            Self::TriangleDown,
            Self::TriangleUp,
        ]
    }
}
