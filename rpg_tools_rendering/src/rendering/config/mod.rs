use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::color::ColorConfig;
use crate::rendering::config::ear::EarConfig;
use crate::rendering::config::equipment::belt::BeltConfig;
use crate::rendering::config::equipment::eyewear::EyewearConfig;
use crate::rendering::config::equipment::footwear::FootwearConfig;
use crate::rendering::config::equipment::outerwear::OuterwearConfig;
use crate::rendering::config::equipment::pants::PantsConfig;
use crate::rendering::config::equipment::shirt::ShirtConfig;
use crate::rendering::config::eye::EyeConfig;
use crate::rendering::config::hair::HairConfig;
use crate::rendering::config::head::HeadConfig;
use crate::rendering::config::mouth::MouthConfig;
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::character::appearance::mouth::TeethColor;
use rpg_tools_core::model::character::appearance::skin::Skin;

pub mod body;
pub mod color;
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
    pub color: ColorConfig,
    pub body: BodyConfig,
    pub ear: EarConfig,
    pub eye: EyeConfig,
    pub hair: HairConfig,
    pub head: HeadConfig,
    pub mouth: MouthConfig,
    pub belt: BeltConfig,
    pub eyewear: EyewearConfig,
    pub footwear: FootwearConfig,
    pub outerwear: OuterwearConfig,
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
            self.color.get_skin_color(skin),
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
}
