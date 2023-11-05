use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::outerwear::cloak::render_cloak_behind_body;
use crate::rendering::equipment::outerwear::coat::render_coat;
use crate::rendering::equipment::pants::interpolate_pants_y;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::{Outerwear, OuterwearLength};

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
    if let Outerwear::Coat(coat) = outerwear {
        render_coat(renderer, config, aabb, body, coat, from_front);
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
        if from_front {
            render_cloak_behind_body(renderer, config, aabb, body, cloak, from_front);
        }
    }
}

pub fn render_outerwear_over_hands(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    outerwear: &Outerwear,
    from_front: bool,
) {
    if let Outerwear::Cloak(cloak) = outerwear {
        if !from_front {
            render_cloak_behind_body(renderer, config, aabb, body, cloak, from_front)
        }
    }
}

pub fn get_bottom_y(config: &RenderConfig, body: &Body, length: OuterwearLength) -> f32 {
    let y_factor = match length {
        OuterwearLength::Hip => config.pants.height_shorts,
        OuterwearLength::Knee => config.pants.height_bermuda,
        OuterwearLength::Ankle => config.pants.get_bottom_y(&config.body, body),
    };
    interpolate_pants_y(config, body, y_factor)
}
