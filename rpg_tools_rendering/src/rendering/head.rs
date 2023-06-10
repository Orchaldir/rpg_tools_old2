use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use rpg_tools_core::model::character::appearance::head::Head;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct HeadRenderer {
    border: u32,
}

impl HeadRenderer {
    pub fn render(&self, renderer: &mut dyn Renderer, aabb: &AABB, head: &Head) {}
}
