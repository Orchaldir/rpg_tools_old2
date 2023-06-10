use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct BodyRenderer {}

impl BodyRenderer {
    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        body: &Body,
    ) {
    }
}
