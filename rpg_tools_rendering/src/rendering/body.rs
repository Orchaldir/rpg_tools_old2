use crate::math::aabb2d::AABB;
use crate::math::orientation::Orientation;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::width::Width;
use std::ops::Mul;

pub fn render_body(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, body: &Body) {
    let options = config.get_skin_options(&body.skin);

    let torso_height = 0.5;
    let arm_height = 0.36;

    let width_factor = get_width_factor(body);
    let shoulder_width = get_shoulder_width(body, width_factor);
    let hip_width = get_hip_width(body, width_factor);
    let torso_width = shoulder_width.max(hip_width);
    let arm_width = 0.1 * width_factor;
    let leg_width = 0.14 * width_factor;
    let feet_width = 0.19 * width_factor;

    let hands_factor = 0.14 * 0.5;

    let torso_y = 0.21;
    let arm_y = torso_y + arm_height;
    let leg_y = torso_y + torso_height;

    let torso_start_x = 0.5 - torso_width / 2.0;
    let hip_star_x = 0.5 - hip_width / 2.0;
    let torso_start = aabb.get_point(torso_start_x, torso_y);
    let torso_size = aabb.size().scale(torso_width, torso_height);
    let torso_aabb = AABB::new(torso_start, torso_size);

    let polygon = match body.shape {
        BodyShape::Fat => create_fat(&torso_aabb),
        BodyShape::Hourglass => create_hourglass(&torso_aabb),
        BodyShape::Muscular => create_muscular(&torso_aabb),
        BodyShape::Rectangle => create_rectangle(&torso_aabb),
    };
    let smooth_polygon = config.cut_corners(&polygon).unwrap();
    renderer.render_polygon(&smooth_polygon, &options);

    let arm_size = aabb.size().scale(arm_width, arm_height);
    let left_arm_start = aabb.get_point(0.5 + torso_width / 2.0, torso_y);
    renderer.render_rectangle(&AABB::new(left_arm_start, arm_size), &options);
    let right_arm_start = aabb.get_point(torso_start_x - arm_width, torso_y);
    renderer.render_rectangle(&AABB::new(right_arm_start, arm_size), &options);

    let hand_radius = aabb.calculate_from_height(hands_factor);
    let arm_offset = (torso_width + arm_width) / 2.0;
    let left_hand_center = aabb.get_point(0.5 + arm_offset, arm_y);
    renderer.render_circle(&left_hand_center, hand_radius, &options);
    let right_hand_center = aabb.get_point(0.5 - arm_offset, arm_y);
    renderer.render_circle(&right_hand_center, hand_radius, &options);

    let left_leg_start_x = 0.5 + hip_width / 2.0 - leg_width;
    let left_leg_start = aabb.get_point(left_leg_start_x, leg_y);
    let leg_size = aabb.size().scale(leg_width, 1.0 - leg_y);
    renderer.render_rectangle(&AABB::new(left_leg_start, leg_size), &options);
    let right_leg_start = aabb.get_point(hip_star_x, leg_y);
    renderer.render_rectangle(&AABB::new(right_leg_start, leg_size), &options);

    let left_foot_start = aabb.get_point(left_leg_start_x + leg_width / 2.0, 1.0);
    let right_foot_start = aabb.get_point(hip_star_x + leg_width / 2.0, 1.0);
    let foot_radius = (aabb.size().width() as f32 * feet_width / 2.0) as u32;
    let offset = Orientation::from_degree(0.0);
    let angle = Orientation::from_degree(180.0);

    renderer.render_circle_arc(&left_foot_start, foot_radius, offset, angle, &options);
    renderer.render_circle_arc(&right_foot_start, foot_radius, offset, angle, &options);
}

fn create_fat(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 0.18, 0.09, 0.0)
}

fn create_hourglass(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 0.0, 0.15, 0.0)
}

fn create_muscular(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 0.0, 0.09, 0.18)
}

fn create_rectangle(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 0.0, 0.0, 0.0)
}

fn create_torso(
    aabb: &AABB,
    shoulder_factor: f32,
    waits_factor: f32,
    hip_factor: f32,
) -> Polygon2d {
    let upper_height = 0.3;
    let waist_height = 0.5;
    let lower_height = 0.75;

    let top_left = aabb.get_point(shoulder_factor, 0.0);
    let upper_left = aabb.get_point(shoulder_factor, upper_height);
    let waist_left = aabb.get_point(waits_factor, waist_height);
    let lower_left = aabb.get_point(hip_factor, lower_height);
    let bottom_left = aabb.get_point(hip_factor, 1.0);

    let top_right = aabb.get_point(1.0 - shoulder_factor, 0.0);
    let upper_right = aabb.get_point(1.0 - shoulder_factor, upper_height);
    let waist_right = aabb.get_point(1.0 - waits_factor, waist_height);
    let lower_right = aabb.get_point(1.0 - hip_factor, lower_height);
    let bottom_right = aabb.get_point(1.0 - hip_factor, 1.0);

    Polygon2d::new(vec![
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
    ])
}

pub fn calculate_head_aabb(aabb: &AABB) -> AABB {
    let head_size = 0.286;

    let head_start = aabb.get_point(0.5 - head_size / 2.0, 0.0);
    let head_size = aabb.size().mul(head_size);
    AABB::new(head_start, head_size)
}

fn get_width_factor(body: &Body) -> f32 {
    match body.width {
        Width::Thin => 0.8,
        Width::Average => 0.9,
        Width::Wide => 1.0,
    }
}

fn get_shoulder_width(body: &Body, width_factor: f32) -> f32 {
    0.35 * (width_factor
        + match body.shape {
            BodyShape::Muscular => 0.4,
            _ => 0.0,
        })
}

fn get_hip_width(body: &Body, width_factor: f32) -> f32 {
    0.35 * (width_factor
        + match body.shape {
            BodyShape::Fat => 0.4,
            _ => 0.0,
        })
}
