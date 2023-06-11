use crate::model::character::appearance::skin::Skin;

/// How does the humanoid body  look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Body {
    pub shape: BodyShape,
    pub width: BodyWidth,
    pub skin: Skin,
}

/// The body shape is defined by the ratio between shoulders, waist & hips.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

/// How wide is the body?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyWidth {
    Thin,
    Average,
    Wide,
}
