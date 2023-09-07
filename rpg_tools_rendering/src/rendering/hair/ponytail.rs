use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::hair::ponytail::position::PonytailPosition;
use rpg_tools_core::model::character::appearance::hair::ponytail::style::PonytailStyle;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use std::ops::Add;

pub fn render_ponytail(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    position: PonytailPosition,
    style: PonytailStyle,
    length: Length,
    color: Color,
) {
    let options = config.get_options(color);
    let radius = 0.2;

    let polygon = match position {
        PonytailPosition::High => get_ponytail_down(aabb, style, radius, length),
        _ => get_ponytail_down(aabb, style, 1.0 - radius, length),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_ponytail_down(aabb: &AABB, style: PonytailStyle, start: f32, length: Length) -> Polygon2d {
    let (top_left, top_right) = aabb.get_mirrored_points(0.2, start);
    let down = Point2d::vertical(length.to_millimetre() as i32);
    let bottom_left = top_left.add(down);
    let bottom_right = top_right.add(down);

    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
