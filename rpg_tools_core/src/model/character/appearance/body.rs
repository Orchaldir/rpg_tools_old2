use crate::model::character::appearance::skin::Skin;
use crate::model::width::Width;
use serde::Serialize;

/// How does the humanoid body look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Body {
    pub shape: BodyShape,
    /// How wide is the body?
    pub width: Width,
    pub skin: Skin,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            shape: BodyShape::Rectangle,
            width: Width::Average,
            skin: Skin::default(),
        }
    }
}

/// The body shape is defined by the ratio between shoulders, waist & hips.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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
