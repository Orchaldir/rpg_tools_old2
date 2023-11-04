use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::equipment::shirt::ShirtConfig;
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;

pub fn add_neckline(
    aabb: &AABB,
    config: &ShirtConfig,
    torso: &TorsoConfig,
    neckline: Neckline,
    builder: &mut Polygon2dBuilder,
) {
    match neckline {
        Neckline::Boat => add_round(aabb, torso, builder, config.boat_width, config.boat_depth),
        Neckline::Crew => add_round(aabb, torso, builder, config.crew_width, config.crew_depth),
        Neckline::DeepV => add_v(aabb, torso, builder, config.deep_v_depth),
        Neckline::None => add_straight_neckline(aabb, torso, builder),
        Neckline::Scoop => add_round(aabb, torso, builder, config.scoop_width, config.scoop_depth),
        Neckline::V => add_v(aabb, torso, builder, config.v_depth),
        Neckline::VeryDeppV => add_v(aabb, torso, builder, config.very_deep_v_depth),
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
    add_straight_neckline(aabb, torso, builder);
    builder.add_point(aabb.get_point(0.5, depth), true);
}

pub fn add_straight_neckline(aabb: &AABB, torso: &TorsoConfig, builder: &mut Polygon2dBuilder) {
    let width = torso.shoulder_width / 3.0;
    builder.add_mirrored_points(aabb, width, 0.0, true);
}

pub fn get_neckline_bottom_y(config: &ShirtConfig, neckline: Neckline) -> f32 {
    match neckline {
        Neckline::Boat => config.boat_depth,
        Neckline::Crew => config.crew_depth,
        Neckline::DeepV => config.deep_v_depth,
        Neckline::None => 0.0,
        Neckline::Scoop => config.scoop_depth,
        Neckline::V => config.v_depth,
        Neckline::VeryDeppV => config.very_deep_v_depth,
    }
}
