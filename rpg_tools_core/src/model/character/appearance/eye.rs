use crate::model::color::Color;

/// How many the eyes does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eyes {
    None,
    One(Eye),
    Two { eye: Eye, distance: EyeDistance },
}

/// How does the eye look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eye {
    Simple {
        eye_shape: EyeShape,
        color: Color,
    },
    Normal {
        eye_shape: EyeShape,
        pupil_shape: PupilShape,
        pupil_color: Color,
        background_color: Color,
    },
}

/// What is the distance between the eyes?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EyeDistance {
    Low,
    Medium,
    High,
}

/// What is the shape of the eye?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EyeShape {
    Almond,
    Circle,
    Ellipse,
}

/// What is the shape of the pupil?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PupilShape {
    Circle,
    VerticalSlit,
    HorizontalSlit,
}
