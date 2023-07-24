use crate::model::character::appearance::eye::pupil::PupilShape;
use crate::model::character::appearance::eye::shape::EyeShape;
use crate::model::color::Color;
use crate::model::size::Size;
use serde::Serialize;

pub mod pupil;
pub mod shape;

/// How many eyes does the character have?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Eyes {
    None,
    One {
        eye: Eye,
    },
    Two {
        /// Both eyes are identical.
        eye: Eye,
        /// What is the distance between the eyes?
        distance: Size,
    },
}

/// How does the eye look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
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

impl Default for Eye {
    fn default() -> Self {
        Self::Simple {
            eye_shape: EyeShape::default(),
            color: Color::default(),
        }
    }
}
