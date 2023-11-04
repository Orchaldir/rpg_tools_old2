use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::outerwear::cloak::{
    render_cloak_before_body, render_cloak_behind_body,
};
use crate::rendering::equipment::outerwear::coat::render_coat;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::Outerwear;

pub mod cloak;
pub mod coat;

pub fn render_outerwear_before_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    outerwear: &Outerwear,
    from_front: bool,
) {
    match outerwear {
        Outerwear::None => {}
        Outerwear::Cloak(cloak) => {
            render_cloak_before_body(renderer, config, aabb, body, cloak, from_front)
        }
        Outerwear::Coat { style } => render_coat(renderer, config, aabb, body, style, from_front),
    }
}

pub fn render_outerwear_behind_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    outerwear: &Outerwear,
    from_front: bool,
) {
    if let Outerwear::Cloak(cloak) = outerwear {
        render_cloak_behind_body(renderer, config, aabb, body, cloak, from_front);
    }
}
