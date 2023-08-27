use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::hair::bun::BunStyle;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::size::Size;

pub fn render_buns(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: BunStyle,
    size: Size,
    color: Color,
) {
    let options = config.get_options(color);
    let radius = aabb.calculate_from_height(match size {
        Size::Small => 0.15,
        Size::Medium => 0.2,
        Size::Large => 0.25,
    });

    match style {
        BunStyle::Low => {}
        BunStyle::High => {
            let center = aabb.get_point(0.5, 0.0);
            renderer.render_circle(&center, radius, &options);
        }
        BunStyle::Twin => {}
    }
}
