use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::belt::Belt;

pub fn render_belt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    belt: &Belt,
) {
    render_band(renderer, config, aabb, body, belt);
}

fn render_band(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    belt: &Belt,
) {
    let options = config.get_options(belt.color);
    let polygon = get_band(config, aabb, body);

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_band(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let hip_width = config.pants.get_hip_width(&config.body, body) + 0.04;
    let top_y = config.body.y_lower + 0.04;
    let top_center_y = top_y + config.pants.offset_center;
    let bottom_y = top_y + 0.1;
    let bottom_center_y = bottom_y + config.pants.offset_center;
    let mut builder = Polygon2dBuilder::new();

    // center curves downwards
    builder.add_point(torso_aabb.get_point(0.5, top_center_y), false);
    builder.add_mirrored_points(&torso_aabb, hip_width, top_y, true);
    builder.add_mirrored_points(&torso_aabb, hip_width, bottom_y, true);
    builder.add_point(torso_aabb.get_point(0.5, bottom_center_y), false);

    builder.build()
}
