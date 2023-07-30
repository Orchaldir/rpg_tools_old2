use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};

/// The rendering config of the [`body`](Body).
#[derive(Debug, PartialEq)]
pub struct BodyConfig {
    pub width: WidthConfig,
    pub torso_factor: f32,
    pub muscular_shoulder_bonus: f32,
    pub fat_hip_bonus: f32,
}

impl BodyConfig {
    pub fn get_width_factor(&self, body: &Body) -> f32 {
        self.width.convert(body.width)
    }

    pub fn get_shoulder_width(&self, body: &Body) -> f32 {
        (self.get_width_factor(body)
            + match body.shape {
                BodyShape::Muscular => self.muscular_shoulder_bonus,
                _ => 0.0,
            })
            * self.muscluar_shoulder_bonus
    }

    pub fn get_hip_width(&self, body: &Body) -> f32 {
        (self.get_width_factor(body)
            + match body.shape {
                BodyShape::Fat => self.fat_hip_bonus,
                _ => 0.0,
            })
    }
}
