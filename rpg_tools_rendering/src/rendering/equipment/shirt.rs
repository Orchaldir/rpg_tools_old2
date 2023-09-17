use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::body::torso::create_torso;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::shirt::Shirt;

pub fn render_shirt(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    shirt: &Shirt,
) {
    let options = config.get_options(shirt.color);
    let torso_aabb = config.body.get_torso_aabb(body, aabb);
    let polygon = create_torso(
        &torso_aabb,
        &config.body,
        config.body.get_torso_config(body.shape),
    )
    .build();
    renderer.render_rounded_polygon(&polygon, &options);
}
