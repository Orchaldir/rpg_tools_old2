use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;

pub fn render_torso(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    options: &RenderOptions,
) {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let polygon = create_torso(
        &torso_aabb,
        &config.body,
        config.body.get_torso_config(body.shape),
        0.0,
    )
    .build();
    renderer.render_rounded_polygon(&polygon, options);
}

pub fn create_torso(
    aabb: &AABB,
    config: &BodyConfig,
    torso: &TorsoConfig,
    hip_padding: f32,
) -> Polygon2dBuilder {
    let mut builder = Polygon2dBuilder::new();
    let hip_width = torso.hip_width * (1.0 + hip_padding);
    let waist_width = torso.waist_width * (1.0 + hip_padding);

    builder.add_mirrored_points(aabb, hip_width, 1.0, false);
    builder.add_mirrored_points(aabb, hip_width, config.y_lower, false);
    builder.add_mirrored_points(aabb, waist_width, config.y_waist, false);
    builder.add_mirrored_points(aabb, torso.shoulder_width, config.y_upper, false);
    builder.add_mirrored_points(aabb, torso.shoulder_width, 0.0, false);

    builder
}
