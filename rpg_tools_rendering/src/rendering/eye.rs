use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeDistance, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::color::Color;

pub fn render_eyes(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_eye_width(head.shape);
    let head_width = (aabb.size().height() as f32 * head_width_factor) as u32;
    let radius = head_width / 8;

    match &head.eyes {
        Eyes::None => {}
        Eyes::One(eye) => {
            let center = aabb.get_point(0.5, config.head.y_eye);
            render_eye(renderer, config, &center, radius, eye);
        }
        Eyes::Two { eye, distance } => {
            let head_half = head_width_factor / 2.0;
            let eye_offset = head_half * get_eye_x_scale(*distance);

            let left = aabb.get_point(0.5 - eye_offset, config.head.y_eye);
            render_eye(renderer, config, &left, radius, eye);

            let right = aabb.get_point(0.5 + eye_offset, config.head.y_eye);
            render_eye(renderer, config, &right, radius, eye);
        }
    }
}

fn get_eye_x_scale(distance: EyeDistance) -> f32 {
    match distance {
        EyeDistance::Low => 0.35,
        EyeDistance::Medium => 0.4,
        EyeDistance::High => 0.45,
    }
}

fn render_eye(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
    eye: &Eye,
) {
    match eye {
        Eye::Simple { eye_shape, color } => {
            render_eye_shape(renderer, config, center, radius, *eye_shape, *color)
        }
        Eye::Normal { .. } => {}
    }
}

fn render_eye_shape(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
    shape: EyeShape,
    color: Color,
) {
    let options = config.get_options(color);

    match shape {
        EyeShape::Almond => {}
        EyeShape::Ellipse => renderer.render_ellipse(center, radius, radius / 2, &options),
        EyeShape::Round => renderer.render_circle(center, radius, &options),
    }
}
