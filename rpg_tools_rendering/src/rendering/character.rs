use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
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
        Appearance::HeadOnly { head, .. } => {
            config.head_renderer.render(renderer, config, &inner, head)
        }
        Appearance::Humanoid { body, head, .. } => {
            config.body_renderer.render(renderer, config, &inner, body);
            let head_aabb = config.body_renderer.calculate_head_aabb(&inner);
            config
                .head_renderer
                .render(renderer, config, &head_aabb, head);
        }
    }
}
