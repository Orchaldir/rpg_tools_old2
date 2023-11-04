use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::outerwear::coat::render_coat;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::Outerwear;

pub mod coat;

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
        Outerwear::Cloak(cloak) => {}
        Outerwear::Coat { style } => render_coat(renderer, config, aabb, body, style, from_front),
    }
}
