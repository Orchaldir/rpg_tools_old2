use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use crate::rendering::hair::hairline::add_hairlines;
use crate::rendering::head::render_realistic_with_option;
use rpg_tools_core::model::character::appearance::hair::{HairColor, Hairline};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::size::Size;

pub fn render_buzz_cut_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
    color: HairColor,
) {
    let options = RenderOptions::no_line(config.hair.get_color(color));
    let line = config.get_line_options(1.0);
    let polygon = get_buzz_cut_realistic(config, aabb, realistic, hairline);

    renderer.render_polygon(&polygon, &options);
    render_realistic_with_option(renderer, config, aabb, line, realistic);
}

fn get_buzz_cut_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
) -> Polygon2d {
    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) = aabb.get_mirrored_points(
        config.head.get_forehead_width(realistic),
        config.head.y_forehead,
    );
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(
        config.head.get_eye_width_realistic(realistic),
        config.head.y_eye,
    );
    let mut corners = vec![top_left, forehead_left, bottom_left];

    add_hairlines(config, aabb, hairline, &mut corners);

    corners.push(bottom_right);
    corners.push(forehead_right);
    corners.push(top_right);

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_flat_top_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
    size: Size,
) -> Polygon2d {
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);
    let top_width = config.head.get_top_width(realistic);
    let flattop_width = forehead_width.max(top_width);
    let flattop_y = config.hair.short.get_flattop_y(size);

    let (top_left, top_right) = aabb.get_mirrored_points(flattop_width, flattop_y);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = get_inner_points(config, aabb, bottom_width);

    let mut corners = vec![top_left, forehead_left, bottom_left, inner_left];

    add_hairlines(config, aabb, hairline, &mut corners);

    corners.append(&mut vec![
        inner_right,
        bottom_right,
        forehead_right,
        top_right,
    ]);

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_middle_part_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
) -> Polygon2d {
    let hairline_y = config.hair.short.get_middle_y(hairline.get_y_position());
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);

    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = get_inner_points(config, aabb, bottom_width);
    let (hairline_left, hairline_right) =
        get_hairline_width(config, aabb, forehead_width, hairline_y);
    let (left, right) = aabb.get_mirrored_points(0.0, hairline_y);
    let center = aabb.get_point(0.5, 0.0);

    let mut polygon = Polygon2d::new(vec![
        top_left,
        forehead_left,
        bottom_left,
        inner_left,
        hairline_left,
        left,
        center,
        right,
        hairline_right,
        inner_right,
        bottom_right,
        forehead_right,
        top_right,
    ]);
    polygon = polygon.resize(1.1);
    config.cut_corners(&polygon).unwrap()
}

pub fn get_side_part_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    side: Side,
) -> Polygon2d {
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);
    let side_part_horizontal = config
        .hair
        .short
        .get_side_part_horizontal(side, forehead_width);

    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = get_inner_points(config, aabb, bottom_width);
    let (hairline_left, hairline_right) =
        get_hairline_width(config, aabb, forehead_width, config.head.y_forehead);
    let side_part = aabb.get_point(side_part_horizontal, config.head.y_forehead - 0.1);

    let mut corners = vec![
        top_left,
        forehead_left,
        bottom_left,
        inner_left,
        hairline_left,
    ];

    match side {
        Side::Left => {
            corners.push(hairline_right);
            corners.push(side_part);
        }
        Side::Right => {
            corners.push(side_part);
            corners.push(hairline_left);
        }
    }

    corners.append(&mut vec![
        hairline_right,
        inner_right,
        bottom_right,
        forehead_right,
        top_right,
    ]);
    let mut polygon = Polygon2d::new(corners);
    polygon = polygon.resize(1.1);
    config.cut_corners(&polygon).unwrap()
}

fn get_hairline_width(
    config: &RenderConfig,
    aabb: &AABB,
    forehead_width: f32,
    y: f32,
) -> (Point2d, Point2d) {
    aabb.get_mirrored_points(forehead_width * config.hair.short.hairline_width, y)
}

fn get_inner_points(config: &RenderConfig, aabb: &AABB, bottom_width: f32) -> (Point2d, Point2d) {
    aabb.get_mirrored_points(
        bottom_width * config.hair.short.inner_width,
        config.head.y_eye,
    )
}
