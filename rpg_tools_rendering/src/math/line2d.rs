use crate::math::point2d::Point2d;

/// Defines a line as a list of [`points`](Point2d).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Line2d {
    corners: Vec<Point2d>,
}

impl Line2d {
    /// Returns a new point.
    pub fn new(corners: Vec<Point2d>) -> Line2d {
        Line2d { corners }
    }

    /// Returns the corners.
    pub fn corners(&self) -> &[Point2d] {
        &self.corners
    }
}

impl From<(Point2d, Point2d)> for Line2d {
    fn from(points: (Point2d, Point2d)) -> Self {
        Line2d::new(vec![points.0, points.1])
    }
}
