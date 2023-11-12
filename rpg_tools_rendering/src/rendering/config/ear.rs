use crate::math::aabb2d::AABB;
use crate::rendering::config::size::SizeConfig;
use rpg_tools_core::model::appearance::size::Size;

/// The rendering config of the [`ears`](rpg_tools_core::model::character::appearance::ear::Ears).
#[derive(Debug, PartialEq)]
pub struct EarConfig {
    pub normal_offset: f32,
    pub normal_top_x: f32,
    pub normal_width: f32,
    pub pointed_length: SizeConfig,
    pub round_factor: SizeConfig,
}

impl EarConfig {
    pub fn get_pointed_ear_length(&self, size: Size) -> f32 {
        self.pointed_length.convert(size)
    }

    pub fn get_round_ear_radius(&self, head: &AABB, size: Size) -> u32 {
        (head.size().height() as f32 * self.round_factor.convert(size)) as u32
    }
}
