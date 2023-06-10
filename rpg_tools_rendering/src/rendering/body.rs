use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use rpg_tools_core::model::character::appearance::body::Body;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct BodyRenderer {
    border: u32,
}

impl BodyRenderer {
    pub fn render(&self, renderer: &mut dyn Renderer, aabb: &AABB, body: &Body) {}
}
