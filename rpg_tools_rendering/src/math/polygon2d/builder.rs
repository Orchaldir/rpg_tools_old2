use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;

/// Defines a polygons as a list of [`points`](Point2d).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Polygon2dBuilder {
    left_corners: Vec<Point2d>,
    right_corners: Vec<Point2d>,
}

impl Polygon2dBuilder {
    /// Returns an empty polygon builder.
    pub fn new() -> Self {
        Self {
            left_corners: vec![],
            right_corners: vec![],
        }
    }

    /// Adds mirrored points of the [`aabb`](crate::math::aabb2d::AxisAlignedBoundingBox).
    pub fn add_mirrored_points(
        &mut self,
        aabb: &AABB,
        width: f32,
        vertical: f32,
        is_sharp: bool,
    ) -> (Point2d, Point2d) {
        let (left, right) = aabb.get_mirrored_points(width, vertical);

        self.add_points(left, right, is_sharp);

        (left, right)
    }

    /// Adds a point.
    pub fn add_point(&mut self, point: Point2d, is_sharp: bool) {
        self.left_corners.push(point);

        if is_sharp {
            self.left_corners.push(point);
        }
    }

    /// Adds 2 points.
    pub fn add_points(&mut self, left: Point2d, right: Point2d, is_sharp: bool) {
        self.left_corners.push(left);
        self.right_corners.push(right);

        if is_sharp {
            self.left_corners.push(left);
            self.right_corners.push(right);
        }
    }

    /// Build the polygon from the corners.
    pub fn build(mut self) -> Polygon2d {
        self.right_corners.reverse();
        self.left_corners.append(&mut self.right_corners);

        Polygon2d::new(self.left_corners)
    }
}
