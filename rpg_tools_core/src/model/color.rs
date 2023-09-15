use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// A color defined by a name.
/// See https://en.wikipedia.org/wiki/Web_colors.
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Aqua,
    Black,
    Blue,
    Fuchsia,
    Gray,
    Green,
    Lime,
    Maroon,
    Navy,
    Olive,
    Orange,
    #[default]
    Purple,
    Red,
    SaddleBrown,
    Silver,
    Teal,
    White,
    Yellow,
}
