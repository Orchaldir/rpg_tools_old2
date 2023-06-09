use crate::math::aabb2d::AABB;
use rpg_tools_core::model::color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RenderOptions {
    fill_color: Color,
    line_color: Color,
    line_width: u32,
}

pub trait Renderer {
    /// Renders an axis aligned rectangle.
    fn render_rectangle(&mut self, aabb: &AABB, options: &RenderOptions);
}
