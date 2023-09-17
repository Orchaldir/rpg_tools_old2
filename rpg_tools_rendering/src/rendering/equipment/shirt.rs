use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::Renderer;
use crate::rendering::body::torso::create_torso;
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::shirt::{Neckline, Shirt};

pub fn render_shirt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
) {
    let options = config.get_options(shirt.color);
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);
    let mut builder = create_torso(&torso_aabb, &config.body, torso);
    add_neckline(config, &torso_aabb, torso, shirt, &mut builder);
    let polygon = builder.build();
    renderer.render_rounded_polygon(&polygon, &options);
}

fn add_neckline(
    config: &RenderConfig,
    aabb: &AABB,
    torso: &TorsoConfig,
    shirt: &Shirt,
    builder: &mut Polygon2dBuilder,
) {
    match shirt.neckline {
        Neckline::Boat => {}
        Neckline::Crew => {
            let width = torso.shoulder_width / 3.0;
            builder.add_mirrored_points(&aabb, width, 0.0, true);
            builder.add_mirrored_points(&aabb, width, 0.1, false);
        }
        Neckline::DeepV => {}
        Neckline::Scoop => {}
        Neckline::V => {
            let width = torso.shoulder_width / 3.0;
            builder.add_mirrored_points(&aabb, width, 0.0, true);
            builder.add_point(aabb.get_point(0.5, 0.2), true);
        }
    }
}
