use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::color::WebColor;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::appearance::side::Side;
use rpg_tools_core::model::equipment::appearance::eyewear::{Eyewear, LensShape, LensStyle};

pub fn render_eyewear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyewear: &Eyewear,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    let radius = config.eyewear.get_radius(radius);

    match eyewear {
        Eyewear::None => {}
        Eyewear::Glasses { style } => render_glasses(renderer, config, style, left, right, radius),
        Eyewear::Monocle { style, side } => match side {
            Side::Left => render_lens(renderer, config, style, left, radius),
            Side::Right => render_lens(renderer, config, style, right, radius),
        },
    }
}

fn render_glasses(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    style: &LensStyle,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    render_lens(renderer, config, style, left, radius);
    render_lens(renderer, config, style, right, radius);
    render_bridge(renderer, config, style, left, right, radius);
}

fn render_bridge(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    style: &LensStyle,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    let options = config.line_with_color(
        style.frame_color,
        config.eyewear.get_bridge_thickness(style.frame_type),
    );
    let start = Point2d::new(left.x + radius as i32, left.y);
    let end = Point2d::new(right.x - radius as i32, right.y);

    renderer.render_line(&(start, end).into(), &options);
}

fn render_lens(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    style: &LensStyle,
    center: &Point2d,
    radius: u32,
) {
    let options = create_frame_config(
        config,
        style,
        config.eyewear.get_frame_thickness(style.frame_type),
    );
    let radius_y = config.eyewear.get_radius_y(radius);

    match style.lens_shape {
        LensShape::Circle => renderer.render_circle(center, radius, &options),
        LensShape::Oval => renderer.render_ellipse(center, radius, radius_y, &options),
        LensShape::Rectangle => {
            let aabb = AABB::with_radii(*center, radius, radius_y);
            renderer.render_rectangle(&aabb, &options);
        }
    }
}

fn create_frame_config(config: &RenderConfig, style: &LensStyle, thickness: f32) -> RenderOptions {
    RenderOptions::new(
        config
            .color
            .get_transparent_color(style.lens_color, style.lens_transparency),
        WebColor::from_color(style.frame_color),
        (config.line_width as f32 * thickness) as u32,
    )
}
