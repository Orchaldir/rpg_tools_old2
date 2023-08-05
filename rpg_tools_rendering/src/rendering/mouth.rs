use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
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
        Mouth::Normal {
            beard,
            width,
            teeth,
            teeth_color,
        } => {
            let width = config.mouth.get_mouth_width(head_width_factor, *width);
            let distance_between_fangs = config.mouth.get_distance_between_fangs(width);
            let down = Orientation::from_degree(90.0);
            let up = Orientation::from_degree(270.0);

            render_normal_mouth(renderer, config, aabb, width);

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

fn render_normal_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    width: f32,
) {
    let options = config.line_with_color(Black, 0.6);
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

    let options = config.get_teeth_options(teeth_color);

    renderer.render_polygon(&polygon, &options);
}
