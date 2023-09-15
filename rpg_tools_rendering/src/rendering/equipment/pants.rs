use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
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
        PantsStyle::HotPants => get_hot_pants(config, aabb, body, pants),
        _ => get_hot_pants(config, aabb, body, pants),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_hot_pants(config: &RenderConfig, aabb: &AABB, body: &Body, pants: &Pants) -> Polygon2d {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);

    let (lower_left, lower_right) =
        torso_aabb.get_mirrored_points(torso.hip_width, config.body.y_lower);
    let (bottom_left, bottom_right) = torso_aabb.get_mirrored_points(torso.hip_width, 1.0);

    Polygon2d::new(vec![
        lower_left,
        lower_left,
        bottom_left,
        bottom_right,
        lower_right,
        lower_right,
    ])
}
