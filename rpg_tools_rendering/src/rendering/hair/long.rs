use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::render_polygon;
use rpg_tools_core::model::character::appearance::hair::long::LongHairStyle;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use std::ops::Add;
use LongHairStyle::{Rounded, Straight, Triangle, Wide};

pub fn render_long_hair(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: LongHairStyle,
    length: Length,
    color: Color,
) {
    let polygon = get_long_hair_polygon(config, aabb, head_shape, style, length);
    render_polygon(renderer, config, &polygon, color);
}

fn get_long_hair_polygon(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: LongHairStyle,
    length: Length,
) -> Polygon2d {
    let width_forehead = config.head.get_forehead_width(head_shape);
    let width_eye = config.head.get_eye_width(head_shape);
    let width_mouth = config.head.get_mouth_width(head_shape);
    let mut width = width_forehead;

    let mut builder = Polygon2dBuilder::new();

    builder.add_mirrored_points(aabb, config.head.get_top_width(head_shape), 0.0, false);
    builder.add_mirrored_points(aabb, width_forehead, config.head.y_forehead, false);

    if width_eye > width {
        width = width_eye;
    }

    builder.add_mirrored_points(aabb, width, config.head.y_eye, false);

    if width_mouth > width {
        width = width_mouth;
    }

    builder.add_mirrored_points(aabb, width, config.head.y_mouth, false);
    builder.add_mirrored_points(aabb, width, 1.0, false);

    let down = Point2d::vertical(length.to_millimetre() as i32);

    match style {
        Rounded | Straight | Wide => {
            let width = width * if style == Wide { 1.5 } else { 1.0 };
            let (left, right) = aabb.get_mirrored_points(width, 1.0);
            let bottom_left = left.add(down);
            let bottom_right = right.add(down);

            builder.add_points(bottom_left, bottom_right, style == Straight);
        }
        Triangle => {
            let center = aabb.get_point(0.5, 1.0);
            let down = center.add(down);

            builder.add_point(down, true);
        }
    }

    builder.build()
}
