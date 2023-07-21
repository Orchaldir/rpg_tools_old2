use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::ear::{EarShape, Ears};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::side::Side;

pub fn render_ears(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let options = config.get_skin_options(&head.skin);

    match &head.ears {
        Ears::None => {}
        Ears::Normal { shape } => match shape {
            EarShape::Round => render_round_ears(renderer, config, &options, aabb, head),
            _ => render_normal_ears(renderer, config, &options, aabb, head, *shape),
        },
    }
}

pub fn render_round_ears(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    head: &Head,
) {
    let width_eyes = config.head.get_eye_width(head.shape);

    render_round_ear(renderer, config, options, aabb, Side::Left, width_eyes);

    render_round_ear(renderer, config, options, aabb, Side::Right, width_eyes);
}

pub fn render_normal_ears(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    head: &Head,
    shape: EarShape,
) {
    let width_eyes = config.head.get_eye_width(head.shape);
    let width_mouth = config.head.get_mouth_width(head.shape);

    render_normal_ear(
        renderer,
        config,
        options,
        aabb,
        shape,
        Side::Left,
        width_eyes,
        width_mouth,
    );

    render_normal_ear(
        renderer,
        config,
        options,
        aabb,
        shape,
        Side::Right,
        width_eyes,
        width_mouth,
    );
}

pub fn render_round_ear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    side: Side,
    head_width: f32,
) {
    let half = head_width / 2.0;
    let radius = (aabb.size().height() as f32 / 10.0) as u32;
    let sign = side.get_sign();

    let center = aabb.get_point(0.5 + half * sign, config.head.y_eye);

    renderer.render_circle(&center, radius, options);
}

pub fn render_normal_ear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    shape: EarShape,
    side: Side,
    eye_width: f32,
    mouth_width: f32,
) {
    let offset = 0.02;
    let half_eye = eye_width / 2.0 - offset;
    let half_mouth = mouth_width / 2.0 - offset;
    let width = 0.08;
    let sign = side.get_sign();
    let top_inner_x = half_eye * sign;
    let top_outer_x = (half_eye + width + 0.02) * sign;
    let bottom_inner_x = half_mouth * sign;
    let bottom_outer_x = (half_mouth + width) * sign;

    let top_inner = aabb.get_point(0.5 + top_inner_x, config.head.y_eye);
    let top_outer = aabb.get_point(0.5 + top_outer_x, config.head.y_eye);
    let bottom_inner = aabb.get_point(0.5 + bottom_inner_x, config.head.y_mouth);
    let bottom_outer = aabb.get_point(0.5 + bottom_outer_x, config.head.y_mouth);

    let mut corners = vec![
        top_inner,
        bottom_inner,
        bottom_inner,
        bottom_outer,
        top_outer,
    ];

    if let EarShape::Pointed(size) = shape {
        let length = config.ear.get_pointed_ear_length(size);
        let point = aabb.get_point(0.5 + top_outer_x, config.head.y_eye - length);

        corners.push(point);
    }

    let polygon = Polygon2d::new(corners);
    let cut = config.cut_corners(&polygon).unwrap();

    renderer.render_polygon(&cut, options);
}
