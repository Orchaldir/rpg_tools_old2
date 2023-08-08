use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;

pub fn get_goat_patch(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let width = mouth_width * 0.8;
    let mut polygon = from_top_and_bottom(aabb, config.head.y_mouth + 0.01, 1.05, width, width);

    polygon.create_sharp_corner(0);
    polygon.create_sharp_corner(4);

    config.cut_corners(&polygon).unwrap()
}

pub fn get_goatee(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let width = mouth_width * 0.8;
    let polygon = from_top_and_bottom(aabb, 0.95, 1.10, width, width);

    config.cut_corners(&polygon).unwrap()
}

pub fn get_soul_patch(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let height = 0.1;
    from_top_and_bottom(
        aabb,
        config.head.y_mouth + 0.01,
        config.head.y_mouth + height,
        height,
        height,
    )
}

pub fn get_van_dyke(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let width = 0.1;
    let mut polygon = from_top_and_bottom(aabb, config.head.y_mouth + 0.01, 1.05, width, width);

    polygon.create_sharp_corner(0);
    polygon.create_sharp_corner(4);

    config.cut_corners(&polygon).unwrap()
}

fn from_top_and_bottom(
    aabb: &AABB,
    top_y: f32,
    bottom_y: f32,
    top_width: f32,
    bottom_width: f32,
) -> Polygon2d {
    let (top_left, top_right) = aabb.get_mirrored_points(top_width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}
