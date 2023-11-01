use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::renderer::color::WebColor;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::equipment::appearance::eyewear::{Eyewear, LensShape, LensStyle};
use rpg_tools_core::model::side::Side;
use std::ops::{Add, Div};

pub fn render_eyewear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyewear: &Eyewear,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    let radius = (radius as f32 * 1.2) as u32;
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
    let width = left.calculate_distance(right) as u32 - 2 * radius;
    let height =
        (width as f32 * 0.1 * config.eyewear.get_bridge_thickness(style.frame_type)) as u32;
    let center = left.add(*right).div(2.0);
    let aabb = AABB::with_center(center, Size2d::new(width, height));
    let options = RenderOptions::no_line(WebColor::from_color(style.frame_color));

    renderer.render_rectangle(&aabb, &options);
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
    let radius_y = (radius as f32 * 0.8) as u32;

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
