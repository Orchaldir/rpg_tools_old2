use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape, BodyWidth};
use std::ops::Mul;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct BodyRenderer {}

impl BodyRenderer {
    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        body: &Body,
    ) {
        let options = config.get_options(&body.skin);
        let height = aabb.size().height();

        let torso_height = 0.5;
        let arm_height = 0.36;
        let leg_height = 0.21;
        let feet_height = 0.07;

        let width_factor = get_width_factor(body);
        let torso_width = 0.35 * width_factor;
        let arm_width = 0.1 * width_factor;
        let leg_width = 0.14 * width_factor;
        let feet_width = 0.21 * width_factor;

        let hands_factor = 0.14 * 0.5;

        let torso_y = 0.21;
        let arm_y = torso_y + arm_height;
        let leg_y = torso_y + torso_height;
        let foot_y = leg_y + leg_height;

        let torso_start_x = 0.5 - torso_width / 2.0;
        let torso_start = aabb.get_point(torso_start_x, torso_y);
        let torso_size = aabb.size().scale(torso_width, torso_height);
        let torso_aabb = AABB::new(torso_start, torso_size);

        match body.shape {
            BodyShape::Hourglass => {
                let polygon = self.render_hourglass(&torso_aabb);
                renderer.render_polygon(&polygon, &options);
            }
            _ => renderer.render_rectangle(&torso_aabb, &options),
        }

        let arm_size = aabb.size().scale(arm_width, arm_height);
        let left_arm_start = aabb.get_point(0.5 + torso_width / 2.0, torso_y);
        renderer.render_rectangle(&AABB::new(left_arm_start, arm_size), &options);
        let right_arm_start = aabb.get_point(torso_start_x - arm_width, torso_y);
        renderer.render_rectangle(&AABB::new(right_arm_start, arm_size), &options);

        let hand_radius = (height as f32 * hands_factor) as u32;
        let arm_offset = (torso_width + arm_width) / 2.0;
        let left_hand_center = aabb.get_point(0.5 + arm_offset, arm_y);
        renderer.render_circle(&left_hand_center, hand_radius, &options);
        let right_hand_center = aabb.get_point(0.5 - arm_offset, arm_y);
        renderer.render_circle(&right_hand_center, hand_radius, &options);

        let leg_size = aabb.size().scale(leg_width, leg_height);
        let left_leg_start_x = 0.5 + torso_width / 2.0 - leg_width;
        let left_leg_start = aabb.get_point(left_leg_start_x, leg_y);
        renderer.render_rectangle(&AABB::new(left_leg_start, leg_size), &options);
        let right_leg_start = aabb.get_point(torso_start_x, leg_y);
        renderer.render_rectangle(&AABB::new(right_leg_start, leg_size), &options);

        let foot_size = aabb.size().scale(feet_width, feet_height);
        let left_foot_start = aabb.get_point(left_leg_start_x, foot_y);
        renderer.render_rectangle(&AABB::new(left_foot_start, foot_size), &options);
        let right_foot_start = aabb.get_point(torso_start_x - feet_width + leg_width, foot_y);
        renderer.render_rectangle(&AABB::new(right_foot_start, foot_size), &options);
    }

    fn render_hourglass(&self, aabb: &AABB) -> Polygon2d {
        self.render_torso(aabb, 0.15)
    }

    fn render_torso(&self, aabb: &AABB, waits_factor: f32) -> Polygon2d {
        let upper_height = 0.3;
        let waist_height = 0.5;
        let lower_height = 0.75;

        let top_left = aabb.get_point(0.0, 0.0);
        let upper_left = aabb.get_point(0.0, upper_height);
        let waist_left = aabb.get_point(waits_factor, waist_height);
        let lower_left = aabb.get_point(0.0, lower_height);
        let bottom_left = aabb.get_point(0.0, 1.0);

        let top_right = aabb.get_point(1.0, 0.0);
        let upper_right = aabb.get_point(1.0, upper_height);
        let waist_right = aabb.get_point(1.0 - waits_factor, waist_height);
        let lower_right = aabb.get_point(1.0, lower_height);
        let bottom_right = aabb.get_point(1.0, 1.0);

        let polygon = Polygon2d::new(vec![
            top_left,
            upper_left,
            waist_left,
            lower_left,
            bottom_left,
            bottom_right,
            lower_right,
            waist_right,
            upper_right,
            top_right,
        ]);
        polygon.cut_corners_n(0.25, 0.25, 3).unwrap()
    }

    pub fn calculate_head_aabb(&self, aabb: &AABB) -> AABB {
        let head_size = 0.286;

        let head_start = aabb.get_point(0.5 - head_size / 2.0, 0.0);
        let head_size = aabb.size().mul(head_size);
        AABB::new(head_start, head_size)
    }
}

fn get_width_factor(body: &Body) -> f32 {
    match body.width {
        BodyWidth::Thin => 0.8,
        BodyWidth::Average => 1.0,
        BodyWidth::Wide => 1.2,
    }
}
