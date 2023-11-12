use crate::renderer::color::WebColor;
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::transparency::Transparency;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};

#[derive(Debug, PartialEq)]
pub struct ColorConfig {
    pub skin_fair: WebColor,
    pub skin_light: WebColor,
    pub skin_medium: WebColor,
    pub skin_tan: WebColor,
    pub skin_dark: WebColor,
    pub skin_very_dark: WebColor,
}

impl ColorConfig {
    pub fn get_transparent_color(&self, color: Color, transparency: Transparency) -> WebColor {
        WebColor::transparent(
            color,
            match transparency {
                Transparency::Opaque => 0,
                Transparency::Low => 64,
                Transparency::Medium => 128,
                Transparency::High => 192,
            },
        )
    }

    pub fn get_skin_color(&self, skin: &Skin) -> WebColor {
        match skin {
            Skin::Scales { color } => WebColor::from_color(*color),
            Skin::NormalSkin { color } => match color {
                SkinColor::Fair => self.skin_fair.clone(),
                SkinColor::Light => self.skin_light.clone(),
                SkinColor::Medium => self.skin_medium.clone(),
                SkinColor::Tan => self.skin_tan.clone(),
                SkinColor::Dark => self.skin_dark.clone(),
                SkinColor::VeryDark => self.skin_very_dark.clone(),
            },
            Skin::ExoticSkin { color } => WebColor::from_color(*color),
        }
    }
}
