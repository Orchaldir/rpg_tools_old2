use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::side::Side::{Left, Right};

pub fn render_eyebrow(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    center: &Point2d,
    radius: u32,
) {
    match eyebrow {
        EyeBrows::Normal { shape, style } | EyeBrows::Unibrow { shape, style } => {
            render_normal_eyebrow(renderer, config, *shape, *style, center, radius, None);
        }
        _ => {}
    }
}

pub fn render_eyebrows(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    match eyebrow {
        EyeBrows::Normal { shape, style } => {
            render_normal_eyebrow(renderer, config, *shape, *style, left, radius, Some(Left));
            render_normal_eyebrow(renderer, config, *shape, *style, right, radius, Some(Right));
        }
        EyeBrows::Unibrow { shape, style } => {}
        _ => {}
    }
}

pub fn render_normal_eyebrow(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    shape: EyebrowShape,
    style: EyebrowStyle,
    center: &Point2d,
    radius: u32,
    side: Option<Side>,
) {
}
