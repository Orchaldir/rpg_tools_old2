use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use crate::rendering::head::render_realistic_with_option;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::Size;

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
                ShortHair::MiddlePart => {
                    render_middle_part_realistic(renderer, config, aabb, realistic, color)
                }
                ShortHair::SidePart(_) => {}
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

fn render_middle_part_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    let mut polygon = get_middle_part_realistic(config, aabb, realistic);
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

    match hairline {
        Hairline::Round(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, &mut corners, hairline_y, 0.4);
        }
        Hairline::Straight(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, &mut corners, hairline_y, 0.6);
        }
        Hairline::Triangle(size) => {
            let hairline_y = get_hairline_y(size);
            add_hairline(aabb, &mut corners, hairline_y, 0.2);
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

    corners.push(bottom_right);
    corners.push(forehead_right);
    corners.push(top_right);

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}

fn get_middle_part_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
) -> Polygon2d {
    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        aabb.get_mirrored_points(config.head.get_forehead_width(realistic), 0.35);

    let (left, right) = aabb.get_mirrored_points(0.0, config.head.y_forehead);
    let center = aabb.get_point(0.5, 0.0);

    let mut polygon = Polygon2d::new(vec![
        top_left,
        forehead_left,
        left,
        center,
        center,
        right,
        forehead_right,
        top_right,
    ]);
    polygon = polygon.resize(1.05);
    config.cut_corners(&polygon).unwrap()
}

fn get_hairline_y(size: Size) -> f32 {
    match size {
        Size::Low => 0.3,
        Size::Medium => 0.2,
        Size::High => 0.1,
    }
}

fn add_hairline(aabb: &AABB, corners: &mut Vec<Point2d>, hairline_y: f32, width: f32) {
    let (left, right) = aabb.get_mirrored_points(width, hairline_y);

    corners.push(left);
    corners.push(right);
}
