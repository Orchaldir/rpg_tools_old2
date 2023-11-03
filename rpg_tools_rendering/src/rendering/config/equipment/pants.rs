use crate::rendering::config::body::BodyConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// The rendering config of the [`pants`](Pants).
#[derive(Debug, PartialEq)]
pub struct PantsConfig {
    pub height_bermuda: f32,
    pub height_shorts: f32,
    pub offset_center: f32,
    pub offset_bottom: f32,
    pub width_padding: f32,
    pub balloon_padding: f32,
}

impl PantsConfig {
    /// Returns the width of the pants and the space between the individuals pants in the body aabb.
    pub fn get_widths(&self, config: &BodyConfig, body: &Body) -> (f32, f32) {
        let legs_width = config.get_legs_width(body);
        let padding = legs_width * self.width_padding;
        let pants_width = legs_width + padding;
        let pant_width = config.get_leg_width(body) + padding;
        let inner_width = pants_width - 2.0 * pant_width;

        (pants_width, inner_width)
    }

    /// Returns the width of the hip in the torso aabb.
    pub fn get_hip_width(&self, config: &BodyConfig, body: &Body) -> f32 {
        let torso = config.get_torso_config(body.shape);
        torso.hip_width * (1.0 + self.width_padding)
    }

    pub fn get_bottom_y(&self, config: &BodyConfig, body: &Body) -> f32 {
        config.y_foot - config.get_foot_radius_factor(body) - self.offset_bottom
    }
}
