use crate::renderer::color::WebColor;
use crate::rendering::config::hair::hairline::HairlineConfig;
use crate::rendering::config::hair::short::ShortHairConfig;
use rpg_tools_core::model::character::appearance::hair::HairColor;
use rpg_tools_core::model::color::Color::White;

pub mod hairline;
pub mod short;

#[derive(Debug, PartialEq)]
pub struct HairConfig {
    pub hairline: HairlineConfig,
    pub short: ShortHairConfig,
}

impl HairConfig {
    pub fn get_color(&self, hair: HairColor) -> WebColor {
        match hair {
            HairColor::White => WebColor::from_color(White),
            HairColor::Grey => WebColor::from_rgb(106, 106, 106),
            HairColor::Blond => WebColor::from_rgb(255, 204, 71),
            HairColor::Orange => WebColor::from_rgb(255, 147, 33),
            HairColor::Red => WebColor::from_rgb(255, 0, 0),
            HairColor::Brown => WebColor::from_rgb(90, 56, 37),
            HairColor::Black => WebColor::from_rgb(36, 32, 36),
            HairColor::Exotic(color) => WebColor::from_color(color),
        }
    }
}
