use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;

/// The rendering config of the [`eyebrows`](rpg_tools_core::model::character::appearance::eye::brow::EyeBrows).
#[derive(Debug, PartialEq)]
pub struct EyebrowConfig {
    pub width: WidthConfig,
}

impl EyebrowConfig {
    /// The eyebrow thickness closer to the center of the face.
    pub fn get_inner_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Bushy | EyebrowStyle::Winged => self.width.wide,
            EyebrowStyle::Even => self.width.average,
            EyebrowStyle::Thin => self.width.thin,
        }
    }

    /// The eyebrow thickness closer to the side of the face.
    pub fn get_outer_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Bushy => self.width.wide,
            EyebrowStyle::Even => self.width.average,
            EyebrowStyle::Thin | EyebrowStyle::Winged => self.width.thin,
        }
    }
}
