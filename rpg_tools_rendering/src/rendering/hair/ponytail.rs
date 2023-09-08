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
    let width =
        (config.head.get_top_width(head_shape) + config.head.get_forehead_width(head_shape)) / 2.0;

    let polygon = match position {
        PonytailPosition::High => get_ponytail_down(aabb, style, radius, length),
        PonytailPosition::Left => get_ponytail_left(aabb, width, style, radius, length),
        _ => get_ponytail_down(aabb, style, 1.0 - radius, length),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_ponytail_down(aabb: &AABB, style: PonytailStyle, start: f32, length: Length) -> Polygon2d {
    let length = aabb.convert_from_height(length.to_millimetre());
    let width = 0.2;
    let bottom_width = width
        * if style == PonytailStyle::Wide {
            2.0
        } else {
            1.0
        };
    let (top_left, top_right) = aabb.get_mirrored_points(width, start);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, start + length);

    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}

fn get_ponytail_left(
    aabb: &AABB,
    head_width: f32,
    style: PonytailStyle,
    start_y: f32,
    length: Length,
) -> Polygon2d {
    let length = aabb.convert_from_height(length.to_millimetre());
    let start_x = head_width / 2.0;
    let bottom_y = start_y + length;
    let width = 0.2;
    let start_half = width / 2.0;
    let bottom_width = width
        * if style == PonytailStyle::Wide {
            2.0
        } else {
            1.0
        };
    let bottom_half = bottom_width / 2.0;

    let top_left = aabb.get_point(start_x - start_half, start_y - start_half);
    let top_right = aabb.get_point(start_x + start_half, start_y + start_half);
    let bottom_left = aabb.get_point(start_x - bottom_half, bottom_y);
    let bottom_right = aabb.get_point(start_x + bottom_half, bottom_y);

    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
