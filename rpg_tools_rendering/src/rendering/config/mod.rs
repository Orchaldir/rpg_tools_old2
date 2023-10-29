use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::ear::EarConfig;
use crate::rendering::config::equipment::belt::BeltConfig;
use crate::rendering::config::equipment::footwear::FootwearConfig;
use crate::rendering::config::equipment::pants::PantsConfig;
use crate::rendering::config::equipment::shirt::ShirtConfig;
use crate::rendering::config::eye::EyeConfig;
use crate::rendering::config::hair::HairConfig;
use crate::rendering::config::head::HeadConfig;
use crate::rendering::config::mouth::MouthConfig;
use rpg_tools_core::model::character::appearance::mouth::TeethColor;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::color::Color;

pub mod body;
pub mod ear;
pub mod equipment;
pub mod example;
pub mod eye;
pub mod hair;
pub mod head;
pub mod mouth;
pub mod size;
pub mod width;

#[derive(Debug, PartialEq)]
pub struct RenderConfig {
    pub border: u32,
    pub line_color: WebColor,
    pub line_width: u32,
    pub thin_line_width: u32,
    pub body: BodyConfig,
    pub ear: EarConfig,
    pub eye: EyeConfig,
    pub hair: HairConfig,
    pub head: HeadConfig,
    pub mouth: MouthConfig,
    pub belt: BeltConfig,
    pub footwear: FootwearConfig,
    pub pants: PantsConfig,
    pub shirt: ShirtConfig,
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

    pub fn get_options(&self, color: Color) -> RenderOptions {
        RenderOptions::new(
            WebColor::from_color(color),
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
            self.thin_line_width,
        )
    }

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
