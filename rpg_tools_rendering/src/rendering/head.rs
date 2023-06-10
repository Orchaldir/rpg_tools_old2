use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::head::{GeometricHeadShape, Head, HeadShape};

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct HeadRenderer {}

impl HeadRenderer {
    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        head: &Head,
    ) {
        match head.shape {
            HeadShape::Geometric(geometric) => {
                self.render_geometric(renderer, config, aabb, head, geometric)
            }
            HeadShape::Realistic(realistic) => {}
        }
    }

    fn render_geometric(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        head: &Head,
        geometric: GeometricHeadShape,
    ) {
        let options = config.get_options(&head.skin);
        match geometric {
            GeometricHeadShape::Circle => {
                renderer.render_circle(&aabb.center(), aabb.inner_radius(), &options)
            }
            GeometricHeadShape::Square => renderer.render_rectangle(aabb, &options),
        }
    }
}
