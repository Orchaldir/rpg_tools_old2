use crate::math::line2d::Line2d;
use crate::math::orientation::Orientation;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;

pub fn path_from_circle_arc(
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

/// Returns the SVG path of a line.
///
/// ```
///# use rpg_tools_rendering::math::point2d::Point2d;
///# use rpg_tools_rendering::math::line2d::Line2d;
///# use rpg_tools_rendering::renderer::svg::path::path_from_line;
/// let polygon = Line2d::new(vec![
///   Point2d::new(0, 0),
///   Point2d::new(100, 0),
///   Point2d::new(0, 100),
/// ]);
/// assert_eq!(path_from_line(&polygon), "M 0 0 L 100 0 L 0 100");
/// ```
pub fn path_from_line(line: &Line2d) -> String {
    path_from_corners(line.corners())
}

/// Returns the SVG path of a polygon.
///
/// ```
///# use rpg_tools_rendering::math::point2d::Point2d;
///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
///# use rpg_tools_rendering::renderer::svg::path::path_from_polygon;
/// let polygon = Polygon2d::new(vec![
///   Point2d::new(0, 0),
///   Point2d::new(100, 0),
///   Point2d::new(0, 100),
/// ]);
/// assert_eq!(path_from_polygon(&polygon), "M 0 0 L 100 0 L 0 100 Z");
/// ```
pub fn path_from_polygon(polygon: &Polygon2d) -> String {
    let mut path = path_from_corners(polygon.corners());

    close(&mut path);

    path
}

fn path_from_corners(corners: &[Point2d]) -> String {
    let mut path = String::new();
    let first = &corners[0];

    move_to(&mut path, first);

    for point in corners.iter().skip(1) {
        line_to(&mut path, point);
    }

    path
}

/// Returns the SVG path of a polygon using quadratic Bézier curves.
/// Each curve goes from the midpoint of one polygon side to the midpoint of the next.
/// The corner between those polygon sides is the control point.
///
/// ```
///# use rpg_tools_rendering::math::point2d::Point2d;
///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
///# use rpg_tools_rendering::renderer::svg::path::path_from_rounded_polygon;
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
///# use rpg_tools_rendering::renderer::svg::path::path_from_rounded_polygon;
/// let polygon = Polygon2d::new(vec![
///   Point2d::new(0, 0),
///   Point2d::new(0, 0),
///   Point2d::new(100, 0),
///   Point2d::new(0, 100),
/// ]);
/// assert_eq!(path_from_rounded_polygon(&polygon), "M 0 0 L 50 0 Q 100 0 50 50 Q 0 100 0 50 Z");
/// ```
pub fn path_from_rounded_polygon(polygon: &Polygon2d) -> String {
    let mut path = String::new();
    let corners = create_corners(polygon);
    let mut previous = &corners[0];
    let mut is_start = true;
    let mut is_sharp = false;
    let mut first_middle = None;

    for point in corners.iter().skip(1) {
        if previous.calculate_distance(point) == 0.0 {
            is_sharp = true;

            if !is_start {
                line_to(&mut path, previous);
            }

            continue;
        }

        if is_start {
            is_start = false;
            let middle = previous.lerp(point, 0.5);

            if is_sharp {
                is_sharp = false;
                move_to(&mut path, previous);
                line_to(&mut path, &middle);
            } else {
                first_middle = Some(middle);
                move_to(&mut path, &middle);
            }
        } else if is_sharp {
            is_sharp = false;
            let middle = previous.lerp(point, 0.5);
            line_to(&mut path, &middle);
        } else {
            let middle = previous.lerp(point, 0.5);
            curve_to(&mut path, previous, &middle);
        }

        previous = point;
    }

    if let Some(middle) = first_middle {
        curve_to(&mut path, previous, &middle);
    } else {
        close(&mut path);
    }

    path
}

fn move_to(path: &mut String, point: &Point2d) {
    path.push_str(format!("M {} {}", point.x, point.y).as_str());
}

fn line_to(path: &mut String, point: &Point2d) {
    path.push_str(format!(" L {} {}", point.x, point.y).as_str());
}

fn curve_to(path: &mut String, control: &Point2d, end: &Point2d) {
    path.push_str(format!(" Q {} {} {} {}", control.x, control.y, end.x, end.y).as_str());
}

fn close(path: &mut String) {
    path.push_str(" Z");
}

fn create_corners(polygon: &Polygon2d) -> Vec<Point2d> {
    let mut corners = Vec::from(polygon.corners());
    corners.push(corners[0]);

    corners
}
