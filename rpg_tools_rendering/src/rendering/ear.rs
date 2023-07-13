use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::side::Side;

pub fn render_ears(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let options = config.get_skin_options(&head.skin);

    match &head.ears {
        Ears::None => {}
        Ears::Normal { size, width } => {
            let width_eyes = config.head.get_eye_width(head.shape);
            let width_mouth = config.head.get_mouth_width(head.shape);

            render_normal_ear(
                renderer,
                config,
                &options,
                aabb,
                Side::Left,
                width_eyes,
                width_mouth,
            );

            render_normal_ear(
                renderer,
                config,
                &options,
                aabb,
                Side::Right,
                width_eyes,
                width_mouth,
            );
        }
    }
}

pub fn render_normal_ear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    options: &RenderOptions,
    aabb: &AABB,
    side: Side,
    head_eye: f32,
    head_mouth: f32,
) {
    let offset = 0.02;
    let half_eye = head_eye / 2.0 - offset;
    let half_mouth = head_mouth / 2.0 - offset;
    let width = 0.08;
    let sign = side.get_sign();
    let top_inner_x = half_eye * sign;
    let top_outer_x = (half_eye + width) * sign;
    let bottom_inner_x = half_mouth * sign;
    let bottom_outer_x = (half_mouth + width) * sign;

    let top_inner = aabb.get_point(0.5 + top_inner_x, config.head.y_eye);
    let top_outer = aabb.get_point(0.5 + top_outer_x, config.head.y_eye);
    let bottom_inner = aabb.get_point(0.5 + bottom_inner_x, config.head.y_mouth);
    let bottom_outer = aabb.get_point(0.5 + bottom_outer_x, config.head.y_mouth);

    let polygon = Polygon2d::new(vec![
        top_inner,
        bottom_inner,
        bottom_inner,
        bottom_outer,
        top_outer,
    ]);
    let cut = config.cut_corners(&polygon).unwrap();

    renderer.render_polygon(&cut, options);
}
