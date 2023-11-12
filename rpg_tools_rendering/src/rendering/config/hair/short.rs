use crate::rendering::config::size::SizeConfig;
use rpg_tools_core::model::appearance::side::Side;

#[derive(Debug, PartialEq)]
pub struct ShortHairConfig {
    pub side_part_offset: f32,
    pub y_flat_top: SizeConfig,
    pub y_middle_part: SizeConfig,
    pub inner_width: f32,
    pub hairline_width: f32,
}

impl ShortHairConfig {
    pub fn get_side_part_horizontal(&self, side: Side, forehead_width: f32) -> f32 {
        0.5 + forehead_width * self.side_part_offset * side.get_sign_from_front()
    }
}
