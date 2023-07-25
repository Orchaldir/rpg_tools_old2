use crate::math::polygon2d::Polygon2d;
use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::ear::EarConfig;
use crate::rendering::config::eye::EyeConfig;
use crate::rendering::config::hair::HairConfig;
use crate::rendering::config::head::HeadConfig;
use crate::rendering::config::mouth::MouthConfig;
use anyhow::Result;
use rpg_tools_core::model::character::appearance::hair::HairColor;
use rpg_tools_core::model::character::appearance::mouth::TeethColor;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::color::Color;

pub mod ear;
pub mod example;
pub mod eye;
pub mod hair;
pub mod head;
pub mod mouth;
pub mod size;

#[derive(Debug, PartialEq)]
pub struct RenderConfig {
    pub border: u32,
    pub line_color: WebColor,
    pub line_width: u32,
    pub cut_corners_u: f32,
    pub cut_corners_v: f32,
    pub cut_corners_n: u32,
    pub hair: HairConfig,
    pub head: HeadConfig,
    pub ear: EarConfig,
    pub eye: EyeConfig,
    pub mouth: MouthConfig,
}

impl RenderConfig {
    pub fn without_line(&self, color: Color) -> RenderOptions {
        let web_color = WebColor::from_color(color);
        RenderOptions::no_line(web_color)
    }

    pub fn get_line_options(&self, width_factor: f32) -> RenderOptions {
        RenderOptions::line(
            self.line_color.clone(),
            (self.line_width as f32 * width_factor) as u32,
        )
    }

    pub fn line_with_color(&self, color: Color, width_factor: f32) -> RenderOptions {
        RenderOptions::line(
            WebColor::from_color(color),
            (self.line_width as f32 * width_factor) as u32,
        )
    }

    pub fn get_hair_options(&self, hair: HairColor) -> RenderOptions {
        RenderOptions::new(
            self.hair.get_color(hair),
            self.line_color.clone(),
            self.line_width,
        )
    }

    pub fn get_skin_options(&self, skin: &Skin) -> RenderOptions {
        RenderOptions::new(
            self.get_skin_color(skin),
            self.line_color.clone(),
            self.line_width,
        )
    }

    pub fn get_teeth_options(&self, color: TeethColor) -> RenderOptions {
        RenderOptions::new(
            self.mouth.get_teeth_color(color),
            WebColor::from_color(Color::Black),
            self.line_width / 10,
        )
    }

    pub fn get_skin_color(&self, skin: &Skin) -> WebColor {
        match skin {
            Skin::Scales(color) => WebColor::from_color(*color),
            Skin::Skin(skin_color) => match skin_color {
                SkinColor::Fair => WebColor::from_rgb(254, 228, 208),
                SkinColor::Light => WebColor::from_rgb(232, 198, 175),
                SkinColor::Medium => WebColor::from_rgb(175, 118, 88),
                SkinColor::Tan => WebColor::from_rgb(156, 89, 60),
                SkinColor::Dark => WebColor::from_rgb(122, 68, 44),
                SkinColor::VeryDark => WebColor::from_rgb(58, 26, 13),
            },
            Skin::ExoticSkin(color) => WebColor::from_color(*color),
        }
    }

    pub fn cut_corners(&self, polygon: &Polygon2d) -> Result<Polygon2d> {
        polygon.cut_corners_n(self.cut_corners_u, self.cut_corners_v, self.cut_corners_n)
    }
}
