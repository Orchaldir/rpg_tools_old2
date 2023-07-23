use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};

pub fn render_head_shape(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
) {
    let options = config.get_skin_options(&head.skin);

    render_head_shape_with_option(renderer, config, aabb, options, head.shape);
}

pub fn render_head_shape_with_option(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    options: RenderOptions,
    shape: HeadShape,
) {
    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(shape), 0.0);
    let (forehead_left, forehead_right) = aabb.get_mirrored_points(
        config.head.get_forehead_width(shape),
        config.head.y_forehead,
    );
    let (mouth_left, mouth_right) =
        aabb.get_mirrored_points(config.head.get_mouth_width(shape), config.head.y_mouth);
    let (chin_left, chin_right) = aabb.get_mirrored_points(config.head.get_chin_width(shape), 1.0);

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
