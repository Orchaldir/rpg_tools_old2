use rpg_tools_core::model::size::Size;

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
    pub fn get_y(&self, size: Size) -> f32 {
        match size {
            Size::Low => self.y_low,
            Size::Medium => self.y_medium,
            Size::High => self.y_high,
        }
    }
}
