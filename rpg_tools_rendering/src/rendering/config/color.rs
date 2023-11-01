use crate::renderer::color::WebColor;
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
