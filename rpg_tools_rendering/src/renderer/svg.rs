use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::math::size2d::Size2d;
use crate::renderer::{RenderOptions, Renderer};
use anyhow::Result;
use std::fs::File;
use std::io::Write;

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
}

fn path_from_circle_arc(
    center: &Point2d,
    radius: u32,
    offset: Orientation,
    angle: Orientation,
) -> String {
    let start = center.calculate_polar(radius as f32, offset);
    let end = center.calculate_polar(radius as f32, offset + angle);

    format!(
        "M {} {} A {} {} 0 0 0 {} {} Z",
        start.x, start.y, radius, radius, end.x, end.y
    )
}

fn path_from_line(polygon: &Line2d) -> String {
    let mut path = String::new();
    let corners = polygon.corners();
    let first = &corners[0];
    path.push_str(format!("M {} {}", first.x, first.y).as_str());

    for point in corners.iter().skip(1) {
        path.push_str(format!(" L {} {}", point.x, point.y).as_str());
    }

    path
}

fn path_from_polygon(polygon: &Polygon2d) -> String {
    let mut path = String::new();
    let corners = polygon.corners();
    let first = &corners[0];
    path.push_str(format!("M {} {}", first.x, first.y).as_str());

    for point in corners.iter().skip(1) {
        path.push_str(format!(" L {} {}", point.x, point.y).as_str());
    }

    path.push_str(" Z");

    path
}

/// Renders a polygon with using quadratic Bézier curves of the SVG path.
/// Each curve goes from the midpoint of one polygon side to the midpoint of the next.
/// The corner between those polygon sides is the control point.
///
/// ```
///# use rpg_tools_rendering::math::point2d::Point2d;
///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
///# use rpg_tools_rendering::renderer::svg::path_from_rounded_polygon;
/// let polygon = Polygon2d::new(vec![
///   Point2d::new(0, 0),
///   Point2d::new(100, 0),
///   Point2d::new(0, 100),
/// ]);
/// assert_eq!(path_from_rounded_polygon(&polygon), "M 50 0 Q 100 0 50 50 Q 0 100 0 50 Q 0 0 50 0");
/// ```
///
/// If a point is twice in a row, its a sharp corner.
///
/// ```
///# use rpg_tools_rendering::math::point2d::Point2d;
///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
///# use rpg_tools_rendering::renderer::svg::path_from_rounded_polygon;
/// let polygon = Polygon2d::new(vec![
///   Point2d::new(0, 0),
///   Point2d::new(0, 0),
///   Point2d::new(100, 0),
///   Point2d::new(0, 100),
/// ]);
/// assert_eq!(path_from_rounded_polygon(&polygon), "M 0 0 L 50 0 Q 100 0 50 50 Q 0 100 0 50 Z");
/// ```
pub fn path_from_rounded_polygon(polygon: &Polygon2d) -> String {
    println!("path_from_smooth_polygon()");
    let mut path = String::new();
    let corners = create_corners(polygon);
    let mut previous = &corners[0];
    let mut is_start = true;
    let mut is_sharp = false;
    let mut index = 1;
    let mut first_middle = None;

    for point in corners.iter().skip(1) {
        if previous.calculate_distance(point) == 0.0 {
            is_sharp = true;

            println!("Point {} is same as previous", index);

            if !is_start {
                println!("Not start");
                path.push_str(format!(" L {} {}", previous.x, previous.y).as_str());
            }
            index += 1;

            continue;
        }

        if is_start {
            is_start = false;
            let middle = previous.lerp(point, 0.5);

            if is_sharp {
                println!("Point {} is sharp start", index);
                is_sharp = false;
                path.push_str(
                    format!(
                        "M {} {} L {} {}",
                        previous.x, previous.y, middle.x, middle.y
                    )
                    .as_str(),
                );
            } else {
                println!("Point {} is not sharp start", index);
                first_middle = Some(middle);
                path.push_str(format!("M {} {}", middle.x, middle.y).as_str());
            }
        } else if is_sharp {
            println!("Point {}: Line to middle after sharp", index);
            is_sharp = false;
            let middle = previous.lerp(point, 0.5);
            path.push_str(format!(" L {} {}", middle.x, middle.y).as_str());
        } else {
            println!("Point {}: curve", index);
            let middle = previous.lerp(point, 0.5);
            path.push_str(
                format!(" Q {} {} {} {}", previous.x, previous.y, middle.x, middle.y).as_str(),
            );
        }

        previous = point;
        index += 1;
    }

    if let Some(middle) = first_middle {
        path.push_str(
            format!(" Q {} {} {} {}", previous.x, previous.y, middle.x, middle.y).as_str(),
        );
    } else {
        path.push_str(" Z");
    }

    path
}

fn create_corners(polygon: &Polygon2d) -> Vec<Point2d> {
    let mut corners = Vec::from(polygon.corners());
    corners.push(corners[0]);

    corners
}

fn to_style(options: &RenderOptions) -> String {
    format!(
        "fill:{};stroke:{};stroke-width:{}",
        match &options.fill_color {
            None => "none".to_string(),
            Some(color) => color.to_string().to_lowercase(),
        },
        options.line_color.to_string().to_lowercase(),
        options.line_width
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point2d::Point2d;
    use crate::renderer::color::WebColor;
    use rpg_tools_core::model::color::Color;

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
