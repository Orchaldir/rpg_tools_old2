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
    )
    .build();
    renderer.render_rounded_polygon(&polygon, options);
}

pub fn create_torso(aabb: &AABB, config: &BodyConfig, torso: &TorsoConfig) -> Polygon2dBuilder {
    let mut builder = Polygon2dBuilder::new();

    builder.add_mirrored_points(aabb, torso.hip_width, 1.0, false);
    builder.add_mirrored_points(aabb, torso.hip_width, config.y_lower, false);
    builder.add_mirrored_points(aabb, torso.waist_width, config.y_waist, false);
    builder.add_mirrored_points(aabb, torso.shoulder_width, config.y_upper, false);
    builder.add_mirrored_points(aabb, torso.shoulder_width, 0.0, false);

    builder
}
