use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;

pub fn get_toothbrush(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let height = 0.1;
    let width = 0.1;
    let center_y = config.head.get_moustache_y();
    let top_y = center_y - height / 2.0;
    let bottom_y = center_y + height / 2.0;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
