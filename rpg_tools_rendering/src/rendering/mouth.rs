use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::beard::{render_beard_behind_mouth, render_beard_in_front_of_mouth};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::size::Size;
use Color::Black;

pub fn render_mouth(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_mouth_width(head.shape);
    let head_width = aabb.calculate_from_height(head_width_factor);

    match &head.mouth {
        Mouth::None => {}
        Mouth::Circle { size, teeth_color } => {
            let free_y = aabb.calculate_from_height(1.0 - config.head.y_eye);
            let max_free_space = head_width.min(free_y);
            let center = aabb.get_point(0.5, config.head.y_mouth);
            let radius = config
                .mouth
                .circular
                .get_mouth_radius(max_free_space, *size);

            render_circular_mouth(renderer, config, &center, radius, *teeth_color);
        }
        Mouth::Simple {
            beard,
            width,
            teeth,
            teeth_color,
        } => {
            let width = config.mouth.get_mouth_width(head_width_factor, *width);

            render_beard_behind_mouth(renderer, config, aabb, head.shape, head, beard);
            render_simple_mouth(renderer, config, aabb, width);
            render_special_teeth(renderer, &config, &aabb, teeth, teeth_color, width);
            render_beard_in_front_of_mouth(renderer, config, aabb, beard, width);
        }
        Mouth::Female {
            width,
            color,
            teeth,
            teeth_color,
        } => {
            let width = config.mouth.get_mouth_width(head_width_factor, *width);

            render_female_mouth(renderer, config, aabb, width, *color);
            render_special_teeth(renderer, &config, &aabb, teeth, teeth_color, width);
        }
    }
}

fn render_special_teeth(
    renderer: &mut dyn Renderer,
    config: &&RenderConfig,
    aabb: &&AABB,
    teeth: &SpecialTeeth,
    teeth_color: &TeethColor,
    width: f32,
) {
    let distance_between_fangs = config.mouth.get_distance_between_fangs(width);
    let down = Orientation::from_degree(90.0);
    let up = Orientation::from_degree(270.0);

    match teeth {
        SpecialTeeth::UpperFangs(size) => {
            render_2_fangs(
                renderer,
                &config,
                &aabb,
                down,
                distance_between_fangs,
                *size,
                *teeth_color,
            );
        }
        SpecialTeeth::LowerFangs(size) => {
            render_2_fangs(
                renderer,
                &config,
                &aabb,
                up,
                distance_between_fangs,
                *size,
                *teeth_color,
            );
        }
        _ => {}
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
    let fang_height = config.mouth.get_fang_height(head_height, size);

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

pub fn render_circular_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
    color: TeethColor,
) {
    let options = config.without_line(Black);
    renderer.render_circle(center, radius, &options);

    let n = 16;
    let step = Orientation::split(n);
    let mut orientation = Orientation::from_degree(0.0);
    let fang_height = config.mouth.circular.get_fang_height(radius);

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

fn render_simple_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    width: f32,
) {
    let options = config.line_with_color(Black, 1.0);
    let line: Line2d = aabb.get_mirrored_points(width, config.head.y_mouth).into();
    renderer.render_line(&line, &options);
}

fn render_female_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    width: f32,
    color: Color,
) {
    let options = config.without_line(color);
    let half_height = 0.04;
    let (left, right) = aabb.get_mirrored_points(width, config.head.y_mouth);
    let (top_left, top_right) =
        aabb.get_mirrored_points(width * 0.5, config.head.y_mouth - half_height);
    let (bottom_left, bottom_right) =
        aabb.get_mirrored_points(width * 0.5, config.head.y_mouth + half_height);
    let top_center = aabb.get_point(0.5, config.head.y_mouth - half_height);
    let cupids_bow = aabb.get_point(0.5, config.head.y_mouth - half_height / 2.0);

    let corners = vec![
        left,
        left,
        bottom_left,
        bottom_right,
        right,
        right,
        top_right,
        //top_center,
        cupids_bow,
        cupids_bow,
        //top_center,
        top_left,
    ];

    let polygon = Polygon2d::new(corners);
    let polygon = config.cut_corners(&polygon).unwrap();

    renderer.render_polygon(&polygon, &options);
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

    let options = config.get_teeth_options(teeth_color);

    renderer.render_polygon(&polygon, &options);
}
