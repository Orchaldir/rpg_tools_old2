use crate::math::aabb2d::AABB;

/// The rendering config of the [`eyes`](rpg_tools_core::model::character::appearance::eye::Eyes).
#[derive(Debug, PartialEq)]
pub struct EyeConfig {
    pub radius: f32,
    pub radius_y_almond: f32,
    pub radius_y_ellipse: f32,
    pub low_distance: f32,
    pub medium_distance: f32,
    pub high_distance: f32,
    pub circle_radius: f32,
    pub slit_radius: f32,
}

impl EyeConfig {
    pub fn get_eye_radius(&self, head: &AABB, head_width_factor: f32) -> u32 {
        let head_width = head.size().height() as f32 * head_width_factor;
        (head_width * self.radius) as u32
    }
}
