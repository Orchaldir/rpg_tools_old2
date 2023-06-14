use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::{
    Eye, EyeDistance, EyeShape, Eyes, PupilShape,
};
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

    match shape {
        EyeShape::Almond => {
            renderer.render_pointed_oval(center, radius, get_radius_y(radius), &options)
        }
        EyeShape::Circle => renderer.render_circle(center, radius, &options),
        EyeShape::Ellipse => {
            renderer.render_ellipse(center, radius, get_radius_y(radius), &options)
        }
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

    match pupil_shape {
        PupilShape::Circle => {
            renderer.render_circle(center, get_pupil_radius(eye_shape, radius), &options)
        }
        PupilShape::VerticalSlit => {
            let radius_y = get_radius_y(radius);
            renderer.render_pointed_oval(center, get_slit_radius(radius_y), radius_y, &options)
        }
        PupilShape::HorizontalSlit => {
            renderer.render_pointed_oval(center, radius, get_slit_radius(radius), &options)
        }
    }
}

fn get_radius_y(radius_x: u32) -> u32 {
    (radius_x as f32 * 0.75) as u32
}

fn get_pupil_radius(eye_shape: EyeShape, radius: u32) -> u32 {
    (match eye_shape {
        EyeShape::Circle => radius,
        _ => get_radius_y(radius),
    } as f32
        * 0.5) as u32
}

fn get_slit_radius(radius: u32) -> u32 {
    (radius as f32 * 0.2) as u32
}
