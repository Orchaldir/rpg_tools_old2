use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The relative size of something.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn get_all() -> Vec<Size> {
        vec![Size::Small, Size::Medium, Size::Large]
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Size {
    fn from(shape: &str) -> Self {
        match shape {
            "Small" => Size::Small,
            "Large" => Size::Large,
            _ => Self::Medium,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for size in Size::get_all() {
            let string = size.to_string();
            assert_eq!(size, Size::from(&*string));
        }
    }
}
