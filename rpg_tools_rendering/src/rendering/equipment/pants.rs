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
        PantsStyle::Regular => get_regular_pants(&config.body, aabb, body, pants),
        _ => get_hot_pants(&config.body, aabb, body, pants),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_hot_pants(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    get_base(config, aabb, body, pants).build()
}

fn get_regular_pants(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let mut builder = get_base(config, aabb, body, pants);
    let legs_width = config.get_legs_width(body);
    let leg_width = config.get_leg_width(body);
    let bottom = 1.0 - config.get_foot_radius_factor(body);

    builder.add_mirrored_points(aabb, legs_width, bottom, true);
    builder.add_mirrored_points(aabb, legs_width - 2.0 * leg_width, bottom, true);
    builder.add_mirrored_points(
        aabb,
        legs_width - 2.0 * leg_width,
        config.get_torso_bottom(),
        false,
    );

    builder.build()
}

fn get_base(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2dBuilder {
    let torso_aabb = config.get_torso_aabb(body, aabb);
    let torso = config.get_torso_config(body.shape);
    let mut builder = Polygon2dBuilder::new();

    builder.add_mirrored_points(&torso_aabb, torso.hip_width, config.y_lower, true);
    builder.add_mirrored_points(&torso_aabb, torso.hip_width, 1.0, false);

    builder
}
