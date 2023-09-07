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

    match style {
        BunStyle::Low => {
            let width = config.head.get_mouth_width(head_shape);
            let (radius, width_factor) = calculate(aabb, size, width);
            let center = aabb.get_point(0.5, 1.0 - width_factor);
            renderer.render_circle(&center, radius, &options);
        }
        BunStyle::High => {
            let width = (config.head.get_top_width(head_shape)
                + config.head.get_forehead_width(head_shape))
                / 2.0;
            let (radius, width_factor) = calculate(aabb, size, width);
            let center = aabb.get_point(0.5, width_factor / 2.0);
            renderer.render_circle(&center, radius, &options);
        }
        BunStyle::Twin => {
            let width = (config.head.get_top_width(head_shape)
                + config.head.get_forehead_width(head_shape))
                / 2.0;
            let (radius, _width_factor) = calculate(aabb, size, width);
            let radius = (0.8 * radius as f32) as u32;
            let (left, right) = aabb.get_mirrored_points(width, 0.1);
            renderer.render_circle(&left, radius, &options);
            renderer.render_circle(&right, radius, &options);
        }
    }
}

fn calculate(aabb: &AABB, size: Size, width: f32) -> (u32, f32) {
    let radius_factor = match size {
        Size::Small => 0.25,
        Size::Medium => 0.3,
        Size::Large => 0.35,
    };
    let width_factor = width * radius_factor;
    let radius = aabb.convert_to_width(width_factor);

    (radius, width_factor)
}
