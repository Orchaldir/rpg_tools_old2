use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::pants::{Pants, PantsStyle};

enum BottomStyle {
    Balloon,
    BootsOverPants,
    Sharp,
}

pub fn render_pants(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    pants: &Pants,
    shaft_y: Option<f32>,
) {
    let options = config.get_options(pants.color);
    let polygon = match pants.style {
        PantsStyle::Balloon => get_balloon(config, aabb, body),
        PantsStyle::Bermuda => get_bermuda(config, aabb, body),
        PantsStyle::HotPants => get_hot_pants(config, aabb, body),
        PantsStyle::Regular => get_regular_pants(config, aabb, body, shaft_y),
        PantsStyle::Shorts => get_shorts(config, aabb, body),
    };

    renderer.render_rounded_polygon(&polygon, &options);
}

fn get_balloon(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    let bottom_y = config.pants.get_bottom_y(&config.body, body);
    get_pants(config, aabb, body, bottom_y, BottomStyle::Balloon)
}

fn get_bermuda(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_shorter_pants(config, aabb, body, config.pants.height_bermuda)
}

fn get_hot_pants(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_base(config, aabb, body).build()
}

fn get_regular_pants(
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shaft_y: Option<f32>,
) -> Polygon2d {
    let bottom_y = config.pants.get_bottom_y(&config.body, body);
    let shaft_y = shaft_y.unwrap_or(1.0);
    let (bottom_y, style) = if bottom_y > shaft_y {
        (shaft_y, BottomStyle::BootsOverPants)
    } else {
        (bottom_y, BottomStyle::Sharp)
    };

    get_pants(config, aabb, body, bottom_y, style)
}

fn get_shorts(config: &RenderConfig, aabb: &AABB, body: &Body) -> Polygon2d {
    get_shorter_pants(config, aabb, body, config.pants.height_shorts)
}

fn get_shorter_pants(config: &RenderConfig, aabb: &AABB, body: &Body, factor: f32) -> Polygon2d {
    let bottom_y = interpolate_pants_y(config, body, factor);
    get_pants(config, aabb, body, bottom_y, BottomStyle::Sharp)
}

pub fn interpolate_pants_y(config: &RenderConfig, body: &Body, factor: f32) -> f32 {
    let top_y = config.body.get_torso_bottom();
    let full_bottom_y = config.pants.get_bottom_y(&config.body, body);
    interpolate(top_y, full_bottom_y, factor)
}

fn get_pants(
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    bottom_y: f32,
    style: BottomStyle,
) -> Polygon2d {
    let mut builder = get_base(config, aabb, body);
    let (pants_width, inner_width) = config.pants.get_widths(&config.body, body);
    let top_y = config.body.get_torso_bottom();
    let mid_y = (top_y + bottom_y) * 0.5;

    builder.add_mirrored_points(aabb, pants_width, mid_y, false);

    match style {
        BottomStyle::Balloon => {
            let balloon_extra = pants_width * config.pants.balloon_padding;
            builder.add_mirrored_points(aabb, pants_width + balloon_extra, bottom_y, false);
            builder.add_mirrored_points(aabb, inner_width - balloon_extra, bottom_y, false);
        }
        BottomStyle::BootsOverPants => {
            let padding = config.body.get_legs_width(body) * config.pants.width_padding;
            builder.add_mirrored_points(aabb, pants_width - padding, bottom_y, true);
            builder.add_mirrored_points(aabb, inner_width + padding, bottom_y, true);
        }
        BottomStyle::Sharp => {
            builder.add_mirrored_points(aabb, pants_width, bottom_y, true);
            builder.add_mirrored_points(aabb, inner_width, bottom_y, true);
        }
    }

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

pub fn interpolate(start: f32, end: f32, factor: f32) -> f32 {
    start * (1.0 - factor) + end * factor
}
