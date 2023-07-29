use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use anyhow::{bail, Result};
use std::ops::RangeInclusive;

const VALID_RANGE: RangeInclusive<f32> = 0.0..=1.0;

/// Defines a polygons as a list of [`points`](Point2d).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Polygon2d {
    corners: Vec<Point2d>,
}

impl Polygon2d {
    /// Returns a new point.
    pub fn new(corners: Vec<Point2d>) -> Polygon2d {
        Polygon2d { corners }
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

    /// Executes a simple corner cutting algorithm multiple times.
    pub fn cut_corners_n(&self, u: f32, v: f32, n: u32) -> Result<Polygon2d> {
        if n == 0 {
            bail!("Parameter n is 0!");
        } else if n == 1 {
            return self.cut_corners(u, v);
        }

        let mut polygon = self.cut_corners(u, v)?;

        for _i in 0..(n - 1) {
            polygon = polygon.cut_corners(u, v)?;
        }

        Ok(polygon)
    }

    /// Executes a simple corner cutting algorithm.
    pub fn cut_corners(&self, u: f32, v: f32) -> Result<Polygon2d> {
        if !VALID_RANGE.contains(&u) {
            bail!("Parameter u ({}) is invalid!", u);
        } else if !VALID_RANGE.contains(&v) {
            bail!("Parameter v ({}) is invalid!", v);
        } else if u + v > 1.0 {
            bail!("Parameters u & v are too big together!");
        }

        let n = self.corners.len();
        let mut start = self.corners.last().unwrap();
        let mut new_corners = Vec::new();

        for i in 0..n {
            let end = &self.corners[i];

            let new0 = start.lerp(end, u);
            let new1 = start.lerp(end, 1.0 - v);

            new_corners.push(new0);
            new_corners.push(new1);

            start = end;
        }

        Ok(Polygon2d::new(new_corners))
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
