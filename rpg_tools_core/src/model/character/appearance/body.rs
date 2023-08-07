use crate::model::character::appearance::skin::Skin;
use crate::model::width::Width;
use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};
use std::fmt;
use BodyShape::*;

/// How does the humanoid body look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Body {
    pub shape: BodyShape,
    /// How wide is the body?
    pub width: Width,
    pub skin: Skin,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            shape: Rectangle,
            width: Width::Average,
            skin: Skin::default(),
        }
    }
}

/// The body shape is defined by the ratio between shoulders, waist & hips.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyShape {
    /// The body gets wider from the shoulders to the hips.
    Fat,
    /// The shoulders & hips are wider than the waist.
    Hourglass,
    /// The body gets wider from the hips to the shoulders.
    Muscular,
    /// The shoulders, waist & hips are equally wide.
    Rectangle,
}

impl BodyShape {
    pub fn get_all() -> Vec<Self> {
        vec![Fat, Hourglass, Muscular, Rectangle]
    }
}

impl fmt::Display for BodyShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for BodyShape {
    fn from(shape: &str) -> Self {
        match shape {
            "Fat" => Fat,
            "Hourglass" => Hourglass,
            "Muscular" => Muscular,
            _ => Rectangle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for shape in BodyShape::get_all() {
            let string = shape.to_string();
            assert_eq!(shape, BodyShape::from(&*string));
        }
    }
}
