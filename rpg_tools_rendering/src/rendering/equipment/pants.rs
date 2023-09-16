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
        PantsStyle::Balloon => get_balloon(config, aabb, body),
        PantsStyle::Bermuda => get_bermuda(config, aabb, body),
        PantsStyle::HotPants => get_hot_pants(config, aabb, body),
        PantsStyle::Regular => get_regular_pants(config, aabb, body),
        PantsStyle::Shorts => get_shorts(config, aabb, body),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_balloon(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let mut builder = get_base(&config.body, aabb, body);
    let (pants_width, pant_width) = config.pants.get_widths(&config.body, body);
    let inner_width = pants_width - 2.0 * pant_width;
    let top_y = config.body.get_torso_bottom();
    let bottom_y = get_bottom_y(&config.body, body);
    let mid_y = top_y * 0.6 + bottom_y * 0.4;

    builder.add_mirrored_points(aabb, pants_width, mid_y, false);
    builder.add_mirrored_points(aabb, pants_width * 1.2, bottom_y, false);
    builder.add_mirrored_points(aabb, inner_width * 0.4, bottom_y, false);
    builder.add_mirrored_points(aabb, inner_width * 0.8, mid_y, false);
    builder.add_point(aabb.get_point(0.5, top_y), false);

    builder.build()
}

fn get_bermuda(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let top_y = config.body.get_torso_bottom();
    let bottom_y = get_bottom_y(&config.body, body);
    get_pants(config, aabb, body, (top_y + bottom_y) * 0.5)
}

fn get_hot_pants(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_base(&config.body, aabb, body).build()
}

fn get_regular_pants(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_pants(config, aabb, body, get_bottom_y(&config.body, body))
}

fn get_shorts(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let top_y = config.body.get_torso_bottom();
    let bottom_y = get_bottom_y(&config.body, body);
    get_pants(config, aabb, body, top_y * 0.7 + bottom_y * 0.3)
}

fn get_pants(config: &RenderConfig, aabb: &AABB, body: &Body, bottom_y: f32) -> Polygon2d {
    let mut builder = get_base(&config.body, aabb, body);
    let (pants_width, pant_width) = config.pants.get_widths(&config.body, body);
    let inner_width = pants_width - 2.0 * pant_width;
    let top_y = config.body.get_torso_bottom();
    let mid_y = (top_y + bottom_y) * 0.5;

    builder.add_mirrored_points(aabb, pants_width, mid_y, false);
    builder.add_mirrored_points(aabb, pants_width, bottom_y, true);
    builder.add_mirrored_points(aabb, inner_width, bottom_y, true);
    builder.add_point(aabb.get_point(0.5, top_y), false);

    builder.build()
}

fn get_base(config: &BodyConfig, aabb: &AABB, body: &Body) -> Polygon2dBuilder {
    let torso_aabb = config.get_torso_aabb(body, aabb);
    let torso = config.get_torso_config(body.shape);
    let hip_width = torso.hip_width * 1.05;
    let mut builder = Polygon2dBuilder::new();

    // center curves downwards
    builder.add_point(torso_aabb.get_point(0.5, config.y_lower + 0.02), false);
    // rectangle forming the base of the pants
    builder.add_mirrored_points(&torso_aabb, hip_width, config.y_lower, true);
    builder.add_mirrored_points(&torso_aabb, hip_width, 1.0, false);

    builder
}

fn get_bottom_y(config: &BodyConfig, body: &Body) -> f32 {
    1.0 - config.get_foot_radius_factor(body)
}
