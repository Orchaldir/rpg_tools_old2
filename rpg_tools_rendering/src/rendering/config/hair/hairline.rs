use crate::rendering::config::size::SizeConfig;

#[derive(Debug, PartialEq)]
pub struct HairlineConfig {
    pub width_round: f32,
    pub width_straight: f32,
    pub width_triangle: f32,
    pub width_widows_peak: f32,
    pub height_widows_peak: f32,
    pub y: SizeConfig,
}
