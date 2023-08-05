use crate::ui::{UiVisitor, UI};
use serde::{Deserialize, Serialize};
use std::fmt;
use ui_macro::ui;

/// How wide?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Width {
    Thin,
    Average,
    Wide,
}

impl Width {
    pub fn get_all() -> Vec<Width> {
        vec![Width::Thin, Width::Average, Width::Wide]
    }
}

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Width {
    fn from(shape: &str) -> Self {
        match shape {
            "Thin" => Width::Thin,
            "Wide" => Width::Wide,
            _ => Self::Average,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for width in Width::get_all() {
            let string = width.to_string();
            assert_eq!(width, Width::from(&*string));
        }
    }
}
