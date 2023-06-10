use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::body::BodyRenderer;
use crate::rendering::head::HeadRenderer;
use rpg_tools_core::model::character::appearance::Appearance;

/// Renders a [`character`](rpg_tools_core::model::character::Character).
#[derive(Debug, PartialEq, Eq)]
pub struct CharacterRenderer {
    border: u32,
    body_renderer: BodyRenderer,
    head_renderer: HeadRenderer,
}

impl CharacterRenderer {
    pub fn calculate_size(&self, appearance: &Appearance) -> Size2d {
        let height = appearance.calculate_height();

        Size2d::square(height.to_millimetre() + self.border * 2)
    }

    pub fn render(&self, renderer: &mut dyn Renderer, aabb: &AABB, appearance: &Appearance) {
        let inner = aabb.shrink(self.border);

        match appearance {
            Appearance::HeadOnly { head, .. } => self.head_renderer.render(renderer, &inner, head),
            Appearance::Humanoid { body, .. } => self.body_renderer.render(renderer, &inner, body),
        }
    }
}
