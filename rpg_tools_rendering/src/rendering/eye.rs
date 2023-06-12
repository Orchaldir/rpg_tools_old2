use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::head::{
    GeometricHeadShape, Head, HeadShape, RealisticHeadShape,
};
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
            let eye_x_scale = 0.50;
            let head_half = head_width_factor / 2.0;
            let eye_offset = head_half * eye_x_scale;

            let left = aabb.get_point(0.5 - eye_offset, config.head.y_eye);
            render_eye(renderer, config, &left, radius, eye);

            let right = aabb.get_point(0.5 + eye_offset, config.head.y_eye);
            render_eye(renderer, config, &right, radius, eye);
        }
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

    renderer.render_circle(center, radius, &options);
}
