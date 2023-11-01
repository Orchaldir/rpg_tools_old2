use rpg_tools_core::model::color::Color;
use std::fmt::{Display, Formatter};

/// A color supported by svg.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebColor {
    /// See https://en.wikipedia.org/wiki/Web_colors.
    Name(String),
    Transparent(String, u8),
    RBG {
        r: u8,
        g: u8,
        b: u8,
    },
    RBGA {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    },
}

impl WebColor {
    /// Returns a new web color.
    pub fn from_color(color: Color) -> WebColor {
        WebColor::Name(color.to_string())
    }

    /// Returns a new transparent web color.
    pub fn transparent(color: Color, transparency: u8) -> WebColor {
        WebColor::Transparent(color.to_string(), transparency)
    }

    /// Returns a new RGB color.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> WebColor {
        WebColor::RBG { r, g, b }
    }

    /// Returns a new RGBA color.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> WebColor {
        WebColor::RBGA { r, g, b, a }
    }
}

impl Display for WebColor {
    /// Returns the color as string.
    ///
    /// ```
    ///# use rpg_tools_rendering::renderer::color::WebColor;
    /// assert_eq!(WebColor::Name("test".to_string()).to_string(), "test");
    /// assert_eq!(WebColor::Transparent("test".to_string(),128).to_string(), "test|128");
    /// assert_eq!(WebColor::from_rgb(255, 128, 0).to_string(), "#ff8000");
    /// assert_eq!(WebColor::from_rgba(255, 128, 0, 64).to_string(), "#ff800040");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebColor::Name(name) => write!(f, "{}", name.clone()),
            WebColor::Transparent(name, transparency) => {
                write!(f, "{}|{}", name.clone(), *transparency)
            }
            WebColor::RBG { r, g, b } => write!(f, "#{:02x}{:02x}{:02x}", *r, *g, *b),
            WebColor::RBGA { r, g, b, a } => write!(f, "#{:02x}{:02x}{:02x}{:02x}", *r, *g, *b, *a),
        }
    }
}
