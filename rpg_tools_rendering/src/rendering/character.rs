use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::Appearance;

/// Renders a [`character`](rpg_tools_core::model::character::Character).
#[derive(Debug, PartialEq, Eq)]
pub struct CharacterRenderer {
    pub border: u32,
}

impl CharacterRenderer {
    pub fn calculate_size(&self, appearance: &Appearance) -> Size2d {
        let height = appearance.calculate_height();

        Size2d::square(height.to_millimetre() + self.border * 2)
    }

    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        appearance: &Appearance,
    ) {
        let inner = aabb.shrink(self.border);

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
}
