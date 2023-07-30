use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};

pub fn render_torso(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    options: &RenderOptions,
) {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let polygon = match body.shape {
        BodyShape::Fat => create_fat(&torso_aabb),
        BodyShape::Hourglass => create_hourglass(&torso_aabb),
        BodyShape::Muscular => create_muscular(&torso_aabb),
        BodyShape::Rectangle => create_rectangle(&torso_aabb),
    };
    let smooth_polygon = config.cut_corners(&polygon).unwrap();
    renderer.render_polygon(&smooth_polygon, options);
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
