use crate::math::aabb2d::{get_end_x, get_start_x, AABB};
use crate::math::point2d::Point2d;
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};

pub mod torso;

/// The rendering config of the [`body`](Body).
#[derive(Debug, PartialEq)]
pub struct BodyConfig {
    pub width: WidthConfig,
    pub torso_factor: f32,
    pub hand_factor: f32,
    pub foot_factor: f32,
    pub muscular_shoulder_bonus: f32,
    pub fat_hip_bonus: f32,
    pub height_head: f32,
    pub height_torso: f32,
    pub height_arm: f32,
    pub fat: TorsoConfig,
    pub hourglass: TorsoConfig,
    pub muscular: TorsoConfig,
    pub rectangle: TorsoConfig,
    pub width_arm: f32,
    pub width_leg: f32,
    pub y_torso: f32,
    pub y_upper: f32,
    pub y_waist: f32,
    pub y_lower: f32,
}

impl BodyConfig {
    pub fn get_arm_width(&self, body: &Body) -> f32 {
        self.width_arm * self.get_width_factor(body)
    }

    pub fn get_leg_width(&self, body: &Body) -> f32 {
        self.width_leg * self.get_width_factor(body) * self.get_torso_config(body.shape).legs_width
    }

    pub fn get_width_factor(&self, body: &Body) -> f32 {
        self.width.convert(body.width)
    }

    pub fn get_hip_width(&self, body: &Body) -> f32 {
        (self.get_width_factor(body)
            + match body.shape {
                BodyShape::Fat => self.fat_hip_bonus,
                _ => 0.0,
            })
            * self.torso_factor
    }

    /// The aabb of both legs is limited to the smaller width of shoulders or hip to match *Fat* & *Muscular* [`body shapes`](BodyShape).
    pub fn get_legs_width(&self, body: &Body) -> f32 {
        self.get_shoulder_width(body).min(self.get_hip_width(body))
            * self.get_torso_config(body.shape).legs_width
    }

    pub fn get_shoulder_width(&self, body: &Body) -> f32 {
        (self.get_width_factor(body)
            + match body.shape {
                BodyShape::Muscular => self.muscular_shoulder_bonus,
                _ => 0.0,
            })
            * self.torso_factor
    }

    pub fn get_torso_aabb(&self, body: &Body, aabb: &AABB) -> AABB {
        let torso_width = self.get_torso_width(body);
        let torso_start_x = get_start_x(torso_width);
        let torso_start = aabb.get_point(torso_start_x, self.y_torso);
        let torso_size = aabb.size().scale(torso_width, self.height_torso);
        AABB::new(torso_start, torso_size)
    }

    /// The torso's aabb covers the shoulders & the hip.
    pub fn get_torso_width(&self, body: &Body) -> f32 {
        self.get_shoulder_width(body).max(self.get_hip_width(body))
    }

    pub fn get_torso_bottom(&self) -> f32 {
        self.y_torso + self.height_torso
    }

    pub fn get_arm_y(&self) -> f32 {
        self.y_torso + 0.02
    }

    pub fn get_leg_y(&self) -> f32 {
        self.get_torso_bottom() - 0.05
    }

    pub fn get_foot_y(&self) -> f32 {
        1.0
    }

    pub fn get_distance_between_hands(&self, body: &Body) -> f32 {
        self.get_torso_width(body) + 0.08
    }

    pub fn get_hand_radius(&self, body: &Body, aabb: &AABB) -> u32 {
        aabb.convert_to_height(self.get_hand_radius_factor(body))
    }

    pub fn get_hand_radius_factor(&self, body: &Body) -> f32 {
        self.hand_factor * self.get_width_factor(body)
    }

    pub fn get_foot_radius(&self, body: &Body, aabb: &AABB) -> u32 {
        aabb.convert_to_height(self.get_foot_radius_factor(body))
    }

    pub fn get_foot_radius_factor(&self, body: &Body) -> f32 {
        self.foot_factor
            * self.get_width_factor(body)
            * self.get_torso_config(body.shape).legs_width
    }

    pub fn get_feet_centers(&self, body: &Body, aabb: &AABB) -> (Point2d, Point2d) {
        let leg_half = self.get_leg_width(body) / 2.0;
        let foot_y = self.get_foot_y();
        let left_leg_start_x = self.get_left_leg_x(body);
        let right_leg_x = self.get_right_leg_x(body);
        let left = aabb.get_point(left_leg_start_x + leg_half, foot_y);
        let right = aabb.get_point(right_leg_x + leg_half, foot_y);

        (left, right)
    }

    pub fn get_left_leg_x(&self, body: &Body) -> f32 {
        let legs_width = self.get_legs_width(body);
        let leg_width = self.get_leg_width(body);
        get_end_x(legs_width) - leg_width
    }

    pub fn get_right_leg_x(&self, body: &Body) -> f32 {
        let legs_width = self.get_legs_width(body);
        get_start_x(legs_width)
    }

    pub fn get_torso_config(&self, shape: BodyShape) -> &TorsoConfig {
        match shape {
            BodyShape::Fat => &self.fat,
            BodyShape::Hourglass => &self.hourglass,
            BodyShape::Muscular => &self.muscular,
            BodyShape::Rectangle => &self.rectangle,
        }
    }
}
