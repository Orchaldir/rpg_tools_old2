use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::character::body::torso::create_torso;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::part::neckline::add_straight_neckline;
use crate::rendering::equipment::part::sleeve::render_sleeves;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::{Coat, Outerwear};

pub fn render_outerwear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    outerwear: &Outerwear,
    from_front: bool,
) {
    match outerwear {
        Outerwear::None => {}
        Outerwear::Coat { style } => render_coat(renderer, config, aabb, body, style, from_front),
    }
}

fn render_coat(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    coat: &Coat,
    from_front: bool,
) {
    render_sleeves(renderer, config, aabb, body, coat.sleeve, coat.color);
    render_torso(renderer, config, aabb, body, coat, from_front)
}

fn render_torso(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    coat: &Coat,
    from_front: bool,
) {
    let options = config.get_options(coat.color);
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let torso = config.body.get_torso_config(body.shape);
    let mut builder = create_torso(&torso_aabb, &config.body, torso);

    add_straight_neckline(&torso_aabb, torso, &mut builder);

    builder.reverse();

    let polygon = builder.build();
    renderer.render_rounded_polygon(&polygon, &options);
}
