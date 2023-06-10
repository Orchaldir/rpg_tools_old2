use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
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

        let top_width = get_top_width(realistic);
        let top_left = aabb.get_point(0.5 - top_width, 0.0);
        let top_right = aabb.get_point(0.5 + top_width, 0.0);

        let forehead = 0.25;
        let forehead_width = get_forehead_width(realistic);
        let forehead_left = aabb.get_point(0.5 - forehead_width, forehead);
        let forehead_right = aabb.get_point(0.5 + forehead_width, forehead);

        let mouth = 0.75;
        let mouth_width = get_mouth_width(realistic);
        let mouth_left = aabb.get_point(0.5 - mouth_width, mouth);
        let mouth_right = aabb.get_point(0.5 + mouth_width, mouth);

        let chin_width = get_chin_width(realistic);
        let chin_left = aabb.get_point(0.5 - chin_width, 1.0);
        let chin_right = aabb.get_point(0.5 + chin_width, 1.0);

        let polygon = Polygon2d::new(vec![
            top_left,
            forehead_left,
            mouth_left,
            chin_left,
            chin_right,
            mouth_right,
            forehead_right,
            top_right,
        ]);
        let cut = polygon.cut_corners_n(0.25, 0.25, 3).unwrap();

        renderer.render_polygon(&cut, &options);
    }
}

const WIDE: f32 = 0.45;
const NARROW: f32 = 0.33;

fn get_top_width(realistic: RealisticHeadShape) -> f32 {
    match realistic {
        Rectangle | Square => 0.3,
        _ => 0.2,
    }
}

fn get_forehead_width(realistic: RealisticHeadShape) -> f32 {
    match realistic {
        Round | Square | TriangleDown => WIDE,
        _ => NARROW,
    }
}

fn get_mouth_width(realistic: RealisticHeadShape) -> f32 {
    match realistic {
        Round | Square | TriangleUp => WIDE,
        _ => NARROW,
    }
}

fn get_chin_width(realistic: RealisticHeadShape) -> f32 {
    match realistic {
        Rectangle | Square | TriangleUp => 0.3,
        Round => 0.2,
        _ => 0.1,
    }
}
