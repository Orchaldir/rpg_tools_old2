use crate::math::aabb2d::{get_end_x, AABB};
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::appearance::size::Size;
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::head::Head;

pub fn render_ears(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let options = config.get_skin_options(&head.skin);

    match &head.ears {
        Ears::None => {}
        Ears::Normal { shape, size } => match shape {
            EarShape::Round => render_round_ears(renderer, config, &options, aabb, head, *size),
            _ => render_normal_ears(renderer, config, &options, aabb, head, *shape, *size),
        },
    }
}

pub fn render_round_ears(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    head: &Head,
    size: Size,
) {
    let width_eyes = config.head.get_eye_width(head.shape);
    let radius = config.ear.get_round_ear_radius(aabb, size);
    let (left, right) = aabb.get_mirrored_points(width_eyes, config.head.y_eye);

    renderer.render_circle(&left, radius, options);
    renderer.render_circle(&right, radius, options);
}

pub fn render_normal_ears(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    head: &Head,
    shape: EarShape,
    size: Size,
) {
    let width_eyes = config.head.get_eye_width(head.shape);
    let width_mouth = config.head.get_mouth_width(head.shape);

    let left_ear = get_normal_ear_left(config, aabb, shape, size, width_eyes, width_mouth);

    renderer.render_rounded_polygon(&left_ear, options);
    renderer.render_rounded_polygon(&aabb.mirrored(&left_ear), options);
}

fn get_normal_ear_left(
    config: &RenderConfig,
    aabb: &AABB,
    shape: EarShape,
    size: Size,
    eye_width: f32,
    mouth_width: f32,
) -> Polygon2d {
    let offset = config.ear.normal_offset;
    let width = config.ear.normal_width;
    let top_inner_x = get_end_x(eye_width) - offset;
    let top_outer_x = top_inner_x + width + config.ear.normal_top_x;
    let bottom_inner_x = get_end_x(mouth_width) - offset;
    let bottom_outer_x = bottom_inner_x + width;

    let top_inner = aabb.get_point(top_inner_x, config.head.y_eye);
    let top_outer = aabb.get_point(top_outer_x, config.head.y_eye);
    let bottom_inner = aabb.get_point(bottom_inner_x, config.head.y_mouth);
    let bottom_outer = aabb.get_point(bottom_outer_x, config.head.y_mouth);

    let mut corners = vec![
        top_inner,
        bottom_inner,
        bottom_inner,
        bottom_outer,
        top_outer,
    ];

    if EarShape::Pointed == shape {
        let length = config.ear.get_pointed_ear_length(size);
        let point = aabb.get_point(top_outer_x, config.head.y_eye - length);

        corners.push(point);
    }

    Polygon2d::new(corners)
}
