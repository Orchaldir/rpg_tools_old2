use crate::model::character::appearance::skin::Skin;

/// How does the head look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Head {
    shape: HeadShape,
    skin: Skin,
}

/// What is the shape of the head?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadShape {
    Geometric(GeometricHeadShape),
    Realistic(RealisticHeadShape),
}

/// What geometric shape does the head have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometricHeadShape {
    Circle,
    Square,
}

/// What geometric shape does the head have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealisticHeadShape {
    Oval,
    Rectangle,
    Round,
    Square,
    TriangleDown,
    TriangleUp,
}
