use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::eye::eyebrow::{render_eyebrow, render_eyebrows};
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::color::Color;

pub mod eyebrow;

pub fn render_eyes(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_eye_width(head.shape);
    let radius = config.eye.get_eye_radius(aabb, head_width_factor);

    match &head.eyes {
        Eyes::None => {}
        Eyes::One { eye, eyebrow } => {
            let half_height = config.eye.get_half_height(eye.get_shape(), radius);
            let center = aabb.get_point(0.5, config.head.y_eye);
            let eye_aabb = AABB::with_radii(center, radius, half_height);
            render_eye(renderer, config, &eye_aabb, eye);
            render_eyebrow(renderer, config, eyebrow, &eye_aabb);
        }
        Eyes::Two {
            eye,
            eyebrows,
            distance,
        } => {
            let half_height = config.eye.get_half_height(eye.get_shape(), radius);
            let distance_between_eyes = config
                .eye
                .get_distance_between_eyes(*distance, head_width_factor);
            let (left, right) = aabb.get_mirrored_points(distance_between_eyes, config.head.y_eye);
            let left_aabb = AABB::with_radii(left, radius, half_height);
            let right_aabb = AABB::with_radii(right, radius, half_height);

            render_eye(renderer, config, &left_aabb, eye);
            render_eye(renderer, config, &right_aabb, eye);

            render_eyebrows(renderer, config, eyebrows, &left_aabb, &right_aabb);
        }
    }
}

fn render_eye(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, eye: &Eye) {
    match eye {
        Eye::Simple { eye_shape, color } => {
            render_eye_shape(renderer, config, aabb, *eye_shape, *color)
        }
        Eye::Normal {
            eye_shape,
            pupil_shape,
            pupil_color,
            background_color,
        } => {
            render_eye_shape(renderer, config, aabb, *eye_shape, *background_color);
            render_pupil(
                renderer,
                config,
                aabb,
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
    aabb: &AABB,
    shape: EyeShape,
    color: Color,
) {
    let options = config.without_line(color);

    match shape {
        EyeShape::Almond => renderer.render_pointed_oval_aabb(aabb, &options),
        EyeShape::Circle => renderer.render_circle_aabb(aabb, &options),
        EyeShape::Ellipse => renderer.render_ellipse_aabb(aabb, &options),
    }
}

fn render_pupil(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    eye_shape: EyeShape,
    pupil_shape: PupilShape,
    color: Color,
) {
    let options = config.without_line(color);
    let radius = aabb.size().width() / 2;
    let slit_width = config.eye.get_slit_width(radius);

    match pupil_shape {
        PupilShape::Circle => renderer.render_circle(
            &aabb.center(),
            config.eye.get_circle_radius(eye_shape, radius),
            &options,
        ),
        PupilShape::VerticalSlit => {
            let half_height = config.eye.get_half_height(eye_shape, radius);
            renderer.render_pointed_oval(&aabb.center(), slit_width, half_height, &options)
        }
        PupilShape::HorizontalSlit => {
            renderer.render_pointed_oval(&aabb.center(), radius, slit_width, &options)
        }
    }
}
