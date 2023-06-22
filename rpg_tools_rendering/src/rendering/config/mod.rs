use crate::math::polygon2d::Polygon2d;
use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::eye::EyeConfig;
use crate::rendering::config::head::HeadConfig;
use anyhow::Result;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::color::Color;

pub mod eye;
pub mod head;

#[derive(Debug, PartialEq)]
pub struct RenderConfig {
    pub border: u32,
    pub line_color: WebColor,
    pub line_width: u32,
    pub cut_corners_u: f32,
    pub cut_corners_v: f32,
    pub cut_corners_n: u32,
    pub head: HeadConfig,
    pub eye: EyeConfig,
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

    pub fn get_skin_options(&self, skin: &Skin) -> RenderOptions {
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

    pub fn cut_corners(&self, polygon: &Polygon2d) -> Result<Polygon2d> {
        polygon.cut_corners_n(self.cut_corners_u, self.cut_corners_v, self.cut_corners_n)
    }
}
