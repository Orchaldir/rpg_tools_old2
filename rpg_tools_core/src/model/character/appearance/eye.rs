use crate::model::color::Color;

/// How do the eyes look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eyes {
    None,
    One(Eye),
    Two { left: Eye, right: Eye },
}

impl Eyes {
    pub fn two_identical(eye: Eye) -> Self {
        Self::Two {
            left: eye,
            right: eye,
        }
    }
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
        iris_color: Color,
        background_color: Color,
    },
}

/// What is the shape of the eye?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EyeShape {
    Almond,
    Ellipse,
    Round,
}

/// What is the shape of the pupil?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PupilShape {
    Round,
    VerticalSlit,
    HorizontalSlit,
}
