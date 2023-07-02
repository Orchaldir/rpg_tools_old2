use crate::renderer::color::WebColor;
use rpg_tools_core::model::character::appearance::hair::HairColor;
use rpg_tools_core::model::character::appearance::Size;
use rpg_tools_core::model::color::Color::White;

#[derive(Debug, PartialEq)]
pub struct HairlineConfig {
    pub width_round: f32,
    pub width_straight: f32,
    pub width_triangle: f32,
    pub width_widows_peak: f32,
    pub height_widows_peak: f32,
    pub y_low: f32,
    pub y_medium: f32,
    pub y_high: f32,
}

impl HairlineConfig {
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

    pub fn get_y(&self, size: Size) -> f32 {
        match size {
            Size::Low => self.y_low,
            Size::Medium => self.y_medium,
            Size::High => self.y_high,
        }
    }
}
