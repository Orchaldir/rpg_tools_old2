use crate::math::aabb2d::AABB;
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
    let options = config.get_skin_options(&head.skin);
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
    let options = config.get_skin_options(&head.skin);

    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) = aabb.get_mirrored_points(
        config.head.get_forehead_width(realistic),
        config.head.y_forehead,
    );
    let (mouth_left, mouth_right) = aabb.get_mirrored_points(
        config.head.get_mouth_width_realistic(realistic),
        config.head.y_mouth,
    );
    let (chin_left, chin_right) =
        aabb.get_mirrored_points(config.head.get_chin_width(realistic), 1.0);

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
    let cut = config.cut_corners(&polygon).unwrap();

    renderer.render_polygon(&cut, &options);
}
