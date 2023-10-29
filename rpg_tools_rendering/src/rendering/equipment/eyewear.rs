use crate::math::point2d::Point2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::equipment::appearance::eyewear::{Eyewear, LensStyle};

pub fn render_eyewear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyewear: &Eyewear,
    left: &Point2d,
    right: &Point2d,
    head_width: f32,
) {
    match eyewear {
        Eyewear::None => {}
        Eyewear::Glasses { style } => {
            render_glasses(renderer, config, style, left, right, head_width)
        }
        Eyewear::Monocle { .. } => {}
    }
}

fn render_glasses(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    style: &LensStyle,
    left: &Point2d,
    right: &Point2d,
    head_width: f32,
) {
}
