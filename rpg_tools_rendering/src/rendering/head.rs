use crate::math::aabb2d::AABB;
use crate::renderer::{RenderOptions, Renderer};
use rpg_tools_core::model::character::appearance::head::{GeometricHeadShape, Head, HeadShape};
use rpg_tools_core::model::color::Color;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct HeadRenderer {}

impl HeadRenderer {
    pub fn render(&self, renderer: &mut dyn Renderer, aabb: &AABB, head: &Head) {
        match head.shape {
            HeadShape::Geometric(geometric) => {
                self.render_geometric(renderer, aabb, head, geometric)
            }
            HeadShape::Realistic(realistic) => {}
        }
    }

    fn render_geometric(
        &self,
        renderer: &mut dyn Renderer,
        aabb: &AABB,
        head: &Head,
        geometric: GeometricHeadShape,
    ) {
        let options = RenderOptions::new(Color::Red, Color::Blue, 20);
        match geometric {
            GeometricHeadShape::Circle => {
                renderer.render_circle(&aabb.center(), aabb.inner_radius(), &options)
            }
            GeometricHeadShape::Square => renderer.render_rectangle(aabb, &options),
        }
    }
}