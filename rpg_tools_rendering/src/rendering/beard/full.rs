use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::length::Length;
use std::ops::Add;

pub fn get_full_rectangle(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    length: &Length,
) -> Polygon2d {
    let top_width =
        (config.head.get_eye_width(head_shape) + config.head.get_mouth_width(head_shape)) / 2.0;
    let top_y = config.head.get_moustache_y();
    let (top_left, top_right) = aabb.get_mirrored_points(top_width, top_y);
    let down = Point2d::new(0, length.to_millimetre() as i32);
    let bottom_left = top_left.add(down);
    let bottom_right = top_right.add(down);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}
