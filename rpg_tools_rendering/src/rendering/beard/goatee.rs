use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;

pub fn get_goat_patch(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let width = mouth_width * 0.8;
    let mut polygon = get_simple(config, aabb, 1.05 - config.head.y_mouth, 0.01, width, width);

    polygon.create_sharp_corner(0);
    polygon.create_sharp_corner(4);

    config.cut_corners(&polygon).unwrap()
}

pub fn get_goatee(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let width = mouth_width * 0.8;
    let diff_y = 1.0 - config.head.y_mouth - 0.05;
    let polygon = get_simple(config, aabb, 0.15, diff_y, width, width);

    config.cut_corners(&polygon).unwrap()
}

pub fn get_soul_patch(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let height = 0.1;
    get_simple(config, aabb, height, 0.01, height, height)
}

pub fn get_van_dyke(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let width = 0.1;
    let mut polygon = get_simple(config, aabb, 1.05 - config.head.y_mouth, 0.01, width, width);

    polygon.create_sharp_corner(0);
    polygon.create_sharp_corner(4);

    config.cut_corners(&polygon).unwrap()
}

fn get_simple(
    config: &RenderConfig,
    aabb: &AABB,
    height: f32,
    offset: f32,
    top_width: f32,
    bottom_width: f32,
) -> Polygon2d {
    let top_y = config.head.y_mouth + offset;
    let bottom_y = top_y + height;
    let (top_left, top_right) = aabb.get_mirrored_points(top_width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
