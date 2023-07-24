use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::color::Color;

pub fn render_eyes(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_eye_width(head.shape);
    let radius = config.eye.get_eye_radius(aabb, head_width_factor);

    match &head.eyes {
        Eyes::None => {}
        Eyes::One { eye } => {
            let center = aabb.get_point(0.5, config.head.y_eye);
            render_eye(renderer, config, &center, radius, eye);
        }
        Eyes::Two { eye, distance } => {
            let distance_between_eyes = config
                .eye
                .get_distance_between_eyes(*distance, head_width_factor);
            let (left, right) = aabb.get_mirrored_points(distance_between_eyes, config.head.y_eye);

            render_eye(renderer, config, &left, radius, eye);
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
        Eye::Normal {
            eye_shape,
            pupil_shape,
            pupil_color,
            background_color,
        } => {
            render_eye_shape(
                renderer,
                config,
                center,
                radius,
                *eye_shape,
                *background_color,
            );
            render_pupil(
                renderer,
                config,
                center,
                radius,
                *eye_shape,
                *pupil_shape,
                *pupil_color,
            );
        }
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
    let options = config.without_line(color);
    let half_height = config.eye.get_half_height(shape, radius);

    match shape {
        EyeShape::Almond => renderer.render_pointed_oval(center, radius, half_height, &options),
        EyeShape::Circle => renderer.render_circle(center, radius, &options),
        EyeShape::Ellipse => renderer.render_ellipse(center, radius, half_height, &options),
    }
}

fn render_pupil(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
    eye_shape: EyeShape,
    pupil_shape: PupilShape,
    color: Color,
) {
    let options = config.without_line(color);
    let slit_width = config.eye.get_slit_width(radius);

    match pupil_shape {
        PupilShape::Circle => renderer.render_circle(
            center,
            config.eye.get_circle_radius(eye_shape, radius),
            &options,
        ),
        PupilShape::VerticalSlit => {
            let half_height = config.eye.get_half_height(eye_shape, radius);
            renderer.render_pointed_oval(center, slit_width, half_height, &options)
        }
        PupilShape::HorizontalSlit => {
            renderer.render_pointed_oval(center, radius, slit_width, &options)
        }
    }
}
