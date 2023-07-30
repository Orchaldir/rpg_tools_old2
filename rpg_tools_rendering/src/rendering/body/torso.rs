use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::body::torso::TorsoConfig;
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
        BodyShape::Fat => create_torso(&torso_aabb, &config.body.fat),
        BodyShape::Hourglass => create_torso(&torso_aabb, &config.body.hourglass),
        BodyShape::Muscular => create_torso(&torso_aabb, &config.body.muscular),
        BodyShape::Rectangle => create_torso(&torso_aabb, &config.body.rectangle),
    };
    let smooth_polygon = config.cut_corners(&polygon).unwrap();
    renderer.render_polygon(&smooth_polygon, options);
}

fn create_torso(aabb: &AABB, torso: &TorsoConfig) -> Polygon2d {
    let upper_height = 0.3;
    let waist_height = 0.5;
    let lower_height = 0.75;

    let (top_left, top_right) = aabb.get_mirrored_points(torso.shoulder_width, 0.0);
    let (upper_left, upper_right) = aabb.get_mirrored_points(torso.shoulder_width, upper_height);
    let (waist_left, waist_right) = aabb.get_mirrored_points(torso.waist_width, waist_height);
    let (lower_left, lower_right) = aabb.get_mirrored_points(torso.hip_width, lower_height);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(torso.hip_width, 1.0);

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
