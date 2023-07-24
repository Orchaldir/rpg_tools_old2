use serde::Serialize;

/// What is the shape of the pupil?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum PupilShape {
    Circle,
    VerticalSlit,
    HorizontalSlit,
}
