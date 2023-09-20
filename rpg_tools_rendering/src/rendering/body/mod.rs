use crate::math::aabb2d::{get_end_x, get_start_x, AABB};
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::size2d::Size2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::body::torso::render_torso;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::pants::interpolate;
use rpg_tools_core::model::character::appearance::body::Body;
use std::ops::Mul;

pub mod torso;

pub fn render_body(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, body: &Body) {
    let options = config.get_skin_options(&body.skin);

    render_legs(renderer, config, aabb, body, &options);
    render_arms(renderer, config, aabb, body, &options);
    render_torso(renderer, config, aabb, body, &options);
}

fn render_legs(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    options: &RenderOptions,
) {
    let legs_width = config.body.get_legs_width(body);
    let leg_width = config.body.get_leg_width(body);
    let leg_y = config.body.get_leg_y();

    let left_leg_start_x = get_end_x(legs_width) - leg_width;
    let left_leg_start = aabb.get_point(left_leg_start_x, leg_y);
    let leg_size = aabb.size().scale(leg_width, 1.0 - leg_y);
    let right_leg_x = get_start_x(legs_width);
    let right_leg_start = aabb.get_point(right_leg_x, leg_y);

    render_leg(renderer, options, left_leg_start, leg_size);
    render_leg(renderer, options, right_leg_start, leg_size);

    let left_foot_start = aabb.get_point(left_leg_start_x + leg_width / 2.0, 1.0);
    let right_foot_start = aabb.get_point(right_leg_x + leg_width / 2.0, 1.0);
    let foot_radius = config.body.get_foot_radius(body, aabb);
    let offset = Orientation::from_degree(0.0);
    let angle = Orientation::from_degree(180.0);

    renderer.render_circle_arc(&left_foot_start, foot_radius, offset, angle, options);
    renderer.render_circle_arc(&right_foot_start, foot_radius, offset, angle, options);
}

pub fn render_hands(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, body: &Body) {
    let options = config.get_skin_options(&body.skin);
    let hand_radius = config.body.get_hand_radius(body, aabb);
    let distance_between_hands =
        config.body.get_distance_between_hands(body) + config.body.get_hand_radius_factor(body);
    let hand_y = config.body.get_arm_y() + config.body.height_arm;
    let (left_hand_center, right_hand_center) =
        aabb.get_mirrored_points(distance_between_hands, hand_y);

    renderer.render_circle(&left_hand_center, hand_radius, &options);
    renderer.render_circle(&right_hand_center, hand_radius, &options);
}

fn render_arms(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    options: &RenderOptions,
) {
    let polygon = get_left_arm(config, aabb, body).build();

    renderer.render_rounded_polygon(&polygon, options);
    renderer.render_rounded_polygon(&aabb.mirrored(&polygon), options);
}

pub fn get_left_arm(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2dBuilder {
    let mut builder = get_left_arm_short(config, aabb, body, false);
    let width = config.body.get_arm_width(body);
    let bottom_x = get_end_x(config.body.get_distance_between_hands(body));
    let y = config.body.get_arm_y() + config.body.height_arm;

    builder.add_point(aabb.get_point(bottom_x, y), false);
    builder.add_point_cw(aabb.get_point(bottom_x + width, y), false);

    builder
}

pub fn get_left_arm_short(
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    bottom_sharp: bool,
) -> Polygon2dBuilder {
    let mut builder = Polygon2dBuilder::new();
    let width = config.body.get_arm_width(body);
    let top_x = get_end_x(config.body.get_shoulder_width(body) * 0.94);
    let bottom_x = get_end_x(config.body.get_distance_between_hands(body));
    let bottom_x = interpolate(top_x, bottom_x, 0.7);
    let y = config.body.get_arm_y();
    let mid_y = y + 0.2;

    builder.add_point(aabb.get_point(top_x, y), true);
    builder.add_point_cw(aabb.get_point(top_x + width, y), false);
    builder.add_point(aabb.get_point(top_x, y + 0.1), true);
    builder.add_point(aabb.get_point(bottom_x, mid_y), bottom_sharp);
    builder.add_point_cw(aabb.get_point(bottom_x + width, mid_y), bottom_sharp);

    builder
}

fn render_leg(renderer: &mut dyn Renderer, options: &RenderOptions, start: Point2d, size: Size2d) {
    let aabb = AABB::new(start, size);
    let polygon = (&aabb).into();
    renderer.render_rounded_polygon(&polygon, options);
}

pub fn calculate_head_aabb(config: &RenderConfig, aabb: &AABB) -> AABB {
    let head_size = config.body.height_head;

    let head_start = aabb.get_point(get_start_x(head_size), 0.0);
    let head_size = aabb.size().mul(head_size);
    AABB::new(head_start, head_size)
}
