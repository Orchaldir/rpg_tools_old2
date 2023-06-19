use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::Size;
use rpg_tools_core::model::color::Color;

pub fn render_mouth(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_mouth_width(head.shape);
    let head_width = aabb.calculate_from_height(head_width_factor);

    match &head.mouth {
        Mouth::None => {}
        Mouth::Circle {
            size,
            teeth,
            teeth_color,
        } => {
            let free_y = aabb.calculate_from_height(1.0 - config.head.y_eye);
            let max_free_space = head_width.min(free_y);
            let center = aabb.get_point(0.5, config.head.y_mouth);
            let radius = get_circle_radius(max_free_space, *size);

            render_circular_mouth(renderer, config, &center, radius);
        }
        Mouth::Normal {
            width,
            height,
            color,
            teeth,
        } => {
            let width = get_width(head_width_factor, *width);
            render_normal_mouth(renderer, config, aabb, width, height);
        }
    }
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

fn get_thickness(size: Size) -> f32 {
    match size {
        Size::Low => 0.4,
        Size::Medium => 0.6,
        Size::High => 0.8,
    }
}

pub fn render_circular_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
) {
    let options = config.without_line(Color::Black);
    renderer.render_circle(center, radius, &options);
}

fn render_normal_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    width: f32,
    height: &Size,
) {
    let options = config.get_line_options(get_thickness(*height));
    let line: Line2d = aabb.get_mirrored_points(width, config.head.y_mouth).into();
    renderer.render_line(&line, &options);
}
