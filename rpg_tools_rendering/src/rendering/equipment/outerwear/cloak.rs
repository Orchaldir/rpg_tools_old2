use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::outerwear::get_bottom_y;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::cloak::Cloak;

pub fn render_cloak_behind_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    cloak: &Cloak,
    from_front: bool,
) {
    let options = config.get_options(cloak.get_color(!from_front));
    let polygon = get_cloak_polygon(config, aabb, body, cloak);

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_cloak_polygon(config: &RenderConfig, aabb: &AABB, body: &Body, cloak: &Cloak) -> Polygon2d {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);
    let mut builder = Polygon2dBuilder::new();
    let y_factor = get_bottom_y(config, body, cloak.length);
    let shoulder_width = torso.shoulder_width + 0.2;
    let bottom_width = shoulder_width * config.body.get_torso_width(body);

    builder.add_mirrored_points(&torso_aabb, shoulder_width, -0.05, false);
    builder.add_mirrored_points(&torso_aabb, shoulder_width, config.body.y_upper, false);
    builder.add_mirrored_points(aabb, bottom_width, y_factor, true);
    builder.add_point(aabb.get_point(0.5, y_factor - 0.01), false);

    builder.build()
}
