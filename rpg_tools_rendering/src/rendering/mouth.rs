use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::color::WebColor;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::Size;
use rpg_tools_core::model::color::Color;

pub fn render_mouth(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_mouth_width(head.shape);
    let head_width = aabb.calculate_from_height(head_width_factor);

    match &head.mouth {
        Mouth::None => {}
        Mouth::Circle { size, teeth_color } => {
            let free_y = aabb.calculate_from_height(1.0 - config.head.y_eye);
            let max_free_space = head_width.min(free_y);
            let center = aabb.get_point(0.5, config.head.y_mouth);
            let radius = get_circle_radius(max_free_space, *size);

            render_circular_mouth(renderer, config, &center, radius, *teeth_color);
        }
        Mouth::Normal {
            width,
            color,
            teeth,
        } => {
            let width = get_width(head_width_factor, *width);
            let distance_between_fangs = width * 0.6;
            let down = Orientation::from_degree(90.0);
            let up = Orientation::from_degree(270.0);

            render_normal_mouth(renderer, config, aabb, width);

            match teeth.special {
                SpecialTeeth::UpperFangs(size) => {
                    render_2_fangs(
                        renderer,
                        &config,
                        &aabb,
                        down,
                        distance_between_fangs,
                        size,
                        TeethColor::White,
                    );
                }
                SpecialTeeth::LowerFangs(size) => {
                    render_2_fangs(
                        renderer,
                        &config,
                        &aabb,
                        up,
                        distance_between_fangs,
                        size,
                        TeethColor::White,
                    );
                }
                _ => {}
            }
        }
    }
}

fn render_2_fangs(
    renderer: &mut dyn Renderer,
    config: &&RenderConfig,
    aabb: &&AABB,
    down: Orientation,
    distance_between_fangs: f32,
    size: Size,
    color: TeethColor,
) {
    let head_height = aabb.size().height();
    let fang_height = 2.0 * get_fang_width(size) * head_height as f32;

    render_fang(
        renderer,
        config,
        fang_height,
        &aabb.get_point(0.5 - distance_between_fangs / 2.0, config.head.y_mouth),
        down,
        color,
    );
    render_fang(
        renderer,
        config,
        fang_height,
        &aabb.get_point(0.5 + distance_between_fangs / 2.0, config.head.y_mouth),
        down,
        color,
    );
}

fn get_circle_radius(head_width: u32, size: Size) -> u32 {
    (head_width as f32
        * match size {
            Size::Low => 0.2,
            Size::Medium => 0.25,
            Size::High => 0.3,
        }) as u32
}

fn get_width(head_width: f32, size: Size) -> f32 {
    head_width
        * match size {
            Size::Low => 0.4,
            Size::Medium => 0.5,
            Size::High => 0.6,
        }
}

pub fn render_circular_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
    color: TeethColor,
) {
    let options = config.without_line(Color::Black);
    renderer.render_circle(center, radius, &options);

    let n = 16;
    let step = Orientation::split(n);
    let mut orientation = Orientation::from_degree(0.0);
    let fang_height = 0.4 * radius as f32;

    for _i in 0..n {
        render_fang(
            renderer,
            config,
            fang_height,
            &center.calculate_polar(radius as f32, orientation),
            orientation.inverse(),
            color,
        );

        orientation = orientation + step;
    }
}

fn render_normal_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    width: f32,
) {
    let options = config.get_line_options(0.6);
    let line: Line2d = aabb.get_mirrored_points(width, config.head.y_mouth).into();
    renderer.render_line(&line, &options);
}

fn render_fang(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    fang_height: f32,
    point: &Point2d,
    orientation: Orientation,
    teeth_color: TeethColor,
) {
    let fang_width = fang_height * 0.5;
    let fang_half = fang_width * 0.5;

    let normal = orientation.normal();

    let left = point.calculate_polar(-fang_half, normal);
    let right = point.calculate_polar(fang_half, normal);
    let tip = point.calculate_polar(fang_height, orientation);
    let polygon = Polygon2d::new(vec![left, tip, right]);

    let options = RenderOptions::no_line(get_teeth_color(teeth_color));

    renderer.render_polygon(&polygon, &options);
}

fn get_fang_width(size: Size) -> f32 {
    match size {
        Size::Low => 0.04,
        Size::Medium => 0.06,
        Size::High => 0.08,
    }
}

fn get_teeth_color(color: TeethColor) -> WebColor {
    match color {
        TeethColor::White => WebColor::from_rgb(255, 255, 255),
        TeethColor::Yellow => WebColor::from_rgb(249, 232, 158),
        TeethColor::Brown => WebColor::Name("brown".to_string()),
        TeethColor::Black => WebColor::from_rgb(0, 0, 0),
    }
}
