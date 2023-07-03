use rpg_tools_core::model::character::appearance::{Side, Size};

#[derive(Debug, PartialEq)]
pub struct ShortHairConfig {
    pub side_part_offset: f32,
    pub y_flat_top_low: f32,
    pub y_flat_top_medium: f32,
    pub y_flat_top_high: f32,
    pub y_middle_part_low: f32,
    pub y_middle_part_medium: f32,
    pub y_middle_part_high: f32,
    pub inner_width: f32,
    pub hairline_width: f32,
}

impl ShortHairConfig {
    pub fn get_side_part_horizontal(&self, side: Side, forehead_width: f32) -> f32 {
        0.5 + forehead_width * self.side_part_offset * side.get_sign()
    }

    pub fn get_flattop_y(&self, size: Size) -> f32 {
        match size {
            Size::Low => self.y_flat_top_low,
            Size::Medium => self.y_flat_top_medium,
            Size::High => self.y_flat_top_high,
        }
    }

    pub fn get_middle_y(&self, size: Size) -> f32 {
        match size {
            Size::Low => self.y_middle_part_low,
            Size::Medium => self.y_middle_part_medium,
            Size::High => self.y_middle_part_high,
        }
    }
}
