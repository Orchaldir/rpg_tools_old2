use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::color::Color;

pub fn render_beard(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
    beard: &Beard,
) {
    match beard {
        Beard::None => {}
        Beard::Stubble { color } => render_stubble(renderer, config, aabb, head, *color),
        Beard::Moustache { .. } => {}
    }
}

fn render_stubble(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
    color: Color,
) {
    let options = config.without_line(color);
    let polygon = get_stubble(config, aabb, head.shape);

    renderer.render_polygon(&polygon, &options);
}

fn get_stubble(config: &RenderConfig, aabb: &AABB, head_shape: HeadShape) -> Polygon2d {
    let (top_left, top_right) =
        aabb.get_mirrored_points(config.head.get_eye_width(head_shape), config.head.y_eye);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(config.head.get_mouth_width(head_shape), config.head.y_mouth);
    let (bottom_left, bottom_right) =
        aabb.get_mirrored_points(config.head.get_chin_width(head_shape), 1.0);
    let corners = vec![
        top_left,
        forehead_left,
        bottom_left,
        bottom_right,
        forehead_right,
        top_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}
