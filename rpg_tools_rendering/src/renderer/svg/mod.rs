use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::math::size2d::Size2d;
use crate::renderer::color::WebColor;
use crate::renderer::svg::path::{
    path_from_circle_arc, path_from_line, path_from_polygon, path_from_polygon_with_hole,
    path_from_rounded_polygon,
};
use crate::renderer::text::TextOptions;
use crate::renderer::{RenderOptions, Renderer};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

pub mod path;

/// A valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct Svg {
    lines: Vec<String>,
}

impl Svg {
    /// Returns the svg as a single string.
    pub fn export(&self) -> String {
        self.lines.join("\n")
    }

    /// Saves the svg to a file.
    pub fn save(&self, path: &str) -> Result<()> {
        let mut output = File::create(path)?;

        for line in &self.lines {
            writeln!(&mut output, "{}", line)?;
        }

        Ok(())
    }
}

/// Builds a valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct SvgBuilder {
    lines: Vec<String>,
}

impl SvgBuilder {
    pub fn new(size: Size2d) -> Self {
        let mut lines = Vec::new();

        lines.push(format!(
            "<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
            size.width(),
            size.height()
        ));

        Self { lines }
    }

    pub fn finish(mut self) -> Svg {
        self.lines.push("</svg>".to_string());

        Svg { lines: self.lines }
    }

    fn render_path(&mut self, path: &str, options: &RenderOptions) {
        self.lines.push(format!(
            "  <path  d=\"{}\" style=\"{}\"/>",
            path,
            to_style(options),
        ));
    }
}

impl Renderer for SvgBuilder {
    fn render_circle(&mut self, center: &Point2d, radius: u32, options: &RenderOptions) {
        self.lines.push(format!(
            "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" style=\"{}\"/>",
            center.x,
            center.y,
            radius,
            to_style(options),
        ));
    }

    fn render_circle_arc(
        &mut self,
        center: &Point2d,
        radius: u32,
        offset: Orientation,
        angle: Orientation,
        options: &RenderOptions,
    ) {
        self.render_path(
            &path_from_circle_arc(center, radius, offset, angle),
            options,
        );
    }

    fn render_ellipse(
        &mut self,
        center: &Point2d,
        radius_x: u32,
        radius_y: u32,
        options: &RenderOptions,
    ) {
        self.lines.push(format!(
            "  <ellipse  cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" style=\"{}\"/>",
            center.x,
            center.y,
            radius_x,
            radius_y,
            to_style(options),
        ));
    }

    fn render_line(&mut self, line: &Line2d, options: &RenderOptions) {
        self.render_path(&path_from_line(line), options);
    }

    fn render_pointed_oval(
        &mut self,
        center: &Point2d,
        radius_x: u32,
        radius_y: u32,
        options: &RenderOptions,
    ) {
        let radius = (radius_x.pow(2) + radius_y.pow(2)) / (2 * radius_x.min(radius_y));
        let aabb = AABB::with_radii(*center, radius_x, radius_y);
        let (left, right) = if radius_x > radius_y {
            (aabb.get_point(0.0, 0.5), aabb.get_point(1.0, 0.5))
        } else {
            (aabb.get_point(0.5, 0.0), aabb.get_point(0.5, 1.0))
        };

        self.render_path(
            &format!(
                "M {} {} A {} {}, 0, 0, 0, {} {} A {} {}, 0, 0, 0, {} {} Z",
                left.x, left.y, radius, radius, right.x, right.y, radius, radius, left.x, left.y,
            ),
            options,
        );
    }

    fn render_polygon(&mut self, polygon: &Polygon2d, options: &RenderOptions) {
        self.render_path(&path_from_polygon(polygon), options);
    }

    fn render_polygon_with_hole(
        &mut self,
        polygon: &Polygon2d,
        hole: &Polygon2d,
        options: &RenderOptions,
    ) {
        self.render_path(&path_from_polygon_with_hole(polygon, hole), options);
    }

    fn render_rounded_polygon(&mut self, polygon: &Polygon2d, options: &RenderOptions) {
        self.render_path(&path_from_rounded_polygon(polygon), options);
    }

    fn render_rectangle(&mut self, aabb: &AABB, options: &RenderOptions) {
        self.lines.push(format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"{}\"/>",
            aabb.start().x,
            aabb.start().y,
            aabb.size().width(),
            aabb.size().height(),
            to_style(options),
        ));
    }

    fn render_text(
        &mut self,
        text: &str,
        center: &Point2d,
        orientation: Orientation,
        options: &TextOptions,
    ) {
        self.lines.push(format!(
            "  <text x=\"{}\" y=\"{}\" transform=\"rotate({},{},{})\" fill=\"{}\" font-size=\"{}px\" text-anchor=\"middle\">{}</text>",
            center.x, center.y, orientation.to_degree(),center.x, center.y,
            options.color, options.size, text
        ));
    }
}

fn to_style(options: &RenderOptions) -> String {
    format!(
        "{};{};stroke-width:{}",
        match &options.fill_color {
            None => "fill:none".to_string(),
            Some(color) => to_color(color, "fill"),
        },
        to_color(&options.line_color, "stroke"),
        options.line_width
    )
}

fn to_color(color: &WebColor, text: &str) -> String {
    match color {
        WebColor::Transparent(name, transparency) => {
            let opacity = 1.0 - (*transparency as f32 / 255.0);
            format!(
                "{0}:{1};{0}-opacity:{2}",
                text,
                name.to_lowercase(),
                opacity
            )
        }
        _ => format!("{}:{}", text, color.to_string().to_lowercase()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point2d::Point2d;
    use crate::renderer::color::WebColor;
    use rpg_tools_core::model::appearance::color::Color;

    #[test]
    fn test_empty_svg() {
        let builder = SvgBuilder::new(Size2d::new(100, 150));
        let svg = builder.finish();

        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">\n</svg>";

        assert_eq!(&svg.export(), result);
    }

    #[test]
    fn test_rectangles() {
        let options = RenderOptions::new(
            WebColor::from_color(Color::Blue),
            WebColor::from_color(Color::Red),
            5,
        );
        let aabb = AABB::new(Point2d::new(10, 20), Size2d::new(30, 40));
        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">
  <rect x=\"10\" y=\"20\" width=\"30\" height=\"40\" style=\"fill:blue;stroke:red;stroke-width:5\"/>
</svg>";

        let mut builder = SvgBuilder::new(Size2d::new(100, 150));
        builder.render_rectangle(&aabb, &options);
        let svg = builder.finish();

        assert_eq!(&svg.export(), result);
    }
}
