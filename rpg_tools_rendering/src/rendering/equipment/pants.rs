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
        PantsStyle::Bermuda => get_bermuda(&config.body, aabb, body, pants),
        PantsStyle::HotPants => get_hot_pants(&config.body, aabb, body, pants),
        PantsStyle::Regular => get_regular_pants(&config.body, aabb, body, pants),
        PantsStyle::Shorts => get_shorts(&config.body, aabb, body, pants),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_bermuda(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let top_y = config.get_torso_bottom();
    let bottom_y = 1.0 - config.get_foot_radius_factor(body) - 0.02;
    get_pants(config, aabb, body, pants, (top_y + bottom_y) * 0.5)
}

fn get_hot_pants(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    get_base(config, aabb, body, pants).build()
}

fn get_regular_pants(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let bottom_y = 1.0 - config.get_foot_radius_factor(body) - 0.02;
    get_pants(config, aabb, body, pants, bottom_y)
}

fn get_shorts(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let top_y = config.get_torso_bottom();
    let bottom_y = 1.0 - config.get_foot_radius_factor(body) - 0.02;
    get_pants(config, aabb, body, pants, top_y * 0.7 + bottom_y * 0.3)
}

fn get_pants(
    config: &BodyConfig,
    aabb: &AABB,
    body: &Body,
    pants: &Pants,
    bottom_y: f32,
) -> Polygon2d {
    let mut builder = get_base(config, aabb, body, pants);
    let legs_width = config.get_legs_width(body) * 1.05;
    let leg_width = config.get_leg_width(body) * 1.1;
    let inner_width = legs_width - 2.0 * leg_width;
    let top_y = config.get_torso_bottom();
    let mid_y = (top_y + bottom_y) * 0.5;

    builder.add_mirrored_points(aabb, legs_width * 0.95, top_y, false);
    builder.add_mirrored_points(aabb, legs_width, mid_y, false);
    builder.add_mirrored_points(aabb, legs_width, bottom_y, true);
    builder.add_mirrored_points(aabb, inner_width, bottom_y, true);
    builder.add_mirrored_points(aabb, inner_width * 0.9, top_y, false);

    builder.build()
}

fn get_base(config: &BodyConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2dBuilder {
    let torso_aabb = config.get_torso_aabb(body, aabb);
    let torso = config.get_torso_config(body.shape);
    let hip_width = torso.hip_width * 1.02;
    let mut builder = Polygon2dBuilder::new();

    // center curves downwards
    builder.add_point(torso_aabb.get_point(0.5, config.y_lower + 0.02), false);
    // rectangle forming the base of the pants
    builder.add_mirrored_points(&torso_aabb, hip_width, config.y_lower, true);
    builder.add_mirrored_points(&torso_aabb, hip_width, 1.0, false);

    builder
}
