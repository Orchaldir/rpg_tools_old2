use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use crate::rendering::head::render_realistic_with_option;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::{Side, Size};

pub fn render_hair(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    match head.shape {
        HeadShape::Geometric(_) => {}
        HeadShape::Realistic(realistic) => match head.hair {
            Hair::None => {}
            Hair::Short {
                style,
                hairline,
                color,
            } => match style {
                ShortHair::BuzzCut => {
                    render_buzz_cut_realistic(renderer, config, aabb, realistic, hairline, color)
                }
                ShortHair::FlatTop(_) => {
                    render_flat_top_realistic(renderer, config, aabb, realistic, hairline, color)
                }
                ShortHair::MiddlePart => {
                    render_middle_part_realistic(renderer, config, aabb, realistic, hairline, color)
                }
                ShortHair::SidePart(side) => {
                    render_side_part_realistic(renderer, config, aabb, realistic, side, color)
                }
            },
        },
    }
}

fn render_buzz_cut_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
    color: HairColor,
) {
    let options = RenderOptions::no_line(config.get_hair_color(color));
    let line = config.get_line_options(1.0);
    let polygon = get_cut_realistic(config, aabb, realistic, hairline);

    renderer.render_polygon(&polygon, &options);
    render_realistic_with_option(renderer, config, aabb, line, realistic);
}

fn render_flat_top_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    let mut polygon = get_flat_top_realistic(config, aabb, realistic, hairline);
    polygon = polygon.resize(1.03);
    renderer.render_polygon(&polygon, &options);
}

fn render_middle_part_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    let mut polygon = get_middle_part_realistic(config, aabb, realistic, hairline);
    polygon = polygon.resize(1.03);
    renderer.render_polygon(&polygon, &options);
}

fn render_side_part_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    side: Side,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    let mut polygon = get_side_part_realistic(config, aabb, realistic, side);
    polygon = polygon.resize(1.03);
    renderer.render_polygon(&polygon, &options);
}

fn get_cut_realistic(
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

    add_hairlines(aabb, hairline, &mut corners);

    corners.push(bottom_right);
    corners.push(forehead_right);
    corners.push(top_right);

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

fn get_flat_top_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
) -> Polygon2d {
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);
    let top_width = config.head.get_top_width(realistic);
    let flattop_width = forehead_width.max(top_width);

    let (top_left, top_right) = aabb.get_mirrored_points(flattop_width, -0.1);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = aabb.get_mirrored_points(bottom_width * 0.8, config.head.y_eye);

    let mut corners = vec![top_left, forehead_left, bottom_left, inner_left];

    add_hairlines(aabb, hairline, &mut corners);

    corners.append(&mut vec![
        inner_right,
        bottom_right,
        forehead_right,
        top_right,
    ]);

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

fn get_middle_part_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    hairline: Hairline,
) -> Polygon2d {
    let hairline_y = get_middle_y(hairline.get_y_position());
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);

    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = aabb.get_mirrored_points(bottom_width * 0.8, config.head.y_eye);
    let (hairline_left, hairline_right) =
        aabb.get_mirrored_points(forehead_width * 0.6, hairline_y);
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

fn get_side_part_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    side: Side,
) -> Polygon2d {
    let bottom_width = config.head.get_eye_width_realistic(realistic);
    let forehead_width = config.head.get_forehead_width(realistic);
    let side_part_horizontal = get_side_part_horizontal(side, forehead_width);

    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(forehead_width, config.head.y_forehead);
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(bottom_width, config.head.y_eye);
    let (inner_left, inner_right) = aabb.get_mirrored_points(bottom_width * 0.8, config.head.y_eye);
    let (hairline_left, hairline_right) =
        aabb.get_mirrored_points(forehead_width * 0.6, config.head.y_forehead);
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

fn get_side_part_horizontal(side: Side, forehead_width: f32) -> f32 {
    match side {
        Side::Left => 0.5 + forehead_width * 0.3,
        Side::Right => 0.5 - forehead_width * 0.3,
    }
}

fn get_hairline_y(size: Size) -> f32 {
    match size {
        Size::Low => 0.25,
        Size::Medium => 0.2,
        Size::High => 0.15,
    }
}

fn get_middle_y(size: Size) -> f32 {
    match size {
        Size::Low => 0.35,
        Size::Medium => 0.3,
        Size::High => 0.25,
    }
}

fn add_hairlines(aabb: &AABB, hairline: Hairline, corners: &mut Vec<Point2d>) {
    match hairline {
        Hairline::Round(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, corners, hairline_y, 0.4);
        }
        Hairline::Straight(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, corners, hairline_y, 0.6);
        }
        Hairline::Triangle(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, corners, hairline_y, 0.2);
        }
        Hairline::WidowsPeak(size) => {
            let hairline_y = get_hairline_y(size);
            let (left, right) = aabb.get_mirrored_points(0.4, hairline_y);
            let center = aabb.get_point(0.5, hairline_y + 0.1);

            corners.push(left);
            corners.push(center);
            corners.push(right);
        }
    }
}

fn add_hairline(aabb: &AABB, corners: &mut Vec<Point2d>, hairline_y: f32, width: f32) {
    let (left, right) = aabb.get_mirrored_points(width, hairline_y);

    corners.push(left);
    corners.push(right);
}
