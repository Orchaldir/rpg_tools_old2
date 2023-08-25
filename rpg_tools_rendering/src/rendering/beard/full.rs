use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::length::Length;
use std::ops::{Add, Div, Sub};

pub fn get_full_forked(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    length: &Length,
) -> Polygon2d {
    let width =
        (config.head.get_eye_width(head_shape) + config.head.get_mouth_width(head_shape)) / 2.0;
    let top_y = config.head.get_moustache_y();
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (chin_left, chin_right) = aabb.get_mirrored_points(width, 1.0);
    let center = aabb.get_point(0.5, 1.0);
    let down = Point2d::new(0, length.to_millimetre() as i32);
    let side = chin_right.sub(chin_left).div(4.0);
    let bottom = center.add(down);
    let bottom_left = bottom.sub(side);
    let bottom_right = bottom.add(side);
    let corners = vec![
        top_left,
        chin_left,
        bottom_left,
        center,
        bottom_right,
        chin_right,
        top_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_full_rectangle(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    length: &Length,
) -> Polygon2d {
    let width =
        (config.head.get_eye_width(head_shape) + config.head.get_mouth_width(head_shape)) / 2.0;
    let top_y = config.head.get_moustache_y();
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (chin_left, chin_right) = aabb.get_mirrored_points(width, 1.0);
    let down = Point2d::new(0, length.to_millimetre() as i32);
    let bottom_left = chin_left.add(down);
    let bottom_right = chin_right.add(down);
    let corners = vec![
        top_left,
        chin_left,
        bottom_left,
        bottom_right,
        chin_right,
        top_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_full_triangle(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    length: &Length,
) -> Polygon2d {
    let width =
        (config.head.get_eye_width(head_shape) + config.head.get_mouth_width(head_shape)) / 2.0;
    let top_y = config.head.get_moustache_y();
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (chin_left, chin_right) = aabb.get_mirrored_points(width, 1.0);
    let down = Point2d::new(0, length.to_millimetre() as i32);
    let center = aabb.get_point(0.5, 1.0).add(down);
    let corners = vec![top_left, chin_left, center, chin_right, top_right];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}
