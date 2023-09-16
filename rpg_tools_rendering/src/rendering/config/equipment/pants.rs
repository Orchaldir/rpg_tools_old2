use crate::rendering::config::body::BodyConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// The rendering config of the [`pants`](Pants).
#[derive(Debug, PartialEq)]
pub struct PantsConfig {
    pub width_padding: f32,
}

impl PantsConfig {
    /// Returns the width of pants and an pant.
    pub fn get_widths(&self, config: &BodyConfig, body: &Body) -> (f32, f32) {
        let legs_width = config.get_legs_width(body);
        let padding = legs_width * self.width_padding;
        let pants_width = legs_width + padding;
        let pant_width = config.get_leg_width(body) + padding;

        (pants_width, pant_width)
    }
}
