use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::body::{calculate_head_aabb, render_body};
use crate::rendering::config::RenderConfig;
use crate::rendering::eye::render_eyes;
use crate::rendering::head::render_head;
use rpg_tools_core::model::character::appearance::Appearance;

pub fn calculate_character_size(config: &RenderConfig, appearance: &Appearance) -> Size2d {
    let height = appearance.calculate_height();

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
        Appearance::HeadOnly { head, .. } => render_head(renderer, config, &inner, head),
        Appearance::Humanoid { body, head, .. } => {
            render_body(renderer, config, &inner, body);
            let head_aabb = calculate_head_aabb(&inner);
            render_head(renderer, config, &head_aabb, head);
            render_eyes(renderer, config, &head_aabb, head);
        }
    }
}
