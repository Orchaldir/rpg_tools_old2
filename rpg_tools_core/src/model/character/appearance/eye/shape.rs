use serde::Serialize;

/// What is the shape of the eye?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EyeShape {
    Almond,
    Circle,
    Ellipse,
}
