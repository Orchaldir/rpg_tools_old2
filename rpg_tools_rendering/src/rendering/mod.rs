use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::appearance::color::Color;

pub mod character;
pub mod config;
pub mod equipment;

pub fn render_polygon(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    polygon: &Polygon2d,
    color: Color,
) {
    let options = config.get_options(color);
    renderer.render_rounded_polygon(polygon, &options);
}
