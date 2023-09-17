use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::pants::{Pants, PantsStyle};

pub fn render_pants(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    pants: &Pants,
) {
    let options = config.get_options(pants.color);
    let polygon = match pants.style {
        PantsStyle::Balloon => get_balloon(config, aabb, body),
        PantsStyle::Bermuda => get_bermuda(config, aabb, body),
        PantsStyle::HotPants => get_hot_pants(config, aabb, body),
        PantsStyle::Regular => get_regular_pants(config, aabb, body),
        PantsStyle::Shorts => get_shorts(config, aabb, body),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_balloon(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let (pants_width, _inner_width) = config.pants.get_widths(&config.body, body);
    let bottom_y = config.pants.get_bottom_y(&config.body, body);
    let balloon_extra = pants_width * config.pants.balloon_padding;
    get_pants(config, aabb, body, bottom_y, balloon_extra)
}

fn get_bermuda(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_shorter_pants(config, aabb, body, config.pants.height_bermuda)
}

fn get_hot_pants(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_base(config, aabb, body).build()
}

fn get_regular_pants(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let bottom_y = config.pants.get_bottom_y(&config.body, body);
    get_pants(config, aabb, body, bottom_y, 0.0)
}

fn get_shorts(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_shorter_pants(config, aabb, body, config.pants.height_shorts)
}

fn get_shorter_pants(config: &RenderConfig, aabb: &AABB, body: &Body, factor: f32) -> Polygon2d {
    let top_y = config.body.get_torso_bottom();
    let bottom_y = config.pants.get_bottom_y(&config.body, body);
    get_pants(
        config,
        aabb,
        body,
        interpolate(top_y, bottom_y, factor),
        0.0,
    )
}

fn get_pants(
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    bottom_y: f32,
    balloon_extra: f32,
) -> Polygon2d {
    let mut builder = get_base(config, aabb, body);
    let (pants_width, inner_width) = config.pants.get_widths(&config.body, body);
    let top_y = config.body.get_torso_bottom();
    let mid_y = (top_y + bottom_y) * 0.5;
    let is_sharp = balloon_extra <= 0.0;

    builder.add_mirrored_points(aabb, pants_width, mid_y, false);
    builder.add_mirrored_points(aabb, pants_width + balloon_extra, bottom_y, is_sharp);
    builder.add_mirrored_points(aabb, inner_width - balloon_extra, bottom_y, is_sharp);
    builder.add_mirrored_points(aabb, inner_width, mid_y, false);
    builder.add_point(aabb.get_point(0.5, top_y), false);

    builder.build()
}

fn get_base(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2dBuilder {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let hip_width = config.pants.get_hip_width(&config.body, body);
    let top_y = config.body.y_lower;
    let center_y = top_y + config.pants.offset_center;
    let mut builder = Polygon2dBuilder::new();

    // center curves downwards
    builder.add_point(torso_aabb.get_point(0.5, center_y), false);
    // rectangle forming the base of the pants
    builder.add_mirrored_points(&torso_aabb, hip_width, top_y, true);
    builder.add_mirrored_points(&torso_aabb, hip_width, 1.0, false);

    builder
}

fn interpolate(start: f32, end: f32, factor: f32) -> f32 {
    start * (1.0 - factor) + end * factor
}
