use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::character::body::{get_left_arm, get_left_arm_short};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::option::sleeve::SleeveStyle;

pub fn render_sleeves(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    sleeve_style: SleeveStyle,
    sleeve_color: Color,
) {
    let options = config.get_options(sleeve_color);

    let polygon = match sleeve_style {
        SleeveStyle::Long => get_left_arm(config, aabb, body),
        SleeveStyle::None => return,
        SleeveStyle::Short => get_left_arm_short(config, aabb, body, true),
    }
    .build();

    renderer.render_rounded_polygon(&polygon, &options);
    renderer.render_rounded_polygon(&aabb.mirrored(&polygon), &options);
}
