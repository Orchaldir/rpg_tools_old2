use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use std::fmt;
use ui_macro::ui;
use EyeShape::*;

/// What is the shape of the eye?
#[derive(ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EyeShape {
    Almond,
    Circle,
    #[default]
    Ellipse,
}

impl EyeShape {
    pub fn get_all() -> Vec<EyeShape> {
        vec![Almond, Circle, Ellipse]
    }
}

impl fmt::Display for EyeShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for EyeShape {
    fn from(shape: &str) -> Self {
        match shape {
            "Almond" => Almond,
            "Circle" => Circle,
            _ => EyeShape::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for shape in EyeShape::get_all() {
            let string = shape.to_string();
            assert_eq!(shape, EyeShape::from(&*string));
        }
    }
}
