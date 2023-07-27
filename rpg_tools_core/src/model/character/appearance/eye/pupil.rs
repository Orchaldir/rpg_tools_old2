use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use std::fmt;
use ui_macro::ui;
use PupilShape::*;

/// What is the shape of the pupil?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum PupilShape {
    Circle,
    HorizontalSlit,
    VerticalSlit,
}

impl PupilShape {
    pub fn get_all() -> Vec<PupilShape> {
        vec![Circle, HorizontalSlit, VerticalSlit]
    }
}

impl fmt::Display for PupilShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for PupilShape {
    fn from(shape: &str) -> Self {
        match shape {
            "HorizontalSlit" => HorizontalSlit,
            "VerticalSlit" => VerticalSlit,
            _ => Circle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for shape in PupilShape::get_all() {
            let string = shape.to_string();
            assert_eq!(shape, PupilShape::from(&*string));
        }
    }
}
