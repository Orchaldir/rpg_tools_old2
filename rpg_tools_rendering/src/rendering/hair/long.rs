use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
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

    let mut left_corners = vec![];
    let mut right_corners = vec![];

    add_mirrored_points(
        aabb,
        config.head.get_top_width(head_shape),
        0.0,
        &mut left_corners,
        &mut right_corners,
    );

    add_mirrored_points(
        aabb,
        width_forehead,
        config.head.y_forehead,
        &mut left_corners,
        &mut right_corners,
    );

    if width_eye > width {
        width = width_eye;
    }

    add_mirrored_points(
        aabb,
        width,
        config.head.y_eye,
        &mut left_corners,
        &mut right_corners,
    );

    if width_mouth > width {
        width = width_mouth;
    }

    add_mirrored_points(
        aabb,
        width,
        config.head.y_mouth,
        &mut left_corners,
        &mut right_corners,
    );

    add_mirrored_points(aabb, width, 1.0, &mut left_corners, &mut right_corners);

    let down = Point2d::new(0, length.to_millimetre() as i32);

    match style {
        Rounded | Straight | Wide => {
            let width = width * if style == Wide { 1.5 } else { 1.0 };
            let (left, right) = aabb.get_mirrored_points(width, 1.0);
            let bottom_left = left.add(down);
            let bottom_right = right.add(down);

            left_corners.push(bottom_left);
            right_corners.push(bottom_right);

            if style == Straight {
                left_corners.push(bottom_left);
                right_corners.push(bottom_right);
            }
        }
        Triangle => {
            let center = aabb.get_point(0.5, 1.0);
            let down = center.add(down);

            left_corners.push(down);
            left_corners.push(down);
        }
    }

    right_corners.reverse();
    left_corners.append(&mut right_corners);

    Polygon2d::new(left_corners)
}

fn add_mirrored_points(
    aabb: &AABB,
    width: f32,
    vertical: f32,
    left_corners: &mut Vec<Point2d>,
    right_corners: &mut Vec<Point2d>,
) -> (Point2d, Point2d) {
    let (left, right) = aabb.get_mirrored_points(width, vertical);

    left_corners.push(left);
    right_corners.push(right);

    (left, right)
}
