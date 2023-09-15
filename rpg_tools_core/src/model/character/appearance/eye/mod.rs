use crate::model::character::appearance::eye::brow::EyeBrows;
use crate::model::character::appearance::eye::pupil::PupilShape;
use crate::model::character::appearance::eye::shape::EyeShape;
use crate::model::color::Color;
use crate::model::size::Size;
use macro_core::parser::UiParser;
use macro_core::visitor::UI;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod brow;
pub mod pupil;
pub mod shape;

/// How many eyes does the character have?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Eyes {
    None,
    One {
        eye: Eye,
        eyebrows: EyeBrows,
    },
    Two {
        /// Both eyes are identical.
        eye: Eye,
        eyebrows: EyeBrows,
        /// What is the distance between the eyes?
        distance: Size,
    },
}

impl Default for Eyes {
    fn default() -> Self {
        Self::Two {
            eye: Eye::default(),
            eyebrows: EyeBrows::default(),
            distance: Size::Medium,
        }
    }
}

/// How does the eye look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl Eye {
    pub fn get_shape(&self) -> EyeShape {
        match self {
            Eye::Simple { eye_shape, .. } => *eye_shape,
            Eye::Normal { eye_shape, .. } => *eye_shape,
        }
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::Normal {
            eye_shape: EyeShape::default(),
            pupil_shape: PupilShape::Circle,
            pupil_color: Color::Blue,
            background_color: Color::White,
        }
    }
}
