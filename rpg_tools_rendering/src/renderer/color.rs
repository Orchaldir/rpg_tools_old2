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
    /// Returns a new RGB color.
    pub fn from_color(color: Color) -> WebColor {
        WebColor::Name(color.to_string())
    }

    /// Returns a new RGB color.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> WebColor {
        WebColor::RBG { r, g, b }
    }

    /// Returns a new RGBA color.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> WebColor {
        WebColor::RBGA { r, g, b, a }
    }

    /// Converts a string to a color, if possible:
    ///
    /// ```
    ///# use rpg_tools_rendering::renderer::color::WebColor;
    /// assert_eq!(WebColor::convert("#FFA500").unwrap(), WebColor::from_rgb(255, 165, 0));
    /// assert_eq!(WebColor::convert("#FFA50040").unwrap(), WebColor::from_rgba(255, 165, 0, 64));
    /// ```
    pub fn convert(hex_code: &str) -> Option<WebColor> {
        if !hex_code.starts_with('#') {
            return None;
        } else if hex_code.len() != 7 && hex_code.len() != 9 {
            return None;
        }

        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16).ok()?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16).ok()?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16).ok()?;

        if hex_code.len() == 7 {
            return Some(WebColor::from_rgb(r, g, b));
        }

        let a: u8 = u8::from_str_radix(&hex_code[7..9], 16).ok()?;

        Some(WebColor::from_rgba(r, g, b, a))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_empty_string() {
        assert!(WebColor::convert("").is_none());
    }

    #[test]
    fn test_from_string_invalid_start() {
        assert!(WebColor::convert("FFA500").is_none());
    }

    #[test]
    fn test_from_string_wrong_length() {
        assert!(WebColor::convert("#").is_none());
        assert!(WebColor::convert("#FF").is_none());
        assert!(WebColor::convert("#FFA5").is_none());
        assert!(WebColor::convert("#FFA50").is_none());
        assert!(WebColor::convert("#FFA500FFA5").is_none());
    }

    #[test]
    fn test_from_string_ignore_case() {
        let orange = WebColor::from_rgb(255, 165, 0);
        assert_eq!(WebColor::convert("#FFA500").unwrap(), orange);
        assert_eq!(WebColor::convert("#ffa500").unwrap(), orange);
    }
}
