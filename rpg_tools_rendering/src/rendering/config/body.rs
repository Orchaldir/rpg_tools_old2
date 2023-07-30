use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};

/// The rendering config of the [`body`](Body).
#[derive(Debug, PartialEq)]
pub struct BodyConfig {
    pub width: WidthConfig,
    pub torso_factor: f32,
}

impl BodyConfig {
    pub fn get_width_factor(&self, body: &Body) -> f32 {
        self.width.convert(body.width)
    }

    pub fn get_shoulder_width(&self, body: &Body) -> f32 {
        self.torso_factor
            * (self.get_width_factor(body)
                + match body.shape {
                    BodyShape::Muscular => 0.4,
                    _ => 0.0,
                })
    }

    pub fn get_hip_width(&self, body: &Body) -> f32 {
        self.torso_factor
            * (self.get_width_factor(body)
                + match body.shape {
                    BodyShape::Fat => 0.4,
                    _ => 0.0,
                })
    }
}
