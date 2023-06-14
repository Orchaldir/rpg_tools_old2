use crate::math::aabb2d::AABB;
use rpg_tools_core::model::character::appearance::eye::{EyeDistance, EyeShape};

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

    pub fn get_eye_radius_y(&self, shape: EyeShape, radius: u32) -> u32 {
        let factor = match shape {
            EyeShape::Almond => self.radius_y_almond,
            EyeShape::Circle => 1.0,
            EyeShape::Ellipse => self.radius_y_ellipse,
        };
        (radius as f32 * factor) as u32
    }

    pub fn get_circle_radius(&self, eye_shape: EyeShape, radius: u32) -> u32 {
        (match eye_shape {
            EyeShape::Circle => radius,
            _ => self.get_eye_radius_y(eye_shape, radius),
        } as f32
            * self.circle_radius) as u32
    }

    pub fn get_slit_radius(&self, radius: u32) -> u32 {
        (radius as f32 * self.slit_radius) as u32
    }

    pub fn get_distance_between_eyes(&self, distance: EyeDistance, head_width_factor: f32) -> f32 {
        head_width_factor
            * match distance {
                EyeDistance::Low => self.low_distance,
                EyeDistance::Medium => self.medium_distance,
                EyeDistance::High => self.high_distance,
            }
    }
}
