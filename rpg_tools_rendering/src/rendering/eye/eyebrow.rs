use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::side::Side::{Left, Right};
use rpg_tools_core::model::width::Width;

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
            width,
        }
        | EyeBrows::Unibrow {
            color,
            shape,
            style,
            width,
        } => {
            let options = config.without_line(*color);
            let polygon = get_eyebrow(config, *shape, *style, aabb, *width, None);
            renderer.render_smooth_polygon(&polygon, &options);
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
            width,
        } => {
            let options = config.without_line(*color);
            let polygon_left = get_eyebrow(config, *shape, *style, left, *width, Some(Left));
            let polygon_right = get_eyebrow(config, *shape, *style, right, *width, Some(Right));
            renderer.render_smooth_polygon(&polygon_left, &options);
            renderer.render_smooth_polygon(&polygon_right, &options);
        }
        EyeBrows::Unibrow {
            color,
            shape,
            style,
            width,
        } => {
            let options = config.without_line(*color);
            let polygon_left = get_eyebrow(config, *shape, *style, left, *width, Some(Left));
            let polygon_right = get_eyebrow(config, *shape, *style, right, *width, Some(Right));
            let index = polygon_left.corners().len() / 2;
            let polygon = polygon_left.insert(index, &polygon_right);
            renderer.render_smooth_polygon(&polygon, &options);
        }
        _ => {}
    }
}

fn get_eyebrow(
    config: &RenderConfig,
    shape: EyebrowShape,
    style: EyebrowStyle,
    aabb: &AABB,
    width: Width,
    side: Option<Side>,
) -> Polygon2d {
    match shape {
        EyebrowShape::Angled => get_angled_eyebrow(config, style, aabb, width, side),
        EyebrowShape::Curved => get_curved_eyebrow(config, style, aabb, width, side),
        EyebrowShape::Straight => get_straight_eyebrow(config, style, aabb, width, side),
    }
}

fn get_angled_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    width: Width,
    side: Option<Side>,
) -> Polygon2d {
    let mut polygon = get_curved_eyebrow(config, style, aabb, width, side);

    polygon.create_sharp_corner(4);
    polygon.create_sharp_corner(1);

    polygon
}

fn get_curved_eyebrow(
    config: &RenderConfig,
    style: EyebrowStyle,
    aabb: &AABB,
    width: Width,
    side: Option<Side>,
) -> Polygon2d {
    let side_y = -config.eye.eyebrow.distance_to_eye;
    let center_y = side_y - 0.2;
    let left_width = config.eye.eyebrow.get_left_thickness(style, width, side);
    let center_width = config.eye.eyebrow.get_center_thickness(width);
    let right_width = config.eye.eyebrow.get_right_thickness(style, width, side);

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
    width: Width,
    side: Option<Side>,
) -> Polygon2d {
    let bottom_y = -config.eye.eyebrow.distance_to_eye_straight;
    let left_width = config.eye.eyebrow.get_left_thickness(style, width, side);
    let right_width = config.eye.eyebrow.get_right_thickness(style, width, side);
    let top_left = aabb.get_point(0.0, bottom_y - left_width);
    let top_right = aabb.get_point(1.0, bottom_y - right_width);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(1.0, bottom_y);
    let corners = vec![bottom_left, bottom_right, top_right, top_left];

    Polygon2d::new(corners)
}
