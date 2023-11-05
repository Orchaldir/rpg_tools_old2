use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::equipment::appearance::option::button::ButtonColumn;
use rpg_tools_core::model::size::Size;

pub fn render_buttons(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    buttons: ButtonColumn,
    top_y: f32,
    bottom_y: f32,
    x: f32,
) {
    let option = config.without_line(buttons.button.color);
    let distance = bottom_y - top_y;
    let step = distance / buttons.count as f32;
    let mut y = top_y + step / 2.0;
    let radius = aabb.convert_to_height(match buttons.button.size {
        Size::Small => 0.01,
        Size::Medium => 0.015,
        Size::Large => 0.02,
    });

    for _i in 0..buttons.count {
        let center = aabb.get_point(x, y);
        renderer.render_circle(&center, radius, &option);
        y += step;
    }
}
