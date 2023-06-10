use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::renderer::color::WebColor;

pub mod color;
pub mod svg;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderOptions {
    fill_color: WebColor,
    line_color: WebColor,
    line_width: u32,
}

impl RenderOptions {
    pub const fn new(fill_color: WebColor, line_color: WebColor, line_width: u32) -> Self {
        Self {
            fill_color,
            line_color,
            line_width,
        }
    }
}

pub trait Renderer {
    /// Renders a a circle.
    fn render_circle(&mut self, center: &Point2d, radius: u32, options: &RenderOptions);

    /// Renders an axis aligned rectangle.
    fn render_rectangle(&mut self, aabb: &AABB, options: &RenderOptions);
}
