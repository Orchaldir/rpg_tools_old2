use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::head::{
    GeometricHeadShape, Head, HeadShape, RealisticHeadShape,
};

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
            HeadShape::Realistic(realistic) => {
                self.render_realistic(renderer, config, aabb, head, realistic)
            }
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

    fn render_realistic(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        head: &Head,
        realistic: RealisticHeadShape,
    ) {
        let options = config.get_options(&head.skin);

        let forehead = get_forehead(realistic);
        let forehead_width = get_forehead_width(realistic);
        let forehead_left = aabb.get_point(0.5 - forehead_width, forehead);
        let forehead_right = aabb.get_point(0.5 + forehead_width, forehead);

        let mouth = get_mouth(realistic);
        let mouth_width = get_mouth_width(realistic);
        let mouth_left = aabb.get_point(0.5 - mouth_width, mouth);
        let mouth_right = aabb.get_point(0.5 + mouth_width, mouth);

        let chin_width = get_chin_width(realistic);
        let chin_left = aabb.get_point(0.5 - chin_width, 1.0);
        let chin_right = aabb.get_point(0.5 + chin_width, 1.0);

        let top = aabb.get_point(0.5, 0.0);

        renderer.render_polygon(
            &format!(
                "M {} {} L {} {} L {} {} L {} {} L {} {} L {} {} L {} {} Z",
                forehead_left.x,
                forehead_left.y,
                mouth_left.x,
                mouth_left.y,
                chin_left.x,
                chin_left.y,
                chin_right.x,
                chin_right.y,
                mouth_right.x,
                mouth_right.y,
                forehead_right.x,
                forehead_right.y,
                top.x,
                top.y,
            ),
            &options,
        );
    }
}

fn get_forehead(realistic: RealisticHeadShape) -> f32 {
    0.25
}

fn get_mouth(realistic: RealisticHeadShape) -> f32 {
    0.75
}

fn get_forehead_width(realistic: RealisticHeadShape) -> f32 {
    0.5
}

fn get_mouth_width(realistic: RealisticHeadShape) -> f32 {
    0.5
}

fn get_chin_width(realistic: RealisticHeadShape) -> f32 {
    0.2
}
