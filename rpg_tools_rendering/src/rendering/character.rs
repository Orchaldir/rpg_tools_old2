use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::body::{calculate_head_aabb, render_body};
use crate::rendering::config::RenderConfig;
use crate::rendering::eye::render_eyes;
use crate::rendering::hair::render_hair;
use crate::rendering::head::render_head_shape;
use crate::rendering::mouth::render_mouth;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::length::Length;

pub fn calculate_character_size(config: &RenderConfig, appearance: &Appearance) -> Size2d {
    calculate_size(config, appearance.calculate_height())
}

pub fn calculate_size(config: &RenderConfig, height: Length) -> Size2d {
    Size2d::square(height.to_millimetre() + config.border * 2)
}

pub fn render_character(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    appearance: &Appearance,
) {
    let inner = aabb.shrink(config.border);

    match appearance {
        Appearance::HeadOnly { head, .. } => {
            render_head(renderer, config, head, &inner);
        }
        Appearance::Humanoid { body, head, .. } => {
            render_body(renderer, config, &inner, body);
            let head_aabb = calculate_head_aabb(&inner);
            render_head(renderer, config, head, &head_aabb);
        }
    }
}

fn render_head(renderer: &mut dyn Renderer, config: &RenderConfig, head: &Head, aabb: &AABB) {
    render_head_shape(renderer, config, aabb, head);
    render_eyes(renderer, config, aabb, head);
    render_mouth(renderer, config, aabb, head);
    render_hair(renderer, config, aabb, head);
}
