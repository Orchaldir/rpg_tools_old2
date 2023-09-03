use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::color::WebColor;

pub mod color;
pub mod svg;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderOptions {
    fill_color: Option<WebColor>,
    line_color: WebColor,
    line_width: u32,
}

impl RenderOptions {
    pub const fn new(fill_color: WebColor, line_color: WebColor, line_width: u32) -> Self {
        Self {
            fill_color: Some(fill_color),
            line_color,
            line_width,
        }
    }

    pub fn line(color: WebColor, width: u32) -> Self {
        Self {
            fill_color: None,
            line_color: color,
            line_width: width,
        }
    }

    pub fn no_line(color: WebColor) -> Self {
        Self {
            fill_color: Some(color.clone()),
            line_color: color,
            line_width: 0,
        }
    }
}

pub trait Renderer {
    /// Renders a circle.
    fn render_circle(&mut self, center: &Point2d, radius: u32, options: &RenderOptions);

    /// Renders a circle in an [`axis aligned bounding box`](AABB).
    fn render_circle_aabb(&mut self, aabb: &AABB, options: &RenderOptions) {
        self.render_circle(&aabb.center(), aabb.inner_radius(), options)
    }

    /// Renders a circle arc.
    fn render_circle_arc(
        &mut self,
        center: &Point2d,
        radius: u32,
        offset: Orientation,
        angle: Orientation,
        options: &RenderOptions,
    );

    /// Renders an ellipse.
    fn render_ellipse(
        &mut self,
        center: &Point2d,
        radius_x: u32,
        radius_y: u32,
        options: &RenderOptions,
    );

    /// Renders an ellipse in an [`axis aligned bounding box`](AABB).
    fn render_ellipse_aabb(&mut self, aabb: &AABB, options: &RenderOptions) {
        self.render_ellipse(
            &aabb.center(),
            aabb.size().width() / 2,
            aabb.size().height() / 2,
            options,
        )
    }

    /// Renders a line.
    fn render_line(&mut self, line: &Line2d, options: &RenderOptions);

    /// Renders a pointed oval,
    /// which is similar to an ellipse with 2 sharp corners at the ends of the longer dimension.
    fn render_pointed_oval(
        &mut self,
        center: &Point2d,
        radius_x: u32,
        radius_y: u32,
        options: &RenderOptions,
    );

    /// Renders a pointed oval in an [`axis aligned bounding box`](AABB).
    fn render_pointed_oval_aabb(&mut self, aabb: &AABB, options: &RenderOptions) {
        self.render_pointed_oval(
            &aabb.center(),
            aabb.size().width() / 2,
            aabb.size().height() / 2,
            options,
        )
    }

    /// Renders a polygon.
    fn render_polygon(&mut self, polygon: &Polygon2d, options: &RenderOptions);

    /// Renders a smooth polygon.
    fn render_smooth_polygon(&mut self, polygon: &Polygon2d, options: &RenderOptions);

    /// Renders an axis aligned rectangle.
    fn render_rectangle(&mut self, aabb: &AABB, options: &RenderOptions);
}
