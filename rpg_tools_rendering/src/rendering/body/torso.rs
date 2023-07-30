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
    create_torso(aabb, 0.64, 0.82, 1.0)
}

fn create_hourglass(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 1.0, 0.7, 1.0)
}

fn create_muscular(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 1.0, 0.82, 0.64)
}

fn create_rectangle(aabb: &AABB) -> Polygon2d {
    create_torso(aabb, 1.0, 1.0, 1.0)
}

fn create_torso(aabb: &AABB, shoulder_width: f32, waist_width: f32, hip_width: f32) -> Polygon2d {
    let upper_height = 0.3;
    let waist_height = 0.5;
    let lower_height = 0.75;

    let (top_left, top_right) = aabb.get_mirrored_points(shoulder_width, 0.0);
    let (upper_left, upper_right) = aabb.get_mirrored_points(shoulder_width, upper_height);
    let (waist_left, waist_right) = aabb.get_mirrored_points(waist_width, waist_height);
    let (lower_left, lower_right) = aabb.get_mirrored_points(hip_width, lower_height);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(hip_width, 1.0);

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
