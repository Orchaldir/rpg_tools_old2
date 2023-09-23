use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::Renderer;
use crate::rendering::body::render_feet;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::footwear::{Footwear, FootwearStyle};

pub fn render_footwear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    footwear: &Footwear,
    from_front: bool,
) {
    let options = config.get_options(footwear.color);

    if is_foot_visible(footwear.style, from_front) {
        render_feet(renderer, config, aabb, body, &options);
    }

    render_soles(renderer, config, aabb, body, footwear);
}

fn is_foot_visible(style: FootwearStyle, from_front: bool) -> bool {
    match style {
        FootwearStyle::Sandals => false,
        FootwearStyle::Slippers => from_front,
        _ => true,
    }
}

fn render_soles(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    footwear: &Footwear,
) {
    let (left_center, right_center) = config.body.get_feet_center_x(body);

    render_sole(renderer, config, aabb, body, footwear, left_center);
    render_sole(renderer, config, aabb, body, footwear, right_center);
}

fn render_sole(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    footwear: &Footwear,
    center_x: f32,
) {
    let options = config.get_options(footwear.sole);
    let width = config.body.get_foot_radius_factor(body) * 2.0 * 1.05;
    let y_start = config.body.y_foot;
    let y_end = y_start + 0.02;
    let mut builder = Polygon2dBuilder::new();

    builder.add_horizontal_pair(aabb, width, center_x, y_start, true);
    builder.add_horizontal_pair(aabb, width, center_x, y_end, true);
    let polygon = builder.build();

    renderer.render_polygon(&polygon, &options);
}
