use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::pants::render_pants;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod pants;

pub fn render_clothing(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
) {
    if let Clothing::Simple { pants } = &body.clothing {
        render_pants(renderer, config, aabb, body, pants)
    }
}
