use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};
use std::fmt;
use MoustacheStyle::*;

/// What kind of moustache?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoustacheStyle {
    FuManchu,
    Handlebar,
    Pencil,
    Pyramid,
    Toothbrush,
    Walrus,
}

impl MoustacheStyle {
    pub fn get_all() -> Vec<MoustacheStyle> {
        vec![FuManchu, Handlebar, Pencil, Pyramid, Toothbrush, Walrus]
    }
}

impl fmt::Display for MoustacheStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for MoustacheStyle {
    fn from(shape: &str) -> Self {
        match shape {
            "FuManchu" => FuManchu,
            "Pencil" => Pencil,
            "Pyramid" => Pyramid,
            "Toothbrush" => Toothbrush,
            "Walrus" => Walrus,
            _ => Handlebar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for style in MoustacheStyle::get_all() {
            let string = style.to_string();
            assert_eq!(style, MoustacheStyle::from(&*string));
        }
    }
}
