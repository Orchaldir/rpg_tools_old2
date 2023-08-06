use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;

pub fn get_pencil(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let height = 0.02;
    let bottom_y = config.head.y_mouth - height;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(mouth_width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(mouth_width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}

pub fn get_pyramid(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let height = 0.08;
    let width = 0.1;
    let bottom_y = config.head.y_mouth - 0.01;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(mouth_width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}

pub fn get_toothbrush(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let height = 0.08;
    let width = 0.1;
    let bottom_y = config.head.y_mouth - 0.01;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
