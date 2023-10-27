use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::belt::{Belt, Buckle, BuckleStyle};

pub fn render_belt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    belt: &Belt,
    from_front: bool,
) {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    render_band(renderer, config, &torso_aabb, body, belt);

    if from_front {
        render_buckle(renderer, config, &torso_aabb, &belt.buckle);
    }
}

fn render_band(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    belt: &Belt,
) {
    let options = config.get_options(belt.color);
    let polygon = get_band(config, aabb, body);

    renderer.render_rounded_polygon(&polygon, &options);
}

fn render_buckle(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, buckle: &Buckle) {
    let options = config.get_options(buckle.color);
    let box_aabb = config
        .belt
        .get_buckle_aabb(aabb, &config.body, &config.pants);

    match buckle.style {
        BuckleStyle::Box => renderer.render_rectangle(&box_aabb, &options),
        BuckleStyle::Frame => {
            let box_polygon = Polygon2d::new(box_aabb.corners());
            let frame_border = box_aabb.convert_to_width(0.1);
            let hole_polygon = Polygon2d::new(box_aabb.shrink(frame_border).corners());
            renderer.render_polygon_with_hole(&box_polygon, &hole_polygon, &options)
        }
        BuckleStyle::Plate => renderer.render_ellipse_aabb(&box_aabb, &options),
    }
}

fn get_band(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let hip_width = config.belt.get_width(&config.body, &config.pants, body);
    let (top_y, top_center_y, bottom_y, bottom_center_y) =
        config.belt.get_y_values(&config.body, &config.pants);
    let mut builder = Polygon2dBuilder::new();

    // center curves downwards
    builder.add_point(aabb.get_point(0.5, top_center_y), false);
    builder.add_mirrored_points(aabb, hip_width, top_y, true);
    builder.add_mirrored_points(aabb, hip_width, bottom_y, true);
    builder.add_point(aabb.get_point(0.5, bottom_center_y), false);

    builder.build()
}
