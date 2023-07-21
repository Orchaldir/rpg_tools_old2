use crate::rendering::config::size::SizeConfig;
use rpg_tools_core::model::size::Size;

/// The rendering config of the [`ears`](rpg_tools_core::model::character::appearance::ear::Ears).
#[derive(Debug, PartialEq)]
pub struct EarConfig {
    pub pointed_ear_length: SizeConfig,
}

impl EarConfig {
    pub fn get_pointed_ear_length(&self, size: Size) -> f32 {
        self.pointed_ear_length.convert(size)
    }
}
