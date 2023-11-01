use crate::renderer::color::WebColor;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};

#[derive(Debug, PartialEq)]
pub struct ColorConfig {}

impl ColorConfig {
    pub fn get_skin_color(&self, skin: &Skin) -> WebColor {
        match skin {
            Skin::Scales { color } => WebColor::from_color(*color),
            Skin::NormalSkin { color } => match color {
                SkinColor::Fair => WebColor::from_rgb(254, 228, 208),
                SkinColor::Light => WebColor::from_rgb(232, 198, 175),
                SkinColor::Medium => WebColor::from_rgb(175, 118, 88),
                SkinColor::Tan => WebColor::from_rgb(156, 89, 60),
                SkinColor::Dark => WebColor::from_rgb(122, 68, 44),
                SkinColor::VeryDark => WebColor::from_rgb(58, 26, 13),
            },
            Skin::ExoticSkin { color } => WebColor::from_color(*color),
        }
    }
}
