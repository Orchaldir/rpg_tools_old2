use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::Renderer;
use crate::rendering::character::body::torso::create_torso;
use crate::rendering::character::body::{get_left_arm, get_left_arm_short};
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::equipment::shirt::ShirtConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;
use rpg_tools_core::model::equipment::appearance::option::sleeve::SleeveStyle;
use rpg_tools_core::model::equipment::appearance::shirt::Shirt;

pub fn render_shirt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
    from_front: bool,
) {
    render_sleeves(renderer, config, aabb, body, shirt);
    render_torso(renderer, config, aabb, body, shirt, from_front);
}

fn render_torso(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
    from_front: bool,
) {
    let options = config.get_options(shirt.color);
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);
    let mut builder = create_torso(&torso_aabb, &config.body, torso);

    if from_front {
        add_neckline(&torso_aabb, &config.shirt, torso, shirt, &mut builder);
    } else {
        add_straight(&torso_aabb, torso, &mut builder)
    }

    let polygon = builder.build();
    renderer.render_rounded_polygon(&polygon, &options);
}

fn render_sleeves(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
) {
    let options = config.get_options(shirt.color);

    let polygon = match shirt.sleeve_style {
        SleeveStyle::Long => get_left_arm(config, aabb, body),
        SleeveStyle::None => return,
        SleeveStyle::Short => get_left_arm_short(config, aabb, body, true),
    }
    .build();

    renderer.render_rounded_polygon(&polygon, &options);
    renderer.render_rounded_polygon(&aabb.mirrored(&polygon), &options);
}

fn add_neckline(
    aabb: &AABB,
    config: &ShirtConfig,
    torso: &TorsoConfig,
    shirt: &Shirt,
    builder: &mut Polygon2dBuilder,
) {
    match shirt.neckline {
        Neckline::Boat => add_round(aabb, torso, builder, config.boat_width, config.boat_depth),
        Neckline::Crew => add_round(aabb, torso, builder, config.crew_width, config.crew_depth),
        Neckline::DeepV => add_v(aabb, torso, builder, config.deep_v_depth),
        Neckline::None => add_straight(aabb, torso, builder),
        Neckline::Scoop => add_round(aabb, torso, builder, config.scoop_width, config.scoop_depth),
        Neckline::V => add_v(aabb, torso, builder, config.v_depth),
    }
}

fn add_round(
    aabb: &AABB,
    torso: &TorsoConfig,
    builder: &mut Polygon2dBuilder,
    width: f32,
    depth: f32,
) {
    let width = torso.shoulder_width * width;
    builder.add_mirrored_points(aabb, width, 0.0, true);
    builder.add_mirrored_points(aabb, width * 0.7, depth, false);
}

fn add_v(aabb: &AABB, torso: &TorsoConfig, builder: &mut Polygon2dBuilder, depth: f32) {
    add_straight(aabb, torso, builder);
    builder.add_point(aabb.get_point(0.5, depth), true);
}

fn add_straight(aabb: &AABB, torso: &TorsoConfig, builder: &mut Polygon2dBuilder) {
    let width = torso.shoulder_width / 3.0;
    builder.add_mirrored_points(aabb, width, 0.0, true);
}
