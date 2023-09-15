use crate::model::character::appearance::ear::Ears;
use crate::model::character::appearance::eye::Eyes;
use crate::model::character::appearance::hair::Hair;
use crate::model::character::appearance::mouth::Mouth;
use crate::model::character::appearance::skin::Skin;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How does the head look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Head {
    pub ears: Ears,
    pub eyes: Eyes,
    pub hair: Hair,
    pub mouth: Mouth,
    pub shape: HeadShape,
    pub skin: Skin,
}

impl Head {
    /// Mirrors along the center axis.
    pub fn mirror(&self) -> Self {
        Self {
            hair: self.hair.mirror(),
            ..*self
        }
    }
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
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeadShape {
    Oval,
    Rectangle,
    #[default]
    Round,
    Square,
    TriangleDown,
    TriangleUp,
}
