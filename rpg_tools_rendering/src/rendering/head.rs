use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::{
    GeometricHeadShape, Head, HeadShape, RealisticHeadShape,
};

pub fn render_head(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    match head.shape {
        HeadShape::Geometric(geometric) => {
            render_geometric(renderer, config, aabb, head, geometric)
        }
        HeadShape::Realistic(realistic) => {
            render_realistic(renderer, config, aabb, head, realistic)
        }
    }
}

fn render_geometric(
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
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
    realistic: RealisticHeadShape,
) {
    let options = config.get_options(&head.skin);

    let (top_left, top_right) =
        get_mirrored_points(aabb, config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) =
        get_mirrored_points(aabb, config.head.get_forehead_width(realistic), 0.25);
    let (mouth_left, mouth_right) =
        get_mirrored_points(aabb, config.head.get_mouth_width(realistic), 0.75);
    let (chin_left, chin_right) =
        get_mirrored_points(aabb, config.head.get_chin_width(realistic), 1.0);

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

fn get_mirrored_points(aabb: &AABB, width: f32, vertical: f32) -> (Point2d, Point2d) {
    let half = width / 2.0;
    (
        aabb.get_point(0.5 - half, vertical),
        aabb.get_point(0.5 + half, vertical),
    )
}
