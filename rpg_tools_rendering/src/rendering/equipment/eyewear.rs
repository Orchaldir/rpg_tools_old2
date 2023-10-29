use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::equipment::appearance::eyewear::{Eyewear, LensShape, LensStyle};

pub fn render_eyewear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyewear: &Eyewear,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
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
    let options = config.without_line(style.lens_color);
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
