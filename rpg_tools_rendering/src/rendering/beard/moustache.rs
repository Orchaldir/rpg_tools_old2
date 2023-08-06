use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::rendering::config::RenderConfig;

pub fn get_fu_manchu(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let thickness = 0.05;
    let delta_y = 0.02;
    let outer_width = mouth_width + 2.0 * thickness;
    let top_y = config.head.y_mouth - thickness - delta_y;
    let bottom_y = 1.1;
    let (top_left, top_right) = aabb.get_mirrored_points(outer_width, top_y);
    let (mouth_left, mouth_right) =
        aabb.get_mirrored_points(mouth_width, config.head.y_mouth - delta_y);
    let (outer_left, outer_right) = aabb.get_mirrored_points(outer_width, bottom_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(mouth_width, bottom_y);
    let corners = vec![
        top_left,
        outer_left,
        bottom_left,
        mouth_left,
        mouth_right,
        bottom_right,
        outer_right,
        top_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_handlebar(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let thickness = 0.05;
    let width = mouth_width + 2.0 * thickness;
    let center_y = config.head.y_mouth - thickness;
    let bottom_y = config.head.y_mouth + thickness;
    let inner_y = bottom_y - thickness;
    let top_y = inner_y - 0.1;
    let bottom = aabb.get_point(0.5, center_y);
    let top = aabb.get_point(0.5, center_y - thickness);
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (inner_left, inner_right) = aabb.get_mirrored_points(mouth_width, inner_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(width, bottom_y);
    let corners = vec![
        top,
        inner_left,
        top_left,
        bottom_left,
        bottom,
        bottom_right,
        top_right,
        inner_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

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
    let width = 0.08;
    let bottom_y = config.head.y_mouth - 0.01;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(mouth_width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}

pub fn get_toothbrush(config: &RenderConfig, aabb: &AABB) -> Polygon2d {
    let height = 0.08;
    let width = 0.08;
    let bottom_y = config.head.y_mouth - 0.01;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(width, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    Polygon2d::new(corners)
}

pub fn get_walrus(config: &RenderConfig, aabb: &AABB, mouth_width: f32) -> Polygon2d {
    let height = 0.1;
    let width = mouth_width + 0.1;
    let bottom_y = config.head.y_mouth - 0.01;
    let top_y = bottom_y - height;
    let (top_left, top_right) = aabb.get_mirrored_points(width, top_y);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(width, bottom_y);
    let corners = vec![
        top_left,
        bottom_left,
        bottom_left,
        bottom_right,
        bottom_right,
        top_right,
    ];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}
