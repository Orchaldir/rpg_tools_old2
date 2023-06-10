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

    /// Returns the corners.
    pub fn corners(&self) -> &[Point2d] {
        &self.corners
    }

    /// Execute a simple corner cutting algorithm.
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
}
