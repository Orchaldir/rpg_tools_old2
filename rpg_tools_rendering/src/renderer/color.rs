use rpg_tools_core::model::color::Color;

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

    /// Returns the color as string.
    ///
    /// ```
    ///# use rpg_tools_rendering::renderer::color::WebColor;
    /// assert_eq!(WebColor::Name("test".to_string()).to_string(), "test");
    /// assert_eq!(WebColor::from_rgb(255, 128, 0).to_string(), "#ff8000");
    /// ```
    pub fn to_string(&self) -> String {
        match self {
            WebColor::Name(name) => name.clone(),
            WebColor::RBG { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, *g, *b),
        }
    }
}
