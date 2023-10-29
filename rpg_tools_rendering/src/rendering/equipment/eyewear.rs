use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::color::WebColor;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::equipment::appearance::eyewear::{
    Eyewear, FrameType, LensShape, LensStyle,
};

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
        Eyewear::Monocle { .. } => {}
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
}

fn render_lens(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    style: &LensStyle,
    center: &Point2d,
    radius: u32,
) {
    let options = match style.frame_type {
        FrameType::Horn => create_frame_config(config, style, 6.0),
        FrameType::FullRimmed => create_frame_config(config, style, 3.0),
        FrameType::Wire => create_frame_config(config, style, 1.0),
        FrameType::Rimless => config.without_line(style.lens_color),
    };
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
        WebColor::from_color(style.lens_color),
        WebColor::from_color(style.frame_color),
        (config.line_width as f32 * thickness) as u32,
    )
}
