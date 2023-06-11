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
    ///# use rpg_tools_rendering::math::aabb2d::AABB;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let start = Point2d::new(2, 3);
    /// let size = Size2d::new(30, 50);
    /// let aabb = AABB::new(start, size);
    ///
    /// assert_eq!(aabb.start(), &start);
    /// assert_eq!(aabb.end(), &Point2d::new(32, 53));
    /// assert_eq!(aabb.size(), &size);
    /// ```
    pub fn new(start: Point2d, size: Size2d) -> Self {
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box initialized with primitives.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AABB;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// let aabb = AABB::simple(2, 3, 30, 50);
    ///
    /// assert_eq!(aabb.start(), &Point2d::new(2, 3));
    /// assert_eq!(aabb.end(), &Point2d::new(32, 53));
    /// assert_eq!(aabb.size(), &Size2d::new(30, 50));
    /// ```
    pub fn simple(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self::new(Point2d::new(x, y), Size2d::new(width, height))
    }

    /// Returns a new axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AABB;
    ///# use rpg_tools_rendering::math::size2d::Size2d;
    /// assert_eq!(AABB::with_size(Size2d::new(30, 50)), AABB::simple(0, 0, 30, 50));
    /// ```
    pub fn with_size(size: Size2d) -> Self {
        let start = Point2d::new(0, 0);
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    /// Returns a new axis aligned bounding box.
    pub fn with_center(center: Point2d, size: Size2d) -> Self {
        let start = Point2d::new(
            center.x - size.width() as i32 / 2,
            center.y - size.height() as i32 / 2,
        );
        let end = start + size;
        AxisAlignedBoundingBox { start, end, size }
    }

    pub fn start(&self) -> &Point2d {
        &self.start
    }

    /// Returns the center of the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(2, 3, 30, 50);
    ///
    /// assert_eq!(aabb.center(), Point2d::new(17, 28));
    /// ```
    pub fn center(&self) -> Point2d {
        self.start + self.size / 2.0
    }

    pub fn end(&self) -> &Point2d {
        &self.end
    }

    pub fn size(&self) -> &Size2d {
        &self.size
    }

    /// Returns the inner radius of the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AABB;
    /// assert_eq!(AABB::simple(2, 3, 30, 50).inner_radius(), 15);
    /// ```
    pub fn inner_radius(&self) -> u32 {
        self.size.width().min(self.size.height()) / 2
    }

    /// Is the [`Point2d`] inside?
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(10, 20, 30, 40);
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

    /// Shrinks the axis aligned bounding box by a certain amount?
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(10, 20, 130, 140).shrink(5);
    /// let desired = AxisAlignedBoundingBox::simple(15, 25, 120, 130);
    ///
    /// assert_eq!(aabb, desired);
    /// ```
    pub fn shrink(&self, border: u32) -> Self {
        Self::simple(
            self.start.x + border as i32,
            self.start.y + border as i32,
            self.size.width() - border * 2,
            self.size.height() - border * 2,
        )
    }

    /// Gets a [`point`](Point2d) inside the axis aligned bounding box.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::aabb2d::AxisAlignedBoundingBox;
    ///# use rpg_tools_rendering::math::point2d::Point2d;
    /// let aabb = AxisAlignedBoundingBox::simple(2, 3, 30, 60);
    ///
    /// assert_eq!(aabb.get_point(0.5, 0.25), Point2d::new(17, 18));
    /// ```
    pub fn get_point(&self, horizontal: f32, vertical: f32) -> Point2d {
        Point2d::new(
            self.start.x + (self.size.width() as f32 * horizontal) as i32,
            self.start.y + (self.size.height() as f32 * vertical) as i32,
        )
    }
}
