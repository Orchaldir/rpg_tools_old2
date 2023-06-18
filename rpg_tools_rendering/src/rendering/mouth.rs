use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::Size;
use rpg_tools_core::model::color::Color;

pub fn render_mouth(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    let head_width_factor = config.head.get_mouth_width(head.shape);
    let head_width = aabb.size().height() as f32 * head_width_factor;

    match &head.mouth {
        Mouth::None => {}
        Mouth::Circle {
            size,
            teeth,
            teeth_color,
        } => {
            let center = aabb.get_point(0.5, config.head.y_mouth);
            let radius = get_circle_radius(head_width, *size);

            render_circular_mouth(renderer, config, &center, radius);
        }
        Mouth::Normal { .. } => {}
    }
}

fn get_circle_radius(head_width: f32, size: Size) -> u32 {
    (head_width
        * match size {
            Size::Low => 0.1,
            Size::Medium => 0.2,
            Size::High => 0.3,
        }) as u32
}

pub fn render_circular_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    center: &Point2d,
    radius: u32,
) {
    let options = config.without_line(Color::Black);
    renderer.render_circle(center, radius, &options);
}
