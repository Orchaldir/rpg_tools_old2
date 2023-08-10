use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::side::Side::{Left, Right};

pub fn render_eyebrow(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    aabb: &AABB,
) {
    match eyebrow {
        EyeBrows::Normal {
            color,
            shape,
            style,
        }
        | EyeBrows::Unibrow {
            color,
            shape,
            style,
        } => {
            let options = config.with_thickness(*color, 0.5);
            let polygon = get_normal_eyebrow(config, *shape, *style, aabb, None);
            renderer.render_polygon(&polygon, &options);
        }
        _ => {}
    }
}

pub fn render_eyebrows(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    left: &AABB,
    right: &AABB,
) {
    match eyebrow {
        EyeBrows::Normal {
            color,
            shape,
            style,
        } => {
            let options = config.with_thickness(*color, 0.5);
            let polygon_left = get_normal_eyebrow(config, *shape, *style, left, Some(Left));
            let polygon_right = get_normal_eyebrow(config, *shape, *style, right, Some(Right));
            renderer.render_polygon(&polygon_left, &options);
            renderer.render_polygon(&polygon_right, &options);
        }
        EyeBrows::Unibrow {
            color,
            shape,
            style,
        } => {}
        _ => {}
    }
}

fn get_normal_eyebrow(
    config: &RenderConfig,
    shape: EyebrowShape,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    match shape {
        EyebrowShape::Angled => get_angled_eyebrow(config, style, aabb, side),
        EyebrowShape::Curved => get_smooth_curved_eyebrow(config, style, aabb, side),
        EyebrowShape::Straight => get_straight_eyebrow(config, style, aabb, side),
    }
}

fn get_angled_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    let mut polygon = get_curved_eyebrow(config, style, aabb, side);

    polygon.create_sharp_corner(4);
    polygon.create_sharp_corner(1);

    config.cut_corners(&polygon).unwrap()
}

fn get_smooth_curved_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    let polygon = get_curved_eyebrow(config, style, aabb, side);
    config.cut_corners(&polygon).unwrap()
}

fn get_curved_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    let side_y = -config.eye.eyebrow.distance_to_eye;
    let center_y = side_y - 0.2;
    let left_width = config.eye.eyebrow.get_left_thickness(style, side);
    let center_width = config.eye.eyebrow.get_center_thickness(style);
    let right_width = config.eye.eyebrow.get_right_thickness(style, side);

    let top_left = aabb.get_point(0.0, side_y - left_width);
    let top_center = aabb.get_point(0.5, center_y - center_width);
    let top_right = aabb.get_point(1.0, side_y - right_width);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(1.0, side_y);
    let bottom_center = aabb.get_point(0.5, center_y);

    let corners = vec![
        bottom_left,
        bottom_center,
        bottom_right,
        top_right,
        top_center,
        top_left,
    ];

    Polygon2d::new(corners)
}

fn get_straight_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    let bottom_y = -config.eye.eyebrow.distance_to_eye;
    let left_width = config.eye.eyebrow.get_left_thickness(style, side);
    let right_width = config.eye.eyebrow.get_right_thickness(style, side);
    let top_left = aabb.get_point(0.0, bottom_y - left_width);
    let top_right = aabb.get_point(1.0, bottom_y - right_width);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(1.0, bottom_y);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    config.cut_corners(&Polygon2d::new(corners)).unwrap()
}
