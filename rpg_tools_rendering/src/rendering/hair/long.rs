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

pub fn render_long_hair(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: LongHairStyle,
    length: Length,
    color: Color,
) {
    let polygon = get_straight_polygon(config, aabb, head_shape, length);
    render_polygon(renderer, config, &polygon, color);
}

pub fn get_straight_polygon(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    length: Length,
) -> Polygon2d {
    let width_forehead = config.head.get_forehead_width(head_shape);
    let width_eye = config.head.get_eye_width(head_shape);
    let width_mouth = config.head.get_mouth_width(head_shape);
    let mut width = width_forehead;

    let (top_left, top_right) =
        aabb.get_mirrored_points(config.head.get_top_width(head_shape), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(width_forehead, config.head.y_forehead);

    let mut left_corners = vec![top_left, forehead_left];
    let mut right_corners = vec![top_right, forehead_right];

    if width_eye > width {
        let (eye_left, eye_right) = aabb.get_mirrored_points(width_eye, config.head.y_eye);

        left_corners.push(eye_left);
        right_corners.push(eye_right);

        width = width_eye;
    }

    if width_mouth > width {
        let (mouth_left, mouth_right) = aabb.get_mirrored_points(width_mouth, config.head.y_mouth);

        left_corners.push(mouth_left);
        right_corners.push(mouth_right);

        width = width_mouth;
    }

    let (left, right) = aabb.get_mirrored_points(width, 1.0);

    let down = Point2d::new(0, length.to_millimetre() as i32);
    let bottom_left = left.add(down);
    let bottom_right = right.add(down);

    left_corners.push(bottom_left);
    left_corners.push(bottom_left);
    right_corners.push(bottom_right);
    right_corners.push(bottom_right);

    right_corners.reverse();
    left_corners.append(&mut right_corners);

    let polygon = Polygon2d::new(left_corners);
    config.cut_corners(&polygon).unwrap()
}
