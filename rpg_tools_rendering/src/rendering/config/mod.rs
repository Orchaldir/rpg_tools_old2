use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::head::HeadConfig;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};

pub mod head;

#[derive(Debug, PartialEq)]
pub struct RenderConfig {
    pub border: u32,
    pub line_color: WebColor,
    pub line_width: u32,
    pub head: HeadConfig,
}

impl RenderConfig {
    pub fn get_options(&self, skin: &Skin) -> RenderOptions {
        RenderOptions::new(
            self.get_color(skin),
            self.line_color.clone(),
            self.line_width,
        )
    }

    pub fn get_color(&self, skin: &Skin) -> WebColor {
        match skin {
            Skin::Scales(color) => WebColor::from_color(*color),
            Skin::Skin(skin_color) => match skin_color {
                SkinColor::Fair => WebColor::from_rgb(254, 228, 208),
                SkinColor::Light => WebColor::from_rgb(232, 198, 175),
                SkinColor::Medium => WebColor::from_rgb(175, 118, 88),
                SkinColor::Tan => WebColor::from_rgb(156, 89, 60),
                SkinColor::Dark => WebColor::from_rgb(122, 68, 44),
                SkinColor::VeryDark => WebColor::from_rgb(58, 26, 13),
                SkinColor::Exotic(color) => WebColor::from_color(*color),
            },
        }
    }
}
