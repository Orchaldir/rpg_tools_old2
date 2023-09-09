use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;

pub mod builder;

/// Defines a polygons as a list of [`points`](Point2d).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Polygon2d {
    corners: Vec<Point2d>,
}

impl Polygon2d {
    /// Returns a new polygon.
    pub fn new(corners: Vec<Point2d>) -> Self {
        Self { corners }
    }

    /// Calculates the center of the polygon.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
    /// let polygon = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(100, 100),
    ///   Point2d::new(0, 100),
    /// ]);
    /// assert_eq!(polygon.calculate_center(), Point2d::new(50, 50));
    /// ```
    pub fn calculate_center(&self) -> Point2d {
        let mut sum = self.corners[0];

        for c in &self.corners[1..] {
            sum = sum + *c;
        }

        sum / self.corners.len() as f32
    }

    /// Returns the corners.
    pub fn corners(&self) -> &[Point2d] {
        &self.corners
    }

    /// Returns the corners.
    pub fn corners_mut(&mut self) -> &mut [Point2d] {
        &mut self.corners
    }

    /// Create a sharp corner for the corner cutting by duplicating it.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
    /// let mut polygon = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(50, 100),
    /// ]);
    /// let result = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(50, 100),
    /// ]);
    /// assert!(polygon.create_sharp_corner(1));
    /// assert_eq!(polygon, result);
    /// ```
    pub fn create_sharp_corner(&mut self, index: usize) -> bool {
        if let Some(corner) = self.corners.get(index) {
            self.corners.insert(index, *corner);

            return true;
        }

        false
    }

    /// Inserts the corners of another polygon at a certain index.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
    /// let a = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(0, 100),
    /// ]);
    /// let b = Polygon2d::new(vec![
    ///   Point2d::new(200, 0),
    ///   Point2d::new(200, 100),
    ///   Point2d::new(100, 100),
    /// ]);
    /// let result = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(200, 0),
    ///   Point2d::new(200, 100),
    ///   Point2d::new(100, 100),
    ///   Point2d::new(0, 100),
    /// ]);
    /// assert_eq!(a.insert(2, &b), result);
    /// ```
    pub fn insert(&self, index: usize, polygon: &Polygon2d) -> Polygon2d {
        let mut new_corners = self.corners.clone();

        for (i, corner) in polygon.corners.iter().enumerate() {
            new_corners.insert(index + i, *corner);
        }

        Polygon2d::new(new_corners)
    }

    /// Resizes the polygon around the center.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::polygon2d::Polygon2d;
    /// let polygon = Polygon2d::new(vec![
    ///   Point2d::new(0, 0),
    ///   Point2d::new(100, 0),
    ///   Point2d::new(100, 100),
    ///   Point2d::new(0, 100),
    /// ]);
    /// let result = Polygon2d::new(vec![
    ///   Point2d::new(-25, -25),
    ///   Point2d::new(125, -25),
    ///   Point2d::new(125, 125),
    ///   Point2d::new(-25, 125),
    /// ]);
    /// assert_eq!(polygon.resize(1.5), result);
    /// ```
    pub fn resize(&self, factor: f32) -> Polygon2d {
        let center = self.calculate_center();
        let corners = self
            .corners
            .iter()
            .map(|c| *c - center)
            .map(|c| c * factor)
            .map(|c| c + center)
            .collect();

        Polygon2d::new(corners)
    }
}

impl From<&AABB> for Polygon2d {
    fn from(aabb: &AABB) -> Self {
        Polygon2d::new(aabb.corners())
    }
}
