use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;

pub type AABB = AxisAlignedBoundingBox;

/// Defines a 2d axis aligned bounding box.
///
/// # Diagram
///
/// ```svgbob
///   +---------------------> x-axis
///   |     start
///   |     *---------*
///   |     |         |
///   |     |         |
///   |     |         |
///   |     *---------*
///   |           end = start + size
///   v
/// y-axis
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AxisAlignedBoundingBox {
    start: Point2d,
    end: Point2d,
    size: Size2d,
}

impl AxisAlignedBoundingBox {
    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let start = Point2d::new(2, 3);
    /// let size = Size2d::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert_eq!(aabb.start(), start);
    /// assert_eq!(aabb.end(), Point2d::new(32, 53));
    /// assert_eq!(aabb.size(), size);
    /// ```
    pub fn new(start: Point2d, size: Size2d) -> Self {
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let size = Size2d::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::with_size(size);
    ///
    /// assert_eq!(aabb.start(), Point2d::new(0, 0));
    /// assert_eq!(aabb.end(), Point2d::new(30, 50));
    /// assert_eq!(aabb.size(), size);
    /// ```
    pub fn with_size(size: Size2d) -> Self {
        let start = Point2d::new(0, 0);
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    pub fn start(&self) -> Point2d {
        self.start
    }

    /// Returns the center of the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let start = Point2d::new(2, 3);
    /// let size = Size2d::new(30, 50);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert_eq!(aabb.center(), Point2d::new(17, 28));
    /// ```
    pub fn center(&self) -> Point2d {
        self.start + self.size / 2.0
    }

    pub fn end(&self) -> Point2d {
        self.end
    }

    pub fn size(&self) -> Size2d {
        self.size
    }

    /// Is the [`Point2d`] inside?
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let start = Point2d::new(10, 20);
    /// let size = Size2d::new(30, 40);
    /// let aabb = AxisAlignedBoundingBox::new(start, size);
    ///
    /// assert!(aabb.is_inside(&Point2d::new(25, 40)));
    /// assert!(!aabb.is_inside(&Point2d::new(0, 0)));
    /// ```
    pub fn is_inside(&self, point: &Point2d) -> bool {
        point.x >= self.start.x
            && point.y >= self.start.y
            && point.x < self.end.x
            && point.y < self.end.y
    }
}
