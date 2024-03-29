use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::Renderer;
use crate::rendering::character::body::torso::create_torso;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::part::neckline::{add_neckline, add_straight_neckline};
use crate::rendering::equipment::part::sleeve::render_sleeves;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;
use rpg_tools_core::model::equipment::appearance::shirt::Shirt;

pub fn render_shirt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
    from_front: bool,
) {
    render_sleeves(
        renderer,
        config,
        aabb,
        body,
        shirt.sleeve_style,
        shirt.color,
    );
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
    let builder = create_shirt(config, aabb, body, shirt.neckline, from_front, 0.0);
    let polygon = builder.build();
    renderer.render_rounded_polygon(&polygon, &options);
}

pub fn create_shirt(
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    neckline: Neckline,
    from_front: bool,
    hip_padding: f32,
) -> Polygon2dBuilder {
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);
    let mut builder = create_torso(&torso_aabb, &config.body, torso, hip_padding);

    if from_front {
        add_neckline(&torso_aabb, &config.shirt, torso, neckline, &mut builder);
    } else {
        add_straight_neckline(&torso_aabb, torso, &mut builder)
    }

    builder
}
