use rpg_tools_core::model::character::appearance::Side;

#[derive(Debug, PartialEq)]
pub struct ShortHairConfig {
    pub side_part_offset: f32,
}

impl ShortHairConfig {
    pub fn get_side_part_horizontal(&self, side: Side, forehead_width: f32) -> f32 {
        0.5 + forehead_width * self.side_part_offset * side.get_sign()
    }
}
