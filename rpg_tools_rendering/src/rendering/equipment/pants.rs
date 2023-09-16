use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::pants::{Pants, PantsStyle};

pub fn render_pants(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    pants: &Pants,
) {
    let options = config.get_options(pants.color);
    let polygon = match pants.style {
        PantsStyle::HotPants => get_hot_pants(&config.body, aabb, body, pants),
        _ => get_hot_pants(&config.body, aabb, body, pants),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_hot_pants(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let torso_aabb = config.get_torso_aabb(body, aabb);
    let torso = config.get_torso_config(body.shape);
    let mut builder = Polygon2dBuilder::new();

    builder.add_mirrored_points(&torso_aabb, torso.hip_width, config.y_lower, true);
    builder.add_mirrored_points(&torso_aabb, torso.hip_width, 1.0, false);

    builder.build()
}
