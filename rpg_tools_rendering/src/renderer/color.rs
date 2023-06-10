use rpg_tools_core::model::color::Color;
use std::fmt::{Display, Formatter};

/// A valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebColor {
    Name(String),
    RBG { r: u8, g: u8, b: u8 },
}

impl WebColor {
    /// Returns a new RGB color.
    pub fn from_color(color: Color) -> WebColor {
        WebColor::Name(color.to_string())
    }

    /// Returns a new RGB color.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> WebColor {
        WebColor::RBG { r, g, b }
    }
}

impl Display for WebColor {
    /// Returns the color as string.
    ///
    /// ```
    ///# use rpg_tools_rendering::renderer::color::WebColor;
    /// assert_eq!(WebColor::Name("test".to_string()).to_string(), "test");
    /// assert_eq!(WebColor::from_rgb(255, 128, 0).to_string(), "#ff8000");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebColor::Name(name) => write!(f, "{}", name.clone()),
            WebColor::RBG { r, g, b } => write!(f, "#{:02x}{:02x}{:02x}", r, *g, *b),
        }
    }
}
