use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::equipment::pants::PantsConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// The rendering config of the [`belt`](Belt).
#[derive(Debug, PartialEq)]
pub struct BeltConfig {
    pub y_offset: f32,
    pub height: f32,
    pub thickness: f32,
}

impl BeltConfig {
    pub fn get_width(
        &self,
        body_config: &BodyConfig,
        pants_config: &PantsConfig,
        body: &Body,
    ) -> f32 {
        pants_config.get_hip_width(body_config, body) + 2.0 * self.thickness
    }
}
