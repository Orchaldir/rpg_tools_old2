use crate::math::aabb2d::AABB;
use crate::rendering::config::eye::eyebrow::EyebrowConfig;
use crate::rendering::config::size::SizeConfig;
use rpg_tools_core::model::appearance::size::Size;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;

pub mod eyebrow;

/// The rendering config of the [`eyes`](rpg_tools_core::model::character::appearance::eye::Eyes).
#[derive(Debug, PartialEq)]
pub struct EyeConfig {
    pub eyebrow: EyebrowConfig,
    pub radius: f32,
    pub half_height_almond: f32,
    pub half_height_ellipse: f32,
    pub distance_between_eyes: SizeConfig,
    pub circle_radius: f32,
    pub slit_width: f32,
}

impl EyeConfig {
    pub fn get_eye_radius(&self, head: &AABB, head_width_factor: f32) -> u32 {
        let head_width = head.size().height() as f32 * head_width_factor;
        (head_width * self.radius) as u32
    }

    pub fn get_half_height(&self, shape: EyeShape, radius: u32) -> u32 {
        let factor = match shape {
            EyeShape::Almond => self.half_height_almond,
            EyeShape::Circle => 1.0,
            EyeShape::Ellipse => self.half_height_ellipse,
        };
        (radius as f32 * factor) as u32
    }

    pub fn get_circle_radius(&self, eye_shape: EyeShape, radius: u32) -> u32 {
        (match eye_shape {
            EyeShape::Circle => radius,
            _ => self.get_half_height(eye_shape, radius),
        } as f32
            * self.circle_radius) as u32
    }

    /// Calculates the slit's width (the shorter dimension) based on its length.
    pub fn get_slit_width(&self, length: u32) -> u32 {
        (length as f32 * self.slit_width) as u32
    }

    /// Calculates the distance between the center of 2 eyes.
    pub fn get_distance_between_eyes(&self, distance: Size, head_width_factor: f32) -> f32 {
        head_width_factor * self.distance_between_eyes.convert(distance)
    }
}
